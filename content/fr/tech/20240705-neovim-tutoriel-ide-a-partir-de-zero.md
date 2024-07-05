---
title: "Configurer Neovim comme IDE/éditeur de code à partir de zéro"
date: "2024-05-07 09:33:20+01:00"
slug: configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide
tags: neovim, tutorial, lua, vim
draft: true
---

Vous avez envie d'utiliser [_Neovim_](https://neovim.io/) mais ne savez pas par où commencer ? Vous voulez comprendre ce que vous faites au lieu d'utiliser des configurations déjà toutes prêtes ? Vous n'avez aucune idée de comment faire du _Lua_ ou ne savez même pas pourquoi vous devriez ? Cet article est fait pour vous !

<!-- TEASER_END -->

À la fin de cet article, vous devriez avoir un _Neovim_ entièrement utilisable comme IDE pour coder tout ce que vous voulez avec les fonctionnalités suivantes :

- Complétion automatique de code
- Formatage à la sauvegarde
- Intégration de Git
- Explorateur de fichier
- Recherche survitaminée
- Coloration des parenthèses ouvrantes/fermantes
- Indicateurs visuels d'indentation
- Indicateurs des `@FIXME` `@TODO` etc dans le code
- Et tout plein de trucs que j'oublie certainement

Voilà ce à quoi vous devriez à peu près arriver :

![Capture d'écran montrant mon Neovim configuré comme un IDE](/images/configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide/my-neovim.png "Capture d'écran montrant mon Neovim configuré comme un IDE").
