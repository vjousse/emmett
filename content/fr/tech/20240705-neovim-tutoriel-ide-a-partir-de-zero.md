---
title: "Configurer Neovim comme IDE/éditeur de code à partir de zéro"
date: "2024-05-07 09:33:20+01:00"
slug: configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide
tags: neovim, tutorial, lua, vim
status: draft
---

Vous avez envie d'utiliser [_Neovim_](https://neovim.io/) mais ne savez pas par où commencer ? Vous voulez comprendre ce que vous faites au lieu d'utiliser des configurations déjà toutes prêtes ? Vous n'avez aucune idée de comment faire du _Lua_ ou ne savez même pas pourquoi vous devriez ? Cet article est fait pour vous !

<!-- TEASER_END -->

> Cet article a pour unique but de vous apprendre à configurer **_Neovim_**. Si vous voulez apprendre à l'utiliser efficacement pour coder/éditer du texte, « [Vim pour les humains](https://vimebook.com/fr) » sera plus adapté pour vous.

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

![Capture d'écran montrant mon Neovim configuré comme un IDE](/images/configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide/my-neovim.png "Capture d'écran montrant mon Neovim configuré comme un IDE")

## Préambule sur Lua

_Neovim_ sans [Lua](https://www.lua.org/) c'est comme Milan sans Rémo, ça n'a aucun sens (seuls les vieux auront [la référence](https://www.bide-et-musique.com/song/149.html), les autres vous pouvez continuer de lire en ignorant cette disgression 🤓).

Nous allons donc configurer notre _Neovim_ entièrement et uniquement en [Lua](https://www.lua.org/), fini le _Vimscript_. Mais rassurez-vous, vous n'aurez besoin d'aucune connaissance particulière en _Lua_. Moi-même, je ne connais que très peu _Lua_ et je ne le pratique que dans le cadre de ma configuration _Vim_.

## Pré-requis

### Un terminal moderne

Je vous conseille vivement d'utiliser [Wez's Terminal Emulator](https://wezfurlong.org/wezterm/index.html). C'est le terminal que j'utilise tous les jours pour ces principales raisons : il supporte les ligatures (vous savez les jolies -> et autres symboles de programmation qu'on voit sur la capture d'écran), il peut afficher des images dans le terminal, il est hyper rapide (écrit en Rust) et très bien documenté. Je l'utilise pour ma part avec le thème [Tokyo Night](https://wezfurlong.org/wezterm/colorschemes/t/index.html#tokyo-night). Ma [configuration est disponible sur Github](https://github.com/vjousse/dotfiles/blob/master/wezterm/wezterm.lua).

D'autres bonnes alternatives sont [Alacritty](https://alacritty.org/), [Kitty](https://sw.kovidgoyal.net/kitty/) ou encore [foot](https://codeberg.org/dnkl/foot).

### Une police _Nerd font_

Pour pouvoir afficher tous les symboles dont notre configuration _Neovim_ va avoir besoin, vous devez installer une police [Nerd font](https://github.com/ryanoasis/nerd-fonts#tldr). Ce sont des polices de caractères modifiées pour y inclure les glyphes, les icones et les ligatures régulièrement utilisées en développement. Pour ma part j'utilise **FiraCode Nerd Font**.

### `ripgrep`

[`ripgrep`](https://github.com/BurntSushi/ripgrep) est une alternative à `grep` écrite en Rust. Il est sans commune mesure plus rapide que `grep` et c'est sûr lui qu'on va se baser pour la recherche dans _Neovim_.
