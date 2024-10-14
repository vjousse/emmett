---
title: Packaging a Tauri v2 app on Linux for flatpak/flathub and snapcraft with rust, npm and elm
slug: packaging-tauri-v2-flatpak-snapcraft-elm
date: "2024-10-18 09:36:00+00:00"
tags: flatpak, snapcraft, tauri, rust, elm, linux
category:
link:
description:
type: text
status: draft
toc: true
---

I'm developing a [Pomodoro technique](https://en.wikipedia.org/wiki/Pomodoro_Technique) app/tracker called [Pomodorolm](https://github.com/vjousse/pomodorolm) using [Rust](https://www.rust-lang.org/), [Tauri (v2)](https://v2.tauri.app/) and [Elm](https://elm-lang.org/). This app has a JS frontend (compiled from Elm), **plays sounds** bundled with the app using Rust, displays **desktop notifications** and provides a **tray icon updated dynamically** every second with a new icon generated using Rust.

A few weeks ago I decided to package it on Linux for [Flathub](https://flathub.org/apps/org.jousse.vincent.Pomodorolm) and [Snapcraft](https://snapcraft.io/pomodorolm) and it was quite a challenge. So in this post, I will sum-up how to overcome the difficulties I've encountered including:

- Going through the [**flathub review process**](https://github.com/flathub/flathub/pull/5562)
  - Enabling **offline builds** (even for Elm)
  - **Building the entire app from source** instead of using the method [documented on the official Tauri website](https://v2.tauri.app/fr/distribute/flatpak/) using the `.deb` package
  - Enforcing **`/app` prefix** instead of `/usr`
  - Providing a valid `metainfo.xml` file
- Managing **dynamic tray icon** generation support
- **Playing sound** for snapcraft with **alsa** and **pulseaudio**
- Handling **version numbers** all over the place

<!-- TEASER_END -->

## TL;DR

You can find the **flatpak build files** on the [Flathub repository of Pomodorolm](https://github.com/flathub/org.jousse.vincent.Pomodorolm), and the **`snapcraft.yml` file** on the [Github of Pomodorolm](https://github.com/vjousse/pomodorolm/blob/main/snapcraft.yaml) (with the required [`asound.conf` file](https://github.com/vjousse/pomodorolm/tree/main/snapcraft)).

Rust code to manage the `/app` prefix for `Flatpak`: @TODO

Rust code to make tray icons work: @TODO

## `Flatpak` and `Flathub`

First, be sure to install the [prerequisites](https://v2.tauri.app/distribute/flatpak/#prerequisites) as documented on the Tauri website to get `flatpak` and `flatpak-builder` locally.

### Base configuration file

Then, you will need to create a `yml` config file named after your app id, so for my app it's `org.jousse.vincent.Pomodorolm.yml`. If you don't plan to release your app on `flathub`, you should be able to use whatever you want for your app id. If you plan to release your app on `flathub`, be sure to read the note below.

> [!NOTE]
> If you intend to **release your app on Flathub**, you will need to use the reverse notation of a domain related to you, or related to the app, for the name of your `yaml` file (and for the `id` field of your app in the `yaml`). This domain will be checked during the review process on `flathub`. You can use a domain that you own or some Github, Codeberg, Gitlab, SourceHut, _whatever domain_, have a look at the [flathub documentation](https://docs.flathub.org/docs/for-app-authors/requirements/#control-over-domain-or-repository) for more details.
>
> My domain being `vincent.jousse.org`, my app id is naturally `org.jousse.vincent` + `Pomodorolm`, the name of the app.

Let's setup the first part of the file that will contain the required SDK and the options needed for your `flatpak` to work properly.

**`org.jousse.vincent.Pomodorolm.yml`**

```yaml
id: org.jousse.vincent.Pomodorolm

runtime: org.gnome.Platform
runtime-version: "46"
sdk: org.gnome.Sdk
sdk-extensions:
  - org.freedesktop.Sdk.Extension.rust-stable
  - org.freedesktop.Sdk.Extension.node20

command: pomodorolm # The name of the executable of your app that flatpak will run
finish-args:
  - --socket=wayland # Permission needed to show the window on wayland
  - --socket=fallback-x11 # Permission needed to fallback using X11 if wayland is not available
  - --socket=pulseaudio # We want to be able to play sounds
  - --device=dri # OpenGL
  - --share=ipc # Needed for performance reasons if the app fallbacks to X11
  - --env=FLATPAK=1 # Used by my custom Rust code to tell the app to look for assets in /app/lib/pomodorolm
  - --talk-name=org.freedesktop.Notifications # Needed to publish messages on dbus to trigger desktop notifications
  - --talk-name=org.kde.StatusNotifierWatcher # Needed to publish messages on dbus to display and change system tray icon

build-options:
  append-path: /usr/lib/sdk/node20/bin:/usr/lib/sdk/rust-stable/bin
```

Once the basic things have been setup, we will now configure our `flatpak` to be able to build our app without internet access during the build process. To do so, we will need to provide all the necessaries dependencies to `flatpak-builder` before the compilation process. `flatpak-builder` will download the dependencies locally and will then be able to start the build process in offline mode.

Below is the rest of the manifest that I will explain step by step.

**`org.jousse.vincent.Pomodorolm.yml`**

```yaml
# … start of the file explained above

modules:
  - shared-modules/libappindicator/libappindicator-gtk3-12.10.json
  - name: pomodorolm
    buildsystem: simple
    sources:
      - type: git
        url: https://github.com/vjousse/pomodorolm.git
        tag: app-v0.2.1
        commit: 3d4df941552d79d2030abcebfd301d1851d1c13c

      # flatpak-node-generator npm -o flatpak/node-sources.json package-lock.json
      - node-sources.json

      # flatpak-cargo-generator.py -d ./src-tauri/Cargo.lock -o flatpak/cargo-sources.json
      - cargo-sources.json

      # flatpak-elm-generator.py ../elm.json elm-sources.json
      - elm-sources.json

      - type: file
        url: https://raw.githubusercontent.com/robx/shelm/4417730f5847e6ccba1b19f1b25166471433d633/shelm
        sha256: a8b05b32495515f43fa24b2cec4dbb1fc141daf2d30eccb65df01b1a8ba31a5d

      - type: patch
        path: shelm-gnu-find-compat.patch

    build-options:
      env:
        CARGO_HOME: /run/build/pomodorolm/cargo
        XDG_CACHE_HOME: /run/build/pomodorolm/flatpak-node/cache
        npm_config_cache: /run/build/pomodorolm/flatpak-node/npm-cache
        npm_config_offline: "true"
        # Required so that packages are sorted in the C order (capital letters first)
        # by shelm when generating registry.dat
        LC_ALL: C.utf8
        ELM_HOME: elm-stuff/home/.elm
    build-commands:
      # Node packages
      - npm ci --offline --legacy-peer-deps
      # Generate elm packages index file (registry.dat)
      # Patched version of https://github.com/robx/shelm to make it work
      # with gnu find
      - bash ./shelm generate
      # Rust packages
      - cargo --offline fetch --manifest-path src-tauri/Cargo.toml --verbose
      # Compile the app without creating any bundle
      - npm run --offline tauri build -- --no-bundle

      - install -Dm644 -t /app/share/metainfo/ org.jousse.vincent.Pomodorolm.metainfo.xml
      - install -Dm644 -t /app/share/applications/ org.jousse.vincent.Pomodorolm.desktop
      - install -Dm755 -t /app/bin/ src-tauri/target/release/pomodorolm

      # Install icons
      - install -Dm644 src-tauri/icons/32x32.png /app/share/icons/hicolor/32x32/apps/org.jousse.vincent.Pomodorolm.png
      - install -Dm644 src-tauri/icons/128x128.png /app/share/icons/hicolor/128x128/apps/org.jousse.vincent.Pomodorolm.png
      - install -Dm644 src-tauri/icons/128x128@2x.png /app/share/icons/hicolor/256x256/apps/org.jousse.vincent.Pomodorolm.png
      - install -Dm644 src-tauri/icons/icon.png /app/share/icons/hicolor/256x256@2/apps/org.jousse.vincent.Pomodorolm.png

      # Copy the assets
      - mkdir -p /app/lib/pomodorolm
      - cp -rf src-tauri/themes /app/lib/pomodorolm
      - cp -rf src-tauri/audio /app/lib/pomodorolm

    post-install:
      - install -Dm644 LICENSE /app/share/licenses/org.jousse.vincent.Pomodorolm/LICENSE
```

### Build `libappindicator`

<!-- @FIX: give link to the submodule -->

We start by adding a line that will build `libappindicator` required for the system tray. The recommended way to get this file and its dependencies is by adding X as a submodule of your repository like described [here](@TODO).

```yaml
- shared-modules/libappindicator/libappindicator-gtk3-12.10.json
```

### Choose the `buildsystem`

Then we give a name to our build step (`pomodorolm` in my case) and tell `flatpak` to use the `simple` buildsystem to execute a series of shell commands.

```yaml
- name: pomodorolm
  buildsystem: simple
```

### Get the source code to compile

```yaml
sources:
  - type: git
    url: https://github.com/vjousse/pomodorolm.git
    tag: app-v0.2.1
    commit: 3d4df941552d79d2030abcebfd301d1851d1c13c
```

Here I'm building my app from a Github tag and commit. You can of course use other sources like [documented here](@TODO). If you want to build your app using local files, you can replace the lines above with:

```yaml
sources:
  - type: dir
    path: /path/to/your/files
```

### Provide offline dependencies for `npm` and `Cargo`

```yaml
# flatpak-node-generator npm -o flatpak/node-sources.json package-lock.json
- node-sources.json

# flatpak-cargo-generator.py -d ./src-tauri/Cargo.lock -o flatpak/cargo-sources.json
- cargo-sources.json
```

Those files are generated using [flatpak-builder-tools](@TODO) a set of Python scripts that will parse your `package-lock.json` and your `Cargo.lock` files to generate the required `*-sources.json` files. So you will first need to install the `flatpak-builder-tools`.

<!-- @FIX: give details on how to setup the python venvs -->

<!-- @FIX: link to the issue -->

> [!CAUTION]
> For `flatpak-node-generator` to run properly, you will need to call from a directory where there is no `node_modules/` present. You can either run it from outside your project or you can delete the `node_modules/` directory from your project directory first. See this issue for more details.

### Provide offline dependencies for `Elm`

If you don't use Elm in your project, you can skip this part entirely.

```yaml
# flatpak-elm-generator.py ../elm.json elm-sources.json
- elm-sources.json
```

Here I'm using a Python script I made myself to generate the required `flatpak` source file directly from the `elm.json` file. But it's not enough for Elm to work properly offline, we will also need to generate the `registry.dat` binary file using a shell script called `shelm` before the build.

```yaml
- type: file
  url: https://raw.githubusercontent.com/robx/shelm/4417730f5847e6ccba1b19f1b25166471433d633/shelm
  sha256: a8b05b32495515f43fa24b2cec4dbb1fc141daf2d30eccb65df01b1a8ba31a5d

- type: patch
  path: shelm-gnu-find-compat.patch
```

Here we provide she source of the `shelm` script and a simple patch that will make it compatible with GNU find (I suppose it was developed by a Mac OS X user using the default BSD find command). The patch file is automatically applied by `flatpak-builder`.

**`shelm-gnu-find-compat.patch`**

```patch
diff --git i/shelm w/shelm
index 147c0ea..8fa98ed 100755
--- i/shelm
+++ w/shelm
@@ -122,7 +122,7 @@ fetch() {

 # List dependencies in the local package cache, in the form $author/$project/$version.
 list_dependencies() {
-	cd "$pkgdir" && find . -type d -depth 3 | sed 's|^./||'
+	cd "$pkgdir" && find . -mindepth 3 -maxdepth 3 -type d | sed 's|^./||'
 }

 # Prune dependencies from the local package cache that don't match the required
```

### Specify build options for the offline build

```yaml
build-options:
  env:
    CARGO_HOME: /run/build/pomodorolm/cargo
    XDG_CACHE_HOME: /run/build/pomodorolm/flatpak-node/cache
    npm_config_cache: /run/build/pomodorolm/flatpak-node/npm-cache
    npm_config_offline: "true"
    # Required so that packages are sorted in the C order (capital letters first)
    # by shelm when generating registry.dat
    LC_ALL: C.utf8
    ELM_HOME: elm-stuff/home/.elm
```

Here we setup some `Cargo`, `npm` and `Elm` environment variables for the offline build to work properly.

> [!TIP]
> The paths that start with `/run/build/pomodorolm/` are built using the build name defined by `- name: pomodorolm`, so be sure to adapt the paths using your build name `/run/build/<name>/`.

> [!NOTE]
> For the **Elm users** out there, be sure to enforce the `LC_ALL` variable to `C.utf8` or `shelm` will nod be able to generate the `registry.dat` as it should.

### Build the project

```yaml
build-commands:
  # Node packages
  - npm ci --offline --legacy-peer-deps
  # Generate elm packages index file (registry.dat)
  # Patched version of https://github.com/robx/shelm to make it work
  # with gnu find
  - bash ./shelm generate
  # Rust packages
  - cargo --offline fetch --manifest-path src-tauri/Cargo.toml --verbose
  # Compile the app without creating any bundle
  - npm run --offline tauri build -- --no-bundle
```

Nothing fancy here, we just compile the project in offline mode without generating any bundle for Tauri as we will copy the files ourself.

### Install and copy the static files

Contrary to the `deb` package that installs the app in `/usr`, `flatpak` requires the app to be installed in a `/app` directory.

```yaml
# Copy the metainfo.xml that you have to write yourself
- install -Dm644 -t /app/share/metainfo/ org.jousse.vincent.Pomodorolm.metainfo.xml
# Same for the desktop file used by desktop environments to launch your app
- install -Dm644 -t /app/share/applications/ org.jousse.vincent.Pomodorolm.desktop
# Finally copy the generated binary
- install -Dm755 -t /app/bin/ src-tauri/target/release/pomodorolm

# Install icons
- install -Dm644 src-tauri/icons/32x32.png /app/share/icons/hicolor/32x32/apps/org.jousse.vincent.Pomodorolm.png
- install -Dm644 src-tauri/icons/128x128.png /app/share/icons/hicolor/128x128/apps/org.jousse.vincent.Pomodorolm.png
- install -Dm644 src-tauri/icons/128x128@2x.png /app/share/icons/hicolor/256x256/apps/org.jousse.vincent.Pomodorolm.png
- install -Dm644 src-tauri/icons/icon.png /app/share/icons/hicolor/256x256@2/apps/org.jousse.vincent.Pomodorolm.png

# Copy the assets of my app, json theme files and mp33 files
- mkdir -p /app/lib/pomodorolm
- cp -rf src-tauri/themes /app/lib/pomodorolm
- cp -rf src-tauri/audio /app/lib/pomodorolm
```

### Enforce `/app` prefix on the Rust side for resources

In the `finish-args` section of the manifest file, we've provided the following flag : `--env=FLATPAK=1`. It tells `flatpak` to set the `FLATPAK` environment variable to `1` when running the app. I'm using this environment variable to override the default resource resolver of Tauri on Linux as it resolves the assets to `/usr` by default.

```rust
fn resolve_resource_path(
    app_handle: &AppHandle,
    path_to_resolve: String
) -> Result<PathBuf, tauri::Error> {
    let mut resolved_path = app_handle
        .path()
        .resolve(path_to_resolve.clone(), BaseDirectory::Resource);

    #[cfg(target_os = "linux")]
    {
        let flatpak = std::env::var_os("FLATPAK");

        if flatpak.is_some() {
            let package_info = app_handle.package_info();

            resolved_path = Ok(PathBuf::from(format!(
                "/app/lib/{}/{}",
                package_info.crate_name, path_to_resolve
            )));
        }
    }

    resolved_path
}
```

I use this modified resolver when I need to get the path of a resource file like that:

```rust
let resource_path = resolve_resource_path(
    app.handle(),
    format!("audio/{}", sound_file)
);
```

<!-- @FIX: give link to the rust source code of the snippets above -->

### Build your `flatpak` with `flatpak-builder`

<!-- @TODO: write this section -->

### Create `metainfo.xml`

Use `appstreamcli validate your.metainfo.xml` to check for errors.

## Manage tray icon generation for `flatpak` and `Snapcraft`

Here is the problem: by default, Tauri will store the tray icon image into `/tmp` in the sandboxed `flatpak`/`Snapcraft` environment and will tell `libappindicator` that the icon is in `/tmp`. The Dbus message that `libappindicator` emits will be received by your Desktop environment and it will look for the icon in `/tmp` on the host. But of course, there is no icon in `/tmp` on the host as it is in `/tmp` but in the **sandboxed `flatpak`/`Snapcraft` environment**.

Once you've understood the problem, the solution is pretty straightforward: store the icon in a path shared by the sandboxed environment and the host.

<!-- @FIX: give link to the rust source code -->

```rust
let icon_path = tauri::image::Image::from_path(icon_path_buf.clone()).ok();

// Don't let tauri choose where to store the temp icon path as it will by default store it to `/tmp`.
// Setting it manually allows the tray icon to work properly in sandboxes env like Flatpak
// where we can share XDG_DATA_HOME between the host and the sandboxed env
// libappindicator will add the full path of the icon to the dbus message when changing it,
// so the path needs to be the same between the host and the sandboxed env
let local_data_path = app_handle
    .path()
    .resolve("tray-icon", BaseDirectory::AppLocalData)
    .unwrap();

let _ = tray.set_temp_dir_path(Some(local_data_path));
let _ = tray.set_icon(icon_path);
```

So here, I'm storing the icon in `$XDG_DATA_HOME/tray-icon/` directory because **`$XDG_DATA_HOME` is shared between `flatpak` and the host**. For the record, as we can see in the Tauri source code, `AppLocalData` is resolved to `$XDG_DATA_HOME`.

<!-- @FIX: give link to the rust source code -->

```rust
/// Returns the path to the user's local data directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value from the following table, or a `None`.
///
/// |Platform | Value                                    | Example                                  |
/// | ------- | ---------------------------------------- | ---------------------------------------- |
/// | Linux   | `$XDG_DATA_HOME` or `$HOME`/.local/share | /home/alice/.local/share                 |
/// | macOS   | `$HOME`/Library/Application Support      | /Users/Alice/Library/Application Support |
/// | Windows | `{FOLDERID_LocalAppData}`                | C:\Users\Alice\AppData\Local             |
pub fn data_local_dir() -> Option<PathBuf> {
    sys::data_local_dir()
}
```

## `Snapcraft`

### Configure `ALSA`/`pulseaudio` to play audio files

<!-- @FIX: give link to the snapcraft file -->

I will not go in details through the whole `snapcraft.yml` file as it was pretty straightforward to make it work: no offline build required and we can directly use the `.deb` generated by Tauri. Just be sure to use `base: core24` instead of `core22` if you want `rodio` to build properly.

For the sound to work, you need to provide a configuration file for Alsa:

**`asound.conf`**

```conf
pcm.!default {
    type pulse
    fallback "sysdefault"
    hint {
        show on
        description "Default ALSA Output (currently PulseAudio Sound Server)"
    }
}
ctl.!default {
    type pulse
    fallback "sysdefault"
}
```

That you will need to copy at build time:

```yaml
cp snapcraft/asound.conf $SNAPCRAFT_PART_INSTALL/etc/
```

After that, everything should work as expected.

### Build your `snap`

## Cherry on the cake: checking version numbers

<!-- @TODO: write the section -->

## Troubleshooting

### `Snapcraft` build error with Webkit: `Unable to spawn a new child process:…`

No worries, this error only happens for developers when building the app locally, but it's pretty annoying.

    ** (pomodorolm:364934): ERROR **: 11:34:14.245: Unable to spawn a new child process: Failed to spawn child process “/usr/lib/x86_64-linux-gnu/webkit2gtk-4.1/WebKitNetworkProcess” (No such file or directory)

If it happens, just run the following command to fix it:

    sudo /usr/lib/snapd/snap-discard-ns pomodorolm

### Nvidia and webkit error: `Error 71 (Protocol error) dispatching to Wayland display.`

If you run into this error or `Failed to create GBM buffer of size…`, it is likely because you're using Nvidia drivers under Linux. They are several bug reports in Webkit, cf this issue for wails: [https://github.com/wailsapp/wails/issues/2977#issuecomment-1791041741](https://github.com/wailsapp/wails/issues/2977#issuecomment-1791041741).

To fix it, you need to set the `WEBKIT_DISABLE_DMABUF_RENDERER` environment to `1`.

For example, for `flatpak`:

    flatpak override --user --env=WEBKIT_DISABLE_DMABUF_RENDERER=1 org.jousse.vincent.Pomodorolm

## Conclusion

Snapcraft way simpler but better review and selection process for on Flathub.
