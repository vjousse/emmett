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

I’m developing a [Pomodoro technique](https://en.wikipedia.org/wiki/Pomodoro_Technique) app/tracker called [Pomodorolm](https://github.com/vjousse/pomodorolm) using [Rust](https://www.rust-lang.org/), [Tauri (v2)](https://v2.tauri.app/) and [Elm](https://elm-lang.org/). And a few weeks ago I decided to package it on Linux for [Flathub](https://flathub.org/apps/org.jousse.vincent.Pomodorolm) and [Snapcraft](https://snapcraft.io/pomodorolm) and to be honest, it was quite a challenge. So, I will sum-up in this post how to overcome the difficulties I've encountered including:

- Going through the [**flathub review process**](https://github.com/flathub/flathub/pull/5562)
  - Enabling **offline builds** (even for Elm)
  - **Building the entire app from source** instead of using the method [documented on the official Tauri website](https://v2.tauri.app/fr/distribute/flatpak/) using the `.deb` package
  - Enforcing **`/app` prefix** instead of `/usr`
  - Providing a valid `metainfo.xml` file
- Enabling **dynamic tray icon** generation support
- Embedding **app/Tauri resources** (like **audio** and **json** files)
- Managing **playing sound for snapcraft** with **alsa** and **pulseaudio**
- Handling **version numbers** all over the place

<!-- TEASER_END -->

## TL;DR

You can find the **flatpak build files** on the [Flathub repository of Pomodorolm](https://github.com/flathub/org.jousse.vincent.Pomodorolm), and the **`snapcraft.yml` file** on the [Github of Pomodorolm](https://github.com/vjousse/pomodorolm/blob/main/snapcraft.yaml).

## Flatpak offline builds
