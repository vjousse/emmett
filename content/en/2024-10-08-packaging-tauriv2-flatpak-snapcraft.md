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
---

I’m developing a [Pomodoro technique](https://en.wikipedia.org/wiki/Pomodoro_Technique) app/tracker called [Pomodorolm](https://github.com/vjousse/pomodorolm) using [Rust](https://www.rust-lang.org/), [Tauri (v2)](https://v2.tauri.app/) and [Elm](https://elm-lang.org/). A few weeks ago I decided to package it on Linux for [Flathub](https://flathub.org/apps/org.jousse.vincent.Pomodorolm) and [Snapcraft](https://snapcraft.io/pomodorolm) and it was quite a challenge. So in this post, I will sum-up how to overcome the difficulties I've encountered including:

- Going through the [**flathub review process**](https://github.com/flathub/flathub/pull/5562)
  - Enabling **offline builds** (even for Elm)
  - **Building the entire app from source** instead of using the method [documented on the official Tauri website](https://v2.tauri.app/fr/distribute/flatpak/) using the `.deb` package
  - Enforcing **`/app` prefix** instead of `/usr`
  - Providing a valid `metainfo.xml` file
- Managing **dynamic tray icon** generation support
- Displaying **notifications with dynamic icons**
- Embedding **app/Tauri resources** (like **audio** and **json** files)
- **Playing sound** for snapcraft with **alsa** and **pulseaudio**
- Handling **version numbers** all over the place

<!-- TEASER_END -->

## TL;DR

You can find the **flatpak build files** on the [Flathub repository of Pomodorolm](https://github.com/flathub/org.jousse.vincent.Pomodorolm), and the **`snapcraft.yml` file** on the [Github of Pomodorolm](https://github.com/vjousse/pomodorolm/blob/main/snapcraft.yaml) (with the required [`asound.conf` file](https://github.com/vjousse/pomodorolm/tree/main/snapcraft)).

## Flatpak and Flathub

First, be sure to install the [prerequisites](https://v2.tauri.app/distribute/flatpak/#prerequisites) as documented on the Tauri website to install `flatpak` and `flatpak-builder` locally.

### Base configuration file

Then, you will need to create a `yml` config file named after your app id, so for my app it’s `org.jousse.vincent.Pomodorolm`. If you don’t plan to release your app on `flathub`, you should be able to use whatever you want for your app id. If you plan to release your app on `flathub`, be sure to read the note below.

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

Once the basic things have been setup, we will now configure our `flatpak` to be able to build our app without internet access during the build process. To do so, we will need to provide all the necessaries dependencies to `flatpak-builder` before the compilation process. `flatpak-builder` will download the dependencies locally and will then be able to start the build process.

Below is the rest of the manifest that we will explain step by step.

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

### Step by step explanations

<!-- @FIX: give link to the submodule -->

We start by adding a line that will build `libappindicator` required for the system tray. The recommended way to get this file and its dependencies is by adding X as a submodule of your repository like described [here](@TODO).

```yaml
- shared-modules/libappindicator/libappindicator-gtk3-12.10.json
```

Then we give a name to our build step (`pomodorolm` in my case) and tell `flatpak` to use the `simple` buildsystem to execute a series of shell commands.

```yaml
- name: pomodorolm
  buildsystem: simple
```

It' time to provide the required sources for the build.

```yaml
sources:
  - type: git
    url: https://github.com/vjousse/pomodorolm.git
    tag: app-v0.2.1
    commit: 3d4df941552d79d2030abcebfd301d1851d1c13c
```

Here I'm building my app from a github tag and commit. You can of course use other sources like [documented here](@TODO). If you want to build your app using local files, you cat replace the lines above with:

```yaml
sources:
  - type: dir
    path: /path/to/your/files
```

Here comes the interesting part: providing the dependencies for `Cargo` and `npm`.

```yaml
# flatpak-node-generator npm -o flatpak/node-sources.json package-lock.json
- node-sources.json

# flatpak-cargo-generator.py -d ./src-tauri/Cargo.lock -o flatpak/cargo-sources.json
- cargo-sources.json

# flatpak-elm-generator.py ../elm.json elm-sources.json
- elm-sources.json
```

Those files are generated using [flatpak-builder-tools](@TODO) a set of Python scripts that will parse your `package-lock.json` and your `Cargo.lock` files to generate the required `*-sources.json` files.
