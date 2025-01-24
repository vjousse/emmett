---
title: Packaging a Tauri v2 app on Linux for flatpak/flathub and Snapcraft with rust, npm and elm
slug: packaging-tauri-v2-flatpak-snapcraft-elm
date: "2024-10-22 09:36:00+00:00"
updated_at: "2025-01-24 15:00:00+01:00"
tags: flatpak, snapcraft, tauri, rust, elm, linux
category:
link:
description:
type: text
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

You can find the **flatpak build files** on the [Flathub repository of Pomodorolm](https://github.com/flathub/org.jousse.vincent.Pomodorolm/tree/2ab1d72c8a44325882374f0c85ee75fecdf9ed9a), and the **`snapcraft.yml` file** on the [Github of Pomodorolm](https://github.com/vjousse/pomodorolm/blob/9f4a7679ef81c3daa2f74c1ab97fd7ac720abfe4/snapcraft.yaml) (with the required [`asound.conf` file](https://github.com/vjousse/pomodorolm/blob/9f4a7679ef81c3daa2f74c1ab97fd7ac720abfe4/snapcraft/asound.conf)).

[Rust code](https://github.com/vjousse/pomodorolm/blob/9f4a7679ef81c3daa2f74c1ab97fd7ac720abfe4/src-tauri/src/lib.rs#L815) to manage the `/app` prefix for `Flatpak`.

[Rust code](https://github.com/vjousse/pomodorolm/blob/9f4a7679ef81c3daa2f74c1ab97fd7ac720abfe4/src-tauri/src/lib.rs#L531) to make tray icons work.

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

We start by adding a line that will build `libappindicator` required for the system tray. The recommended way to get this file and its dependencies is by adding [https://github.com/flathub/shared-modules](https://github.com/flathub/shared-modules) as a submodule of your repository like described [in the README](https://github.com/flathub/shared-modules?tab=readme-ov-file#adding).

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

Those files are generated using [flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools/) a set of Python scripts that will parse your `package-lock.json` and your `Cargo.lock` files to generate the required `*-sources.json` files. So you will first need to install the `flatpak-builder-tools`.

For `node` packages use [`flatpak-node-generator`](https://github.com/flatpak/flatpak-builder-tools/blob/master/node/README.md) and for `Cargo` packages use [`flatpak-cargo-generator`](https://github.com/flatpak/flatpak-builder-tools/tree/master/cargo).

> [!CAUTION]
> For `flatpak-node-generator` to run properly, you will need to call it from a directory where there is no `node_modules/` present. You can either run it from outside your project or you can delete the `node_modules/` directory from your project directory first. See this [issue for more details](https://github.com/flatpak/flatpak-builder-tools/issues/354#issuecomment-1478518442).

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

You can find the [complete source code on Github](https://github.com/vjousse/pomodorolm/blob/9f4a7679ef81c3daa2f74c1ab97fd7ac720abfe4/src-tauri/src/lib.rs#L815).

### Build your `flatpak` with `flatpak-builder`

#### Installation

Be sure to [install `flatpak` for your distribution](https://flatpak.org/setup/) and then install the `Gnome 46` platform and SDK:

    flatpak install flathub org.gnome.Platform//46 org.gnome.Sdk//46

#### Build

To build your application, use the following command with your manifest file instead of mine:

    flatpak-builder --force-clean --user --repo=repo --install builddir org.jousse.vincent.Pomodorolm.yml

To be sure that your are building your application entirely from scratch, you can delete the build dirs that `flatpak` creates:

    rm -rf builddir .flatpak-builder repo

#### Run

    flatpak run org.jousse.vincent.Pomodorolm

#### Lint

To check that your manifest file doesn't contain any errors:

    flatpak run --command=flatpak-builder-lint org.flatpak.Builder manifest org.jousse.vincent.Pomodorolm.yml

#### Debug

You can start a shell into your `flatpak` by issuing the following command:

    flatpak run --command=bash org.jousse.vincent.Pomodorolm

### Create `metainfo.xml`

The `metainfo.xml` file is used to descibre your application using the [AppStream specification](https://www.freedesktop.org/software/appstream/docs/).

"AppStream is a collaborative effort for enhancing the way we interact with the software repositories provided by the distribution by standardizing sets of additional metadata.

AppStream provides the foundation to build software-center applications. It additionally provides specifications for things like a unified software metadata database, screenshot services and various other things needed to create user-friendly application-centers for software distributions."

You can create your `metainfo.xml` file by [following the specification](https://www.freedesktop.org/software/appstream/docs/) or by using a [Metainfo Generator](https://bilelmoussaoui.github.io/metainfo-generator/).

You can then use `appstreamcli validate your.metainfo.xml` to check for errors.

## Manage tray icon generation for `flatpak` and `Snapcraft`

Here is the problem: by default, Tauri will store the tray icon image into `/tmp` in the sandboxed `flatpak`/`Snapcraft` environment and will tell `libappindicator` that the icon is in `/tmp`. The Dbus message that `libappindicator` emits will be received by your Desktop environment and it will look for the icon in `/tmp` on the host. But of course, there is no icon in `/tmp` on the host as it is in `/tmp` but in the **sandboxed `flatpak`/`Snapcraft` environment**.

Once you've understood the problem, the solution is pretty straightforward: store the icon in a path shared by the sandboxed environment and the host.

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

You can find the [complete source code on Github](https://github.com/vjousse/pomodorolm/blob/9f4a7679ef81c3daa2f74c1ab97fd7ac720abfe4/src-tauri/src/lib.rs#L531).

So here, I'm storing the icon in `$XDG_DATA_HOME/tray-icon/` directory because **`$XDG_DATA_HOME` is shared between `flatpak` and the host**. For the record, as we can see in the [Tauri source code](https://github.com/tauri-apps/tauri/blob/36eee37220cff34a1fab35791dcd0775ae86ec7c/crates/tauri/src/path/desktop.rs#L46), `AppLocalData` is resolved to `$XDG_DATA_HOME`.

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

### The `snapcraft.yaml` file

I will not go in details through the whole [`snapcraft.yml`](https://github.com/vjousse/pomodorolm/blob/9f4a7679ef81c3daa2f74c1ab97fd7ac720abfe4/snapcraft.yaml) file as it was pretty straightforward to make it work: no offline build required and we can directly use the `.deb` generated by Tauri. Just be sure to use `base: core24` instead of `core22` if you want `rodio` to build properly.

```yaml
name: pomodorolm
base: core24
platforms:
  amd64:
  arm64:

version: "0.3.4"
summary: A simple, good looking and multi-platform pomodoro tracker
description: |
  Pomodorolm is a simple and configurable Pomodoro timer. It aims to provide a visually-pleasing and reliable way to track productivity using the Pomodoro Technique.

grade: stable
confinement: strict

layout:
  /usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/webkit2gtk-4.1:
    bind: $SNAP/gnome-platform/usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/webkit2gtk-4.1
  /usr/lib/pomodorolm:
    symlink: $SNAP/usr/lib/pomodorolm
  /usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/alsa-lib:
    bind: $SNAP/usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/alsa-lib
  /usr/share/alsa:
    bind: $SNAP/usr/share/alsa

apps:
  pomodorolm:
    command: usr/bin/pomodorolm
    command-chain:
      - bin/gpu-2404-wrapper
      - snap/command-chain/alsa-launch
    desktop: usr/share/applications/pomodorolm.desktop
    extensions:
      - gnome
    environment:
      LD_LIBRARY_PATH: $LD_LIBRARY_PATH:$SNAP/usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/blas:$SNAP/usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/lapack:$SNAP/usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/samba:$SNAP/usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/vdpau:$SNAP/usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/dri
      ALWAYS_USE_PULSEAUDIO: "1"
    plugs:
      - home
      - browser-support
      - network
      - network-status
      - gsettings
      - desktop
      - opengl
      - alsa
      - audio-playback

package-repositories:
  - type: apt
    components: [main]
    suites: [noble]
    key-id: 78E1918602959B9C59103100F1831DDAFC42E99D
    url: http://ppa.launchpad.net/snappy-dev/snapcraft-daily/ubuntu

parts:
  build-app:
    plugin: dump
    after:
      - alsa-mixin
    build-snaps:
      - node/20/stable
      - rustup/latest/stable
    build-packages:
      - libwebkit2gtk-4.1-dev
      - build-essential
      - curl
      - wget
      - file
      - libxdo-dev
      - libssl-dev
      - libayatana-appindicator3-dev
      - librsvg2-dev
      - dpkg
      - libasound2-dev
    stage-packages:
      - libwebkit2gtk-4.1-0
      - libayatana-appindicator3-1

    source: .

    override-build: |
      set -eu
      npm install
      rustup default stable
      npm run tauri build -- --bundles deb
      dpkg -x src-tauri/target/release/bundle/deb/*.deb $SNAPCRAFT_PART_INSTALL/
      sed -i -e "s|Icon=pomodorolm|Icon=/usr/share/icons/hicolor/32x32/apps/pomodorolm.png|g" $SNAPCRAFT_PART_INSTALL/usr/share/applications/pomodorolm.desktop

  alsa-mixin:
    plugin: dump
    source: https://github.com/diddlesnaps/snapcraft-alsa.git
    source-subdir: snapcraft-assets
    build-packages:
      - libasound2-dev
    stage-packages:
      - libasound2-plugins
      - yad
    stage:
      - etc/asound.conf
      - snap/command-chain/alsa-launch
      - usr/bin/yad*
      - usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/alsa-lib
      - usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/libasound*
      - usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/libdnsfile*
      - usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/libFLAC*
      - usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/libjack*
      - usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/libpulse*
      - usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/libsamplerate*
      - usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/libspeex*
      - usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/libvorbis*
      - usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/pulseaudio

  gpu-2404:
    after:
      - build-app
    source: https://github.com/canonical/gpu-snap.git
    plugin: dump
    override-prime: |
      craftctl default
      ${CRAFT_PART_SRC}/bin/gpu-2404-cleanup mesa-2404 nvidia-2404
    prime:
      - bin/gpu-2404-wrapper
```

For the sound part, this build file was heavily inspired by the one of [bluebubbles-app](https://github.com/BlueBubblesApp/bluebubbles-app/blob/a744a27c9f067578549edab0a75920f99569e25b/snap/snapcraft.yaml).

### Build your `snap`

To build your `snap` just run `snapcraft` at the root of your project where the `snapcraft.yml` file should be located. If you want to activate debug and to automatically start a shell in the `snap` if the build fails, build the `snap` with the following command:

    snapcraft -v --debug

### Install your `snap`

    sudo snap install ./pomodorolm_0.3.0_amd64.snap --dangerous --devmode

### Run your `snap`

    snap run pomodorolm

The app name is provided in your `snapcraft.yml`:

```yaml
apps:
  pomodorolm:
```

## Cherry on the cake: checking version numbers

The more stuff you package, the more version numbers in files you have to manage.

Here is a list of the files in my project containing version numbers:

```python
METAINFO = "org.jousse.vincent.Pomodorolm.metainfo.xml"
PACKAGE_JSON = "package.json"
PACKAGE_LOCK_JSON = "package-lock.json"
SNAPCRAFT = "snapcraft.yaml"
TAURI_CONF = "src-tauri/tauri.conf.json"
CARGO_TOML = "src-tauri/Cargo.toml"
CARGO_LOCK = "src-tauri/Cargo.lock"
AUR_PKGBUILD = "aur/PKGBUILD"
```

I've written a [Python script](https://github.com/vjousse/pomodorolm/blob/98fe901b39ffc8981e37dcf90e62a731533875a9/bin/release.py) that can automatically bump the version number using [`git-cliff`](https://git-cliff.org/) and write it down to the corresponding files. It can also update the versions of your `metainfo.xml` file and check that your version numbers are consistent across all your files.

Don't hesitate to try it and provide feedback!

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

Packaging my app for Linux took me a lot longer than expected. The hardest part was to be able to **build it offline** for `Flatpak/Flathub`. The **review process** for `Flathub` was pretty serious too, two reviewers guided me through the process and asked for a lot of changes to improve the security of the app.

On the other hand, building for `Snapcraft` was easy. No need for offline builds and no review process. It compiles? Ship it to the store!

So as a user of Linux apps **I would recommend using `flatpaks` instead of `snaps`**. The **manual review process** of `flathub` implies that the apps have the minimum **quality** required by the `flathub` reviewers, and now that I know how it works, I have a lot more trust in `flatpaks` than in `snaps`.
