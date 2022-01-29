---
title: Manage multiple Crystal Lang versions
slug: manage-multiple-crystal-lang-versions
date: 2021-02-05 09:00:00+00:00
tags: crystal, beginner
category: 
link: 
description: 
type: text
---

As Crystal 0.36 is now out on [Crystal Lang](https://crystal-lang.org/) you may want to manage multiple versions of the crystal compiler locally. Let's see how to do that quickly and easily.

<!-- TEASER_END -->

## Asdf to the rescue

Whatever the language you are using, I highly recommend using [Asdf-vm](https://asdf-vm.com/) to manage your multiple versions. It works with [almost all the languages](https://asdf-vm.com/#/plugins-all) you can think of.

Follow the instructions to [install asdf](https://asdf-vm.com/#/core-manage-asdf?id=install) on their website and once it's done, install the versions of crystal you want to have locally, so for example, `0.35.1` and `0.36.1`:
```
asdf install crystal latest:0.35
asdf install crystal latest:0.36
```

You can then type `asdf list crystal` and you should see something like that:

```
➜  ~ asdf list crystal
  0.35.1
  0.36.1
```

## Set current crystal version

You can then set your crystal version:

- Globally by using `asdf global crystal 0.36.1`
- In the current shell session by using `asdf shell crystal 0.36.1`
- For the current directory by using `asdf local crystal 0.35.1`

So for example, my setup is this one actually : `0.36.1` globally, and `0.35.1` for my lucky projects because Lucky is not compatible with 0.36 for now.

Enjoy!
