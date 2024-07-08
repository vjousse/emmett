---
title: "Configurer Neovim comme IDE/éditeur de code à partir de zéro"
date: "2024-05-07 09:33:20+01:00"
slug: configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide
tags: neovim, tutorial, lua, vim
status: draft
---

Vous avez envie d'utiliser [_Neovim_](https://neovim.io/) mais ne savez pas par où commencer ? Vous voulez comprendre ce que vous faites au lieu d'utiliser des configurations déjà toutes prêtes ? Vous n'avez aucune idée de comment faire du _Lua_ ou ne savez même pas pourquoi vous devriez ? Cet article est fait pour vous !

<!-- TEASER_END -->

> 📙 Cet article a pour unique but de vous apprendre à configurer **_Neovim_**. Si vous voulez apprendre à l'utiliser efficacement pour coder/éditer du texte, « [Vim pour les humains](https://vimebook.com/fr) » sera plus adapté pour vous.

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

## Préambule

_Neovim_ sans [Lua](https://www.lua.org/) c'est comme Milan sans Rémo, ça n'a aucun sens (seuls les vieux auront [la référence](https://www.bide-et-musique.com/song/149.html), les autres vous pouvez continuer de lire en ignorant cette disgression 🤓).

Nous allons donc configurer notre _Neovim_ entièrement et uniquement en [Lua](https://www.lua.org/), fini le _Vimscript_. Mais rassurez-vous, vous n'aurez besoin d'aucune connaissance particulière en _Lua_. Moi-même, je ne connais que très peu _Lua_ et je ne le pratique que dans le cadre de ma configuration _Vim_.

Le contenu de cet article devrait fonctionner aussi bien sous Mac OS X que sous Linux. Pour les utilisateurs Windows, j'imagine que ça peut aussi être le cas en utilisant WSL.

## Pré-requis

### Un terminal moderne

Je vous conseille vivement d'utiliser [Wez's Terminal Emulator](https://wezfurlong.org/wezterm/index.html). C'est le terminal que j'utilise tous les jours pour ces principales raisons : il supporte les ligatures (vous savez les jolies →, ⇒, ≠ et autres symboles de programmation qu'on voit sur la capture d'écran), il peut afficher des images dans le terminal, il est hyper rapide, écrit en Rust et très bien documenté. Je l'utilise pour ma part avec le thème [Tokyo Night](https://wezfurlong.org/wezterm/colorschemes/t/index.html#tokyo-night). Ma [configuration est disponible sur Github](https://github.com/vjousse/dotfiles/blob/master/wezterm/wezterm.lua).

D'autres bonnes alternatives sont [Alacritty](https://alacritty.org/), [Kitty](https://sw.kovidgoyal.net/kitty/) ou encore [foot](https://codeberg.org/dnkl/foot).

### Une police _Nerd font_

Pour pouvoir afficher tous les symboles dont notre configuration _Neovim_ va avoir besoin, vous devez installer une police [Nerd font](https://github.com/ryanoasis/nerd-fonts#tldr). Ce sont des polices de caractères modifiées pour y inclure les glyphes, les icones et les ligatures régulièrement utilisées en développement. Pour ma part j'utilise **FiraCode Nerd Font**.

### `ripgrep`

[`ripgrep`](https://github.com/BurntSushi/ripgrep) est une alternative à `grep` écrite en Rust. Il est sans commune mesure plus rapide que `grep` et c'est sûr lui qu'on va se baser pour la recherche dans _Neovim_.

## Structure initiale des fichiers

Nous allons commencer par créer les fichiers et les répertoires nécessaires à notre configuration.

```bash
mkdir -p ~/.config/nvim
```

L'option `-p` permet de dire à `mkdir` de créer toute l'arborescence de fichiers si elle n'existe pas déjà.

Nous allons ensuite créer le point d'entrée de notre configuration, à savoir `init.lua`.

```bash
cd ~/.config/nvim
touch init.lua
```

`touch` permet de créer un fichier vide s'il n'existe pas (et aussi de mettre le timestamp de modification du fichier à l'heure actuelle s'il existe déjà).

Maintenant, créons le répertoire où nous allons mettre la configuration des raccourcis clavier et des options de _Neovim_.

```bash
mkdir -p lua/core
```

Puis, créons le répertoire où nous allons configurer nos plugins.

```bash
mkdir -p lua/plugins
```

Voilà à quoi devrait ressembler votre arborescence pour l'instant :

```
~/.config/nvim
├── init.lua
└── lua
    ├── core
    └── plugins
```

> ℹ️ À noter que cette arborescence est totalement arbitraire et est issue de mes préférences personnelles. Libre à vous de ranger les choses différemment une fois que vous aurez compris comment tout cela fonctionne.

## Options par défaut

Éditons maintenant les options par défaut de notre _Neovim_. Placez vous dans `~/.config/nvim` et éditez/créez le fichier `lua/core/options.lua` :

```bash
nvim lua/core/options.lua
```

Placez-y le contenu suivant :

**`lua/core/options.lua`**

```lua
local opt = vim.opt -- raccourci pour un peu plus de concision

-- numéros de ligne
opt.relativenumber = true -- affichage des numéros de ligne relatives à la position actuelle du curseur
opt.number = true -- affiche le numéro absolu de la ligne active lorsque que relativenumber est activé

-- tabs & indentation
opt.tabstop = 2 -- 2 espaces pour les tabulations
opt.shiftwidth = 2 -- 2 espaces pour la taille des indentations
opt.expandtab = true -- change les tabulations en espaces (don't feed the troll please ;) )
opt.autoindent = true -- on garde l'indentation actuelle à la prochaine ligne

-- recherche
opt.ignorecase = true -- ignore la casse quand on recherche
opt.smartcase = true -- sauf quand on fait une recherche avec des majuscules, on rebascule en sensible à la casse
opt.hlsearch = true -- surlignage de toutes les occurences de la recherche en cours

-- ligne du curseur
opt.cursorline = true -- surlignage de la ligne active

-- apparence

-- termguicolors est nécessaire pour que les thèmes modernes fonctionnent
opt.termguicolors = true
opt.background = "dark" -- dark ou light en fonction de votre préférence
opt.signcolumn = "yes" -- affiche une colonne en plus à gauche pour afficher les signes (évite de décaler le texte)

-- retour
opt.backspace = "indent,eol,start" -- on autorise l'utilisation de retour quand on indente, à la fin de ligne ou au début

-- presse papier
opt.clipboard = "unnamedplus" -- on utilise le presse papier du système par défaut

-- split des fenêtres
opt.splitright = true -- le split vertical d'une fenêtre s'affiche à droite
opt.splitbelow = true -- le split horizontal d'une fenêtre s'affiche en bas

opt.swapfile = false -- on supprime le pénible fichier de swap

opt.undofile = true -- on autorise l'undo à l'infini (même quand on revient sur un fichier qu'on avait fermé)

opt.iskeyword:append("-") -- on traite les mots avec des - comme un seul mot

-- affichage des caractères spéciaux
opt.list = true
opt.listchars:append({ nbsp = "␣", trail = "•", precedes = "«", extends = "»", tab = "> " })
```

Gardez à l'esprit que ce sont mes préférences personnelles encore une fois. Libre à vous de modifier ce que vous voulez ici.

À savoir que ce code _Lua_ est l'équivalent de ce qui s'exprimait en _Vimscript_ de cette façon auparavant :

```vim
set ignorecase            " Ignore la casse lors d'une recherche
set smartcase             " Si une recherche contient une majuscule,
                          " re-active la sensibilite a la casse
set hlsearch              " Surligne les resultats de recherche

set backspace=indent,eol,start
```

Toutes les options classiques de _Vim_ peuvent donc être utilisées de la même manière en _Lua_ à quelques différences de syntaxe près.

Rappel à toute fin utile, pour sauvegarder et quitter _Neovim_ utilisez `:wq`.

Maintenant que nous avons mis en place nos options par défaut, il faut que _Neovim_ les prenne en compte. Pour ce faire, nous devons explicitement lui dire de charger le fichier `options.lua`. Nous allons transformer notre répertoire `core` en module _Lua_. Pour ce faire, _Lua_ a besoin d'un fichier `init.lua` qu'il chargera automatiquement à la racine du répertoire.

Éditez ce fichier :

```bash
nvim lua/core/init.lua
```

Puis placez-y le code suivant :

```lua
require("core.options")
```

Cela va notifier à _Lua_ que lorsque nous allons inclure notre module `core` il faudra qu'il inclue par défaut le fichier `core/options.lua`. Notez que le chemin est relatif au répertoire de base `~/.config/nvim/lua`.

Il faut maintenant charger notre module `core` (qui chargera automatiquement `core.options` ensuite) dans notre `init.lua` principal. Soyez sûr d'être dans le répertoire `~/.config/nvim` puis éditez `init.lua` :

```bash
nvim init.lua
```

Placez-y le contenu suivant :

```lua
require("core")
```

Sauvegardez, quittez, puis relancez _Neovim_. La configuration devrait avoir été prise en compte (le numéro des lignes devrait être relatif à la position de votre curseur par exemple).

**Résumons** : _Neovim_ charge par défaut `~/.config/nvim/init.lua` qui lui-même charge `~/.config/nvim/lua/core/init.lua` (grâce au `require("core")`) qui va ensuite charger `~/.config/nvim/lua/core/options.lua` (grâce au `require("core.options")`).

## Raccourcis clavier

Maintenant que nous avons mis en place un fichier pour configurer les options par défaut, nous allons faire de même pour configurer nos raccourcis.

Créez le fichier correspondant :

```bash
nvim lua/core/keymaps.lua
```

Puis placez-y vos raccourcis. Voici un exemple de quelques raccourcis que j'utilise :

**`lua/core/keymaps.lua`**

```lua
-- On définit notre touche leader sur espace
vim.g.mapleader = " "

-- Raccourci pour la fonction set
local keymap = vim.keymap.set

-- on utilise ;; pour sortir du monde insertion
keymap("i", ";;", "<ESC>", { desc = "Sortir du mode insertion avec ;;" })

-- on efface le surlignage de la recherche
keymap("n", "<leader>nh", ":nohl<CR>", { desc = "Effacer le surlignage de la recherche" })

-- I déplace le texte sélectionné vers le haut en mode visuel (activé avec v)
keymap("v", "<S-i>", ":m .-2<CR>==", { desc = "Déplace le texte sélectionné vers le haut en mode visuel" })
-- K déplace le texte sélectionné vers le bas en mode visuel (activé avec v)
keymap("v", "<S-k>", ":m .+1<CR>==", { desc = "Déplace le texte sélectionné vers le bas en mode visuel" })

-- I déplace le texte sélectionné vers le haut en mode visuel bloc (activé avec V)
keymap("x", "<S-i>", ":move '<-2<CR>gv-gv", { desc = "Déplace le texte sélectionné vers le haut en mode visuel bloc" })
-- K déplace le texte sélectionné vers le bas en mode visuel (activé avec V)
keymap("x", "<S-k>", ":move '>+1<CR>gv-gv", { desc = "Déplace le texte sélectionné vers le bas en mode visuel bloc" })

```

Libre à vous de mettre les raccourcis que vous souhaitez. Vous aurez compris que la fonction se comporte comme les fonctions `nmap`, `imap`, … classiques de _Vim_ sauf que vous spécifiez le mode (normal, insertion,…) comme premier paramètre. Notez aussi le 4ème paramètre de la fonction `keymap.set`. Il prend un dictionnaire Lua avec plusieurs valeurs possibles ([tous ceux la fonction map](<https://neovim.io/doc/user/api.html#nvim_set_keymap()>)) et notamment la valeurs `desc` qui va vous permettre de spécifier un mémo pour vous rappeler de ce que fait ce raccourci. Je vous **conseille fortement de vous astreindre à le remplir** car ça pourra être très utile plus tard dans le cas d'utilisation de plugins comme [which-key](https://github.com/folke/which-key.nvim).

Il nous reste maintenant à charger automatiquement ces raccourcis lorsque que l'on fait un `require("core")`. Pour ce faire, éditez `lua/core/init.lua` :

```bash
nvim lua/core/init.lua
```

Et faites en sorte qu'il contienne le code suivant :

```lua
require("core.options")
require("core.keymaps")
```

Sauvegardez, quittez et relancez : vous devriez avoir vos raccourcis claviers pris en compte.

Pour information, à ce stade, votre répertoire `~/.config/nvim/` devrait avoir le contenu suivant :

```
~/.config/nvim
├── init.lua
└── lua
    ├── core
    │   ├── init.lua
    │   ├── keymaps.lua
    │   └── options.lua
    └── plugins
```

## Gestionnaire de plugins : `lazy.nvim`

Nous allons utiliser [lazy.nvim](https://lazy.folke.io/) pour gérer l'installation et la configuration de nos différents plugins. C'est le gestionnaire de plugins le plus utilisé actuellement dans la communauté et il remplace avantageusement [packer.nvim](https://github.com/wbthomason/packer.nvim).

> ⚠️ **Attention** nous parlons bien ici du gestionnaire de plugins `lazy.nvim` et non de la _distribution Neovim_ [LazyVim](https://www.lazyvim.org/) basée sur ce gestionnaire de plugins. La distribution _LazyVim_ a pour but de vous fournir un _Neovim_ entièrement configuré et prêt à l'emploi, ce qui est le complet opposé du but de cet article.

Commençons par créer le répertoire et le fichier qui va accueillir la configuration de `lazy.nvim`.

```bash
mkdir lua/config/
touch lua/config/lazy.lua
```

Éditez `lua/config/lazy.lua` et placez-y le code suivant (issu de la documentation de `lazy.nvim`) :

**`lua/config/lazy.lua`**

```lua
-- Mise en place et installation de lazy.nvim
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not (vim.uv or vim.loop).fs_stat(lazypath) then
  local lazyrepo = "https://github.com/folke/lazy.nvim.git"
  local out = vim.fn.system({ "git", "clone", "--filter=blob:none", "--branch=stable", lazyrepo, lazypath })
  if vim.v.shell_error ~= 0 then
    vim.api.nvim_echo({
      { "Failed to clone lazy.nvim:\n", "ErrorMsg" },
      { out, "WarningMsg" },
      { "\nPress any key to exit..." },
    }, true, {})
    vim.fn.getchar()
    os.exit(1)
  end
end
vim.opt.rtp:prepend(lazypath)

-- Configuration de lazy.nvim
require("lazy").setup({
  spec = {
    -- importation de notre module plugins
    { import = "plugins" },
  },
  -- vérifie automatiquement les mises à jour des plugins
  checker = { enabled = true },
})
```

Créez et éditez ensuite le fichier `lua/plugins/init.lua` en y plaçant le contenu suivant :

**`lua/plugins/init.lua`**

```lua
return {
  "nvim-lua/plenary.nvim", -- ensemble de fonctions lua utilisées par de nombreux plugins
}
```

Ce fichier, lancé au chargement de notre module `lua/plugins` peut contenir tout la liste des plugins que vous souhaitez voir installés par défaut avec si besoin, la configuration associée. Même si n'utiliser que ce fichier est possible, nous allons procéder différemment. Comme recommandé dans la [documentation de `lazy.nvim`](https://lazy.folke.io/usage/structuring) nous allons plutôt utiliser un fichier par plugin au lieu de tout mettre dans `lua/plugins/init.lua`. Quoiqu'il en soit, les contenus de `lua/plugins/init.lua` et des fichiers de plugins `lua/plugins/*.lua` seront fusionnés au chargement de `lazy.nvim`, donc les deux sont possibles et compatibles l'un avec l'autre.

À noter que `lazy.nvim` va chercher les plugins par défaut sur _Github_ mais il est possible de directement lui spécifier n'importe quel dépôt git ou n'importe quel répertoire local.

## Thème de couleurs : `tokyonight.nvim`
