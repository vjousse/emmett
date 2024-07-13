---
title: "Tutoriel : configurer Neovim comme IDE/éditeur de code à partir de zéro"
date: "2024-07-13 09:33:20+01:00"
slug: configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide
tags: neovim, tutoriel, lua, vim
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

## TL;DR

[La configuration finale est disponible sur Github](https://github.com/vjousse/neovim-from-scratch).

## Préambule

_Neovim_ sans [Lua](https://www.lua.org/) c'est comme Milan sans Rémo, ça n'a aucun sens (seuls les vieux auront [la référence](https://www.bide-et-musique.com/song/149.html), les autres vous pouvez continuer de lire en ignorant cette disgression 🤓).

Nous allons donc configurer notre _Neovim_ entièrement et uniquement en [Lua](https://www.lua.org/), fini le _Vimscript_. Mais rassurez-vous, vous n'aurez besoin d'aucune connaissance particulière en _Lua_. Moi-même, je ne connais que très peu _Lua_ et je ne le pratique que dans le cadre de ma configuration _Vim_. Si vous voulez néanmoins avoir quelques bases vous pouvez jetez un œil à [Learn X in Y minutes where X=Lua](https://learnxinyminutes.com/docs/lua/).

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

## Gestionnaire de plugins : [`lazy.nvim`](https://lazy.folke.io/)

Nous allons utiliser [lazy.nvim](https://lazy.folke.io/) pour gérer l'installation et la configuration de nos différents plugins. C'est le gestionnaire de plugins le plus utilisé actuellement dans la communauté et il remplace avantageusement [packer.nvim](https://github.com/wbthomason/packer.nvim).

![Capture d'écran montrant Neovim avec lazy-nvim](/images/configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide/lazy-nvim.png "Capture d'écran montrant Neovim avec lazy-nvim")

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

-- Configuration de lazy.nvim et importation du répertoire `plugins`
require("lazy").setup({ { import = "plugins" } }, {
  -- désactive la pénible notification au démarrage
  change_detection = {
    notify = false,
  },
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

Quelques subtilités à connaître :

- Vous pouvez lancer la fenêtre de gestion des plugins via `:Lazy`
- Vous pouvez quitter la dite fenêtre en appuyant sur `q`
- Appuyez sur `U` pour mettre automatiquement à jour tous les plugins dans la fenêtre de _Lazy_

## Un joli _Neovim_, le thème [`tokyonight.nvim`](https://github.com/folke/tokyonight.nvim)

Nous allons utiliser par défaut le thème [`tokyonight.nvim`](https://github.com/folke/tokyonight.nvim). Libre à vous d'en utiliser un autre si vous voulez (vous en trouverez des exemples [sur le site dotfyle par exemple](https://dotfyle.com/neovim/colorscheme/trending)) mais celui-ci a l'avantage d'être disponible en plusieurs versions sombres ou claires (_Moon, Storm, Night, Day_) et est aussi supporté dans nombres d'autres applications comme WezTerm (pratique pour avoir un terminal avec le même thème que votre _Neovim_).

![Capture d'écran montrant les différentes variantes du thème tokyonight](/images/configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide/tokyonight.png "Capture d'écran montrant les différentes variantes du thème tokyonight")

Créez le fichier `lua/plugins/tokyonight.lua` :

```bash
nvim lua/plugins/tokyonight.lua
```

Et placez-y le contenu suivant :

**`lua/plugins/tokyonight.lua`**

```lua
return {
  "folke/tokyonight.nvim",
  lazy = false,
  priority = 1000,
  opts = {},
  config = function()
    -- chargement du thème
    vim.cmd([[colorscheme tokyonight]])
  end,
}
```

Quittez et relancez _Neovim_ : le thème devrait maintenant être activé par défaut !

Vous pouvez aussi activer `tokyonight` lors du chargement de la fenêtre d'installation des nouveaux plugins par _lazy.nvim_ au chargement de _Neovim_ (par défaut il utilise un autre thème). Pour ce faire modifiez `lua/config/lazy.lua` et ajoutez la ligne `install = { colorscheme = { "tokyonight" } }` :

```lua
-- … début du fichier

-- Configuration de lazy.nvim et importation du répertoire `plugins`
require("lazy").setup({ { import = "plugins" } }, {
  -- thème utilisé lors de l'installation de plugins
  install = { colorscheme = { "tokyonight" } },
  -- désactive la pénible notification au démarrage
  change_detection = {
    notify = false,
  },
})
```

## L'explorateur de fichiers : [`nvim-tree.lua`](https://github.com/nvim-tree/nvim-tree.lua)

Éditez `lua/plugins/nvim-tree.lua` et placez-y le code suivant :

**`lua/plugins/nvim-tree.lua`**

```lua
return {
  "nvim-tree/nvim-tree.lua",
  version = "*",
  lazy = false,
  dependencies = {
    "nvim-tree/nvim-web-devicons",
  },
  config = function()
    require("nvim-tree").setup({})

    -- On utilise <leader>e pour ouvrir/fermer l'explorateur
    vim.keymap.set(
      "n",
      "<leader>e",
      "<cmd>NvimTreeFindFileToggle<CR>",
      { desc = "Ouverture/fermeture de l'explorateur de fichiers" }
    )
  end,
}
```

Par défaut j'utilise `<leader>e` pour ouvrir fermer mon explorateur, mais libre à vous de changer ce raccourci (pour rappel mon `<leader>` est la touche espace).

Vous trouverez tous les mappings par défaut et comment les modifier dans la [documentation du plugin](https://github.com/nvim-tree/nvim-tree.lua#custom-mappings).

Je vous recommande chaudement de rajouter par la même occasion ces mappings dans `lua/core/keymaps.lua` :

```lua
-- Changement de fenêtre avec Ctrl + déplacement uniquement au lieu de Ctrl-w + déplacement
keymap("n", "<C-h>", "<C-w>h", { desc = "Déplace le curseur dans la fenêtre de gauche" })
keymap("n", "<C-j>", "<C-w>j", { desc = "Déplace le curseur dans la fenêtre du bas" })
keymap("n", "<C-k>", "<C-w>k", { desc = "Déplace le curseur dans la fenêtre du haut" })
keymap("n", "<C-l>", "<C-w>l", { desc = "Déplace le curseur dans la fenêtre droite" })
```

Ça va vous permettre de passer facilement de la fenêtre `nvim-tree` à votre fenêtre d'édition avec `Ctrl-h` et `Ctrl-l` au lieu de `Ctrl-w h` et `Ctrl-w l` par défaut. Sauvegardez, quittez et relancez _Neovim_.

## Mise en place de [`telescope.nvim`](https://github.com/nvim-telescope/telescope.nvim) : le plugin de fuzzy finding dont vous avez toujours rêvé

`telescope.nvim` vu nous permettre de chercher un peu tout et n'importe quoi partout en utilisant une technique de recherche floue/approximative. En gros, tapez un bout de ce que vous voulez chercher (que ça soit un mot, des mots, de bouts de mots, peu importe) et telescope fera le reste à l'aide de [fzf](https://github.com/junegunn/fzf).

Nous allons placer la configuration de `telescope.nvim` dans `lua/plugins/telescope.lua`.

Vous pouvez l'éditer via `nvim lua/plugins/telescope.lua` comme d'habitude ou alors vous pouvez utiliser le plugin `nvim-tree` fraichement installé. Pour ce faire activez le avec `<leader>e`, entrez dans le répertoire `lua/plugins` (via la touche `entrée`) puis appuyez sur `a` pour créer un fichier. Nommez-le `telescope.lua` et appuyez sur `entrée` pour le créer. Appuyez de nouveau sur `entrée` pour l'ouvrir en édition.

**`lua/plugins/telescope.lua`**

```lua
return {
  "nvim-telescope/telescope.nvim",
  branch = "0.1.x",
  dependencies = {
    "nvim-lua/plenary.nvim",
    -- fzf implémentation en C pour plus de rapidité
    { "nvim-telescope/telescope-fzf-native.nvim", build = "make" },
    "nvim-tree/nvim-web-devicons",
  },
  config = function()
    local telescope = require("telescope")
    local actions = require("telescope.actions")

    telescope.setup({
      defaults = {

        -- Parce que c'est joli
        prompt_prefix = " ",
        selection_caret = " ",
        path_display = { "smart" },
        file_ignore_patterns = { ".git/", "node_modules" },

        mappings = {
          i = {
            ["<C-j>"] = actions.move_selection_next,
            ["<C-k>"] = actions.move_selection_previous,
          },
        },
      },
    })

    telescope.load_extension("fzf")

    -- set keymaps
    local keymap = vim.keymap -- for conciseness

    keymap.set(
      "n",
      "<leader>ff",
      "<cmd>Telescope find_files<cr>",
      { desc = "Recherche de chaînes de caractères dans les noms de fichiers" }
    )
    keymap.set(
      "n",
      "<leader>fg",
      "<cmd>Telescope live_grep<cr>",
      { desc = "Recherche de chaînes de caractères dans le contenu des fichiers" }
    )
    keymap.set(
      "n",
      "<leader>fb",
      "<cmd>Telescope buffers<cr>",
      { desc = "Recherche de chaînes de caractères dans les noms de buffers" }
    )
    keymap.set(
      "n",
      "<leader>fx",
      "<cmd>Telescope grep_string<cr>",
      { desc = "Recherche de la chaîne de caractères sous le curseur" }
    )
  end,
}
```

J'ai configuré quelques raccourcis par défaut adaptés à mon utilisation :

- `Ctrl-k` pour remonter dans la liste de sélection
- `Ctrl-j` pour descencdre dans la liste de sélection
- `<leader>ff` pour chercher dans les noms de fichiers
- `<leader>fg` pour chercher dans les contenus des fichiers
- `<leader>fb` pour chercher dans les noms de buffers
- `<leader>fx` pour chercher le mot sous le curseurs dans le contenu des fichiers

Libre à vous d'en paramètrer d'autres ou d'utiliser la [list des raccourcis déjà disponibles](https://github.com/nvim-telescope/telescope.nvim#default-mappings) par défaut.

## Affichage des buffers et barre d'onglets : [`bufferline.nvim`](https://github.com/akinsho/bufferline.nvim)

Pour pouvoir facilement avoir un aperçu de nos buffers en cours, nous allons utiliser [bufferline.nvim](https://github.com/akinsho/bufferline.nvim).

Éditez `lua/plugins/bufferline.lua` et placez-y le code suivant :

**`lua/plugins/bufferline.lua`**

```lua
return {
  "akinsho/bufferline.nvim",
  dependencies = { "nvim-tree/nvim-web-devicons" },
  version = "*",
  opts = {
    options = {
      separator_style = "slant",
      offsets = { { filetype = "NvimTree", text = "", padding = 1 } },
    },
  },
}
```

Sauvegardez, quitter, relancez et vous devriez maintenant avoir une belle barre d'onglets en haut de votre _Neovim_.

Personnellement, j'ai aussi ces raccourcis dans mon `lua/core/keymaps.lua` :

```lua
-- Navigation entre les buffers
keymap("n", "<S-l>", ":bnext<CR>", opts)
keymap("n", "<S-h>", ":bprevious<CR>", opts)
```

Ça me permet, en mode normal, de passer d'un buffer à l'autre via `L` et `H`. Vous pouvez aussi utiliser telescope et `<leader>fb` pour naviguer dans vos buffers ouverts.

## Barre de statut dopée aux stéroïdes : [`lualine`](https://github.com/nvim-lualine/lualine.nvim)

Pour configurer [lualine](https://github.com/nvim-lualine/lualine.nvim), comme d'habitude, éditez `lua/plugins/lualine.lua` et placez-y le code suivant :

**`lua/plugins/lualine.lua`**

```lua
return {
  "nvim-lualine/lualine.nvim",
  dependencies = { "nvim-tree/nvim-web-devicons" },
  config = function()
    local lualine = require("lualine")
    local lazy_status = require("lazy.status") -- affiche le nombre de mise à jour plugins lazy dans la barre

    -- configuration de lualine
    lualine.setup({
      options = {
        icons_enabled = true,
        theme = "auto",
        component_separators = { left = "", right = "" },
        section_separators = { left = "", right = "" },
        disabled_filetypes = {
          statusline = {},
          winbar = {},
        },
        ignore_focus = {},
        always_divide_middle = true,
        globalstatus = false,
        refresh = {
          statusline = 1000,
          tabline = 1000,
          winbar = 1000,
        },
      },
      sections = {
        lualine_a = { "mode" },
        lualine_b = { "branch", "diff", "diagnostics" },
        lualine_c = { { "filename", path = 1 } },
        lualine_x = {
          {
            lazy_status.updates,
            cond = lazy_status.has_updates,
            color = { fg = "#ff9e64" },
          },
          { "encoding" },
          { "fileformat" },
          { "filetype" },
        },
        lualine_y = { "progress" },
        lualine_z = { "location" },
      },
      inactive_sections = {
        lualine_a = {},
        lualine_b = {},
        lualine_c = { "filename" },
        lualine_x = { "location" },
        lualine_y = {},
        lualine_z = {},
      },
      tabline = {},
      winbar = {},
      inactive_winbar = {},
      extensions = {},
    })
  end,
}
```

C'est ma config personnelle donc libre à vous de modifier comme vous le souhaitez. Je vous laisse consulter la [page du plugin](https://github.com/nvim-lualine/lualine.nvim) pour découvrir toutes les options possibles !

Au passage, modifiez la configuration de `lazy.nvim` dans `lua/config/lazy.lua` pour ajouter la vérification automatique des mises à jour :

```lua
-- Configuration de lazy.nvim et importation du répertoire `plugins`
require("lazy").setup({ { import = "plugins" } }, {
  -- vérifie automatiquement les mises à jour des plugins mais sans notifier
  -- lualine va se charger de nous afficher un icône
  checker = {
    enabled = true,
    notify = false,
  },
  -- thème utilisé lors de l'installation de plugins
  install = { colorscheme = { "tokyonight" } },
  -- désactive la pénible notification au démarrage
  change_detection = {
    notify = false,
  },
})
```

## Installation de [`nvim-treesitter`](https://github.com/nvim-treesitter/nvim-treesitter)

[tree-sitter](https://tree-sitter.github.io/tree-sitter/) est un outil incroyable (non spécifique à _Neovim_) qui va permettre de parser et de « comprendre » la syntaxe d'un grand nombre de langages de programmation. Son intégration dans _Neovim_ à l'aide de [`nvim-treesitter`](https://github.com/nvim-treesitter/nvim-treesitter) va permettre une meilleure coloration syntaxique, de l'indentation plus intelligente, des tags automatiques, des sélections intelligentes en fonction du langage de programmation et du contexte, j'en passe et des meilleures. Bref, c'est un plugin indispensable.

Éditez `lua/plugins/treesitter.lua` :

**`lua/plugins/treesitter.lua`**

```lua
return {
  "nvim-treesitter/nvim-treesitter",
  build = ":TSUpdate",
  config = function()
    local treesitter = require("nvim-treesitter.configs")

    -- configuration de treesitter
    treesitter.setup({
      -- activation de la coloration syntaxique
      highlight = {
        enable = true,
      },
      -- activation de l'indentation améliorée
      indent = { enable = true },

      -- langages installés et configurés
      ensure_installed = {
        "bash",
        "dockerfile",
        "gitignore",
        "html",
        "javascript",
        "json",
        "lua",
        "markdown",
        "markdown_inline",
        "python",
        "rst",
        "rust",
        "typescript",
        "vim",
        "yaml",
      },
      -- lorse de l'appui sur <Ctrl-space> sélectionne le bloc
      -- courant spécifique au langage de programmation
      incremental_selection = {
        enable = true,
        keymaps = {
          init_selection = "<C-space>",
          node_incremental = "<C-space>",
          scope_incremental = false,
          node_decremental = "<bs>",
        },
      },
    })
  end,
}
```

Encore une fois, c'est ma configuration personnelle, libre à vous de la modifier comme vous le souhaitez. La [liste des langages supportés](https://github.com/nvim-treesitter/nvim-treesitter#supported-languages) est disponible sur le dépôt Github.

## Mise en place de l'autocompletion avec [`nvim-cmp`](https://github.com/hrsh7th/nvim-cmp)

[`nvim-cmp`](https://github.com/hrsh7th/nvim-cmp) va nous permettre d'avoir un système de complétion pour un peu tout et n'importe quoi : les fonctions du langage, des snippets, des emojis, … Ce plugin fourni juste l'interface de complétion, il devra par la suite être configuré avec les sources de ces complétions (le serveur du langage de programmation, les snippets, etc).

Éditez `lua/plugins/nvim-cmp.lua` et mettez-y le code suivant :

**`lua/plugins/nvim-cmp.lua`**

```lua
return {
  "hrsh7th/nvim-cmp",
  event = { "InsertEnter", "CmdlineEnter" },
  dependencies = {
    "hrsh7th/cmp-buffer", -- source pour compléter le texte déjà présent dans le buffer
    "hrsh7th/cmp-path", -- source pour compléter les chemins des fichiers
    "hrsh7th/cmp-cmdline", -- source pour les completions de la cmdline de vim
    {
      "L3MON4D3/LuaSnip",
      -- follow latest release.
      version = "v2.*", -- Replace <CurrentMajor> by the latest released major (first number of latest release)
      -- install jsregexp (optional!).
      build = "make install_jsregexp",
    },
    "saadparwaiz1/cmp_luasnip", -- ajoute LuaSnip à l'autocompletion
    "rafamadriz/friendly-snippets", -- collection de snippets pratiques
    "hrsh7th/cmp-emoji", -- complétion d'émojis à la saisie de :
    "onsails/lspkind.nvim", -- vs-code pictogrammes
  },
  config = function()
    local cmp = require("cmp")

    local luasnip = require("luasnip")

    local lspkind = require("lspkind")

    -- chargement des snippets (e.g. friendly-snippets)
    require("luasnip.loaders.from_vscode").lazy_load()

    cmp.setup({
      completion = {
        completeopt = "menu,menuone,preview,noselect",
      },
      snippet = { -- on utilise luasnip comme moteur de snippets
        expand = function(args)
          luasnip.lsp_expand(args.body)
        end,
      },
      mapping = {
        ["<C-k>"] = cmp.mapping.select_prev_item(),
        ["<C-j>"] = cmp.mapping.select_next_item(),
        ["<C-b>"] = cmp.mapping.scroll_docs(-1),
        ["<C-f>"] = cmp.mapping.scroll_docs(1),
        ["<C-Space>"] = cmp.mapping.complete(),
        ["<C-e>"] = cmp.mapping.abort(),
        ["<CR>"] = cmp.mapping.confirm({ select = true }), -- Accepte la sélection courante. Mettre à `false` pour ne confirmer que les items explicitement sélectionnés
      },

      -- sources pour l'autocompletion
      sources = cmp.config.sources({
        { name = "nvim_lua" },
        { name = "luasnip" }, -- snippets
        { name = "buffer" }, -- texte du buffer courant
        { name = "path" }, -- chemins dy système de fichier
        { name = "emoji" }, -- emojis
      }),

      formatting = {
        -- Comportement par défaut
        expandable_indicator = true,
        -- Champs affichés par défaut
        fields = { "abbr", "kind", "menu" },
        format = lspkind.cmp_format({
          mode = "symbol_text",
          -- On suffixe chaque entrée par son type
          menu = {
            buffer = "[Buffer]",
            luasnip = "[LuaSnip]",
            nvim_lua = "[Lua]",
            path = "[Path]",
            emoji = "[Emoji]",
          },
        }),
      },
    })

    -- `/` complétion
    cmp.setup.cmdline("/", {
      mapping = cmp.mapping.preset.cmdline(),
      sources = {
        { name = "buffer" },
      },
    })

    -- `:` complétion
    cmp.setup.cmdline(":", {
      mapping = cmp.mapping.preset.cmdline(),
      sources = cmp.config.sources({
        { name = "path" },
      }, {
        {
          name = "cmdline",
          option = {
            ignore_cmds = { "Man", "!" },
          },
        },
      }),
    })

  end,
}
```

Ce code configure `nvim-cmp` pour fournir comme propositions d'autocomplétion le texte du buffer courant (via [`cmp-buffer`](https://github.com/hrsh7th/cmp-buffer)), les chemins de fichiers de votre disque local (via [`cmp-path`](https://github.com/hrsh7th/cmp-path)), des snippets (via [`LuaSnip`](https://github.com/L3MON4D3/LuaSnip) et [`friendly-snippets`](https://github.com/rafamadriz/friendly-snippets)), les commandes vim (via [`cmp-cmdline`](https://github.com/hrsh7th/cmp-cmdline)) et des emojis (via [`cmp-emoji`](https://github.com/hrsh7th/cmp-emoji)). Si vous souhaitez ajouter d'autres sources de complétion, vous trouverez une [liste des différentes sources possibles sur le wiki](https://github.com/hrsh7th/nvim-cmp/wiki/List-of-sources).

J'ai aussi ajouté l'affichage d'un petit pictogramme à la vscode devant chaque entrée de complétion via le plugin [`lspkind`](https://github.com/onsails/lspkind.nvim).

Pour ceux qui se posent la question : pour l'instant, il n'y a pas de complétion des fonctions/du code source, ça vient dans le prochaine chapitre !

## Support des LSP (Language Server Protocol)

Les LSP sont des protocoles qui vont permettre à _Neovim_ de « connaître » le langage de programmation sur lequel vous travaillez. C'est grâce à eux que vous pourrez obtenir la complétion automatique ou encore le « go to definition » qui permet de facilement naviguer dans son code. Ces protocoles ne sont pas spécifiques à _Neovim_ et sont utilisés par les principaux éditeurs de texte.

Les LSP et la complétion automatique du code sont un peu les **boss de fin de niveau d'un jeu vidéo pour _Neovim_**. Et comme tout jeu vidéo qui se respecte, vous pouvez y jouer avec plusieurs niveaux de difficulté.

Il y a tout d'abord le niveau de **difficulté hardcore** décrit dans ce post de blog : [A guide on Neovim's LSP client](https://vonheikemen.github.io/devlog/tools/neovim-lsp-client-guide/). Ce post explique comment configurer la complétion automatique sans utiliser aucun plugin.

Ensuite il y a le **niveau intermédiaire** expliqué dans ce post de blog : [You might not need lsp-zero](https://lsp-zero.netlify.app/v3.x/blog/you-might-not-need-lsp-zero). L'idée ici est d'utiliser des plugins qui gèrent l'installation des LSP, la complétion, etc et de les mettre ensemble soi-même. C'est d'ailleurs ce que je fais dans [cette configuration là](https://github.com/vjousse/dotfiles/tree/b35c4654c589f2bcbdcda64dc3cfd14d2feaedfb/nvim-lazy).

Et puis il y a le **niveau c'est pas hyper facile mais ça va quand même** qu'on va décrire ici, via l'utilisation de [`lsp-zero`](https://lsp-zero.netlify.app/v4.x/).

### Explication de la problématique

Avant toute chose, je vais je vous expliquer pourquoi mettre en place la complétion avec les LSP n'est pas si trivial.

Premièrement, vous allez devoir disposer localement des LSP. Les LSP sont juste des programmes qui doivent être présents sur votre système. Il en faut un (ou des fois plusieurs) pour chaque langage de programmation pour lequel vous voudrez la complétion et les actions automatiques de type IDE (renommage, etc). Vous pourriez tout à fait les installer via votre système d'exploitation, mais il est possible d'installer un gestionnaire de paquets directement dans _Neovim_ qui va gérer tout ça pour vous : [mason.nvim](https://github.com/williamboman/mason.nvim). En plus d'automatiser l'installation des LSP il va rendre notre configuration de _Neovim_ complètement portable car les LSP requis seront spécifiés dans des fichiers _Lua_ et installés automatiquement (vous pourrez donc versionner tout ça sur Git).

Ensuite, nous allons avoir besoin d'un moyen pour configurer ces LSP de manière unifiée. Si par exemple vous voulez rajouter telle option à votre LSP python ou telle autre à votre LSP javascript. Pour ce faire, nous allons utiliser [`nvim-lspconfig`](https://github.com/neovim/nvim-lspconfig).

Maintenant que nous avons de quoi installer nos LSP et de quoi les configurer, il va falloir faire le lien entre les deux : les installer automatiquement avec `mason.nvim` et les configurer via `nvim-lspconfig` lorsqu'ils sont installés et chargés par `mason.nvim`. C'est le plugin [`mason-lspconfig`](https://github.com/williamboman/mason-lspconfig.nvim) qui va nous aider à faire ce lien.

Et pour finir nous utiliserons le plugin [`lsp-zero`](https://lsp-zero.netlify.app/v4.x/) qui rendra pas mal de code un peu plus simple.

### Préparation du répertoire

Dans le répertoire `lua/plugins` créez un nouveau répertoire nommé `lsp` dans lequel vous mettrez la configuration de tout ce qui est relatif aux LSP :

```bash
mkdir lua/plugins/lsp
```

Éditez ensuite `lua/config/lazy.lua` pour y ajouter l'import du répertoire `plugins/lsp` :

**`lua/config/lazy.lua`**

```lua
-- … début du fichier

-- Configuration de lazy.nvim et importation des répertoires `plugins` et `plugins.lsp`
require("lazy").setup({ { import = "plugins" }, { import = "plugins.lsp"} }, {
  -- vérifie automatiquement les mises à jour des plugins mais sans notifier
  -- lualine va se charger de nous afficher un icône
  checker = {
    enabled = true,
    notify = false,
  },
  -- thème utilisé lors de l'installation de plugins
  install = { colorscheme = { "tokyonight" } },
  -- désactive la pénible notification au démarrage
  change_detection = {
    notify = false,
  },
})

```

### Préparer la configuration des LSP : [`nvim-lspconfig`](https://github.com/neovim/nvim-lspconfig)

Commençons par installer de quoi configurer nos LSP via [`nvim-lspconfig`](https://github.com/neovim/nvim-lspconfig) et [`lsp-zero`](https://lsp-zero.netlify.app/v4.x/).

Éditez `lua/plugins/lsp/lspconfig.lua` et placez-y le contenu suivant :

**`lua/plugins/lsp/lspconfig.lua`**

```lua
return {
  "neovim/nvim-lspconfig",
  event = { "BufReadPre", "BufNewFile" },
  dependencies = {
    -- Va permetre de remplir le plugin de complétion automatique nvim-cmp
    -- avec les résultats des LSP
    "hrsh7th/cmp-nvim-lsp",
    -- Ajoute les « code actions » de type renommage de fichiers intelligent, etc
    { "antosha417/nvim-lsp-file-operations", config = true },
  },
  config = function()
    -- import de lsp-zero
    local lsp_zero = require("lsp-zero")

    -- lsp_attach sert à activer des fonctionnalités qui ne seront disponibles
    -- que s'il il y a un LSP d'activé pour le fichier courant
    local lsp_attach = function(_, bufnr)
      local opts = { buffer = bufnr, silent = true }

      -- configuration des raccourcis
      -- je ne vous les traduis pas, ils me semblent parler d'eux-même ;)
      opts.desc = "Show LSP references"
      vim.keymap.set("n", "gR", "<cmd>Telescope lsp_references<CR>", opts) -- show definition, references

      opts.desc = "Go to declaration"
      vim.keymap.set("n", "gD", vim.lsp.buf.declaration, opts) -- go to declaration

      opts.desc = "Show LSP definitions"
      vim.keymap.set("n", "gd", "<cmd>Telescope lsp_definitions<CR>", opts) -- show lsp definitions

      opts.desc = "Show LSP implementations"
      vim.keymap.set("n", "gi", "<cmd>Telescope lsp_implementations<CR>", opts) -- show lsp implementations

      opts.desc = "Show LSP type definitions"
      vim.keymap.set("n", "gt", "<cmd>Telescope lsp_type_definitions<CR>", opts) -- show lsp type definitions

      opts.desc = "Show LSP signature help"
      vim.keymap.set("n", "gs", vim.lsp.buf.signature_help, opts)

      opts.desc = "See available code actions"
      vim.keymap.set({ "n", "v" }, "<leader>ca", vim.lsp.buf.code_action, opts) -- see available code actions, in visual mode will apply to selection

      opts.desc = "Smart rename"
      vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, opts) -- smart rename

      opts.desc = "Show buffer diagnostics"
      vim.keymap.set("n", "<leader>D", "<cmd>Telescope diagnostics bufnr=0<CR>", opts) -- show  diagnostics for file

      opts.desc = "Show line diagnostics"
      vim.keymap.set("n", "<leader>d", vim.diagnostic.open_float, opts) -- show diagnostics for line

      opts.desc = "Go to previous diagnostic"
      vim.keymap.set("n", "[d", function()
        vim.diagnostic.jump({ count = -1, float = true })
      end, opts) -- jump to previous diagnostic in buffer

      opts.desc = "Go to next diagnostic"
      vim.keymap.set("n", "]d", function()
        vim.diagnostic.jump({ count = 1, float = true })
      end, opts) -- jump to next diagnostic in buffer

      opts.desc = "Show documentation for what is under cursor"
      vim.keymap.set("n", "K", vim.lsp.buf.hover, opts) -- show documentation for what is under cursor

      opts.desc = "Format buffer"
      vim.keymap.set({ "n", "x" }, "F", "<cmd>lua vim.lsp.buf.format({async = true})<cr>", opts)

      opts.desc = "Restart LSP"
      vim.keymap.set("n", "<leader>rs", ":LspRestart<CR>", opts) -- mapping to restart lsp if necessary
    end

    lsp_zero.extend_lspconfig({
      -- On affiche les signes des diagnostics dans la gouttière de gauche
      sign_text = true,
      -- On attache notre fonction qui définit les raccourcis
      lsp_attach = lsp_attach,
      -- On augmente les capacités de complétion par défaut avec les propositions du LSP
      capabilities = require("cmp_nvim_lsp").default_capabilities(),
    })

    -- On utilise lsp_zero pour configurer quelques éléments de design
    lsp_zero.ui({
      float_border = "rounded",
      sign_text = {
        error = " ",
        warn = " ",
        hint = "󰠠 ",
        info = " ",
      },
    })
  end,
}
```

Je vous laisse déduire l'utilité des raccourcis configurés. Vous noterez que pas mal d'entre eux utilisent quand c'est possible un affichage directement dans _Telescope_.

### Installation des LSP : [`mason.nvim`](https://github.com/williamboman/mason.nvim)

Comme nous l'avons vu, afin de ne pas avoir à installer les LSP à la main, nous allons utiliser [`mason.nvim`](https://github.com/williamboman/mason.nvim) qui va gérer tout cela pour nous.

Éditez `lua/plugins/lsp/mason.lua` avec le code suivant :

**`lua/plugins/lsp/mason.lua`**

```lua
return {
  "williamboman/mason.nvim",
  dependencies = {
    "williamboman/mason-lspconfig.nvim",
  },
  config = function()
    -- import de mason
    local mason = require("mason")

    -- import de mason-lspconfig
    local mason_lspconfig = require("mason-lspconfig")

    -- import de lspconfig
    local lspconfig = require("lspconfig")

    -- active mason et personnalise les icônes
    mason.setup({
      ui = {
        icons = {
          package_installed = "✓",
          package_pending = "➜",
          package_uninstalled = "✗",
        },
      },
    })

    mason_lspconfig.setup({
      -- Liste des serveurs à installer par défaut
      -- List des serveurs possibles : https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md
      -- Vous pouvez ne pas en mettre ici et tout installer en utilisant :Mason
      -- Mais au lieu de passer par :Mason pour installer, je vous recommande d'ajouter une entrée à cette liste
      -- Ça permettra à votre configuration d'être plus portable
      ensure_installed = {
        "cssls",
        "elmls",
        "graphql",
        "html",
        "lua_ls",
        "pylsp",
        "ruff_lsp",
        "rust_analyzer",
        "sqlls",
        "svelte",
        "tsserver",
        "yamlls",
      },
      handlers = {
        -- Fonction appelée au chargement de chaque LSP de la liste ensure_installed
        function(server_name)
          -- On active tous les LSP de ensure_installed avec sa configuration par défaut
          lspconfig[server_name].setup({})
        end,

        -- On peut ensuite configurer chaque LSP comme on veut
        -- Les détails des configurations possibles sont disponibles ici :
        -- https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md
        -- Quelques exemples avec Python (pylsp et ruff) ainsi que Rust ci-dessous
        --
        -- Pour désactiver un LSP il suffit de faire
        -- mon_lsp = require("lsp-zero").noop,

        -- le nom du lsp avant le `= function()` doit être le même que celui après `lspconfig.`
        -- le premier est la clé utilisée par mason_lspconfig, le deuxième est celle utilisée par lspconfig (ce sont les mêmes)
        -- ils correspondent aux entrées du ensure_installed

        -- https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md#pylsp
        pylsp = function()
          lspconfig.pylsp.setup({
            settings = {
              pylsp = {
                plugins = {
                  pyflakes = { enabled = false },
                  pycodestyle = {
                    enabled = true,
                    ignore = { "E501" },
                  },
                },
              },
            },
          })
        end,

        -- https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md#ruff_lsp
        ruff_lsp = function()
          lspconfig.ruff_lsp.setup({
            init_options = {
              settings = {
                -- Arguments par défaut de la ligne de commande ruff
                -- (on ajoute les warnings pour le tri des imports)
                args = { "--extend-select", "I" },
              },
            },
          })
        end,

        -- https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md#rust_analyzer
        rust_analyzer = function()
          lspconfig.rust_analyzer.setup({
            settings = {
              ["rust-analyzer"] = {
                diagnostics = {
                  enable = true,
                  styleLints = {
                    enable = true,
                  },
                  experimental = {
                    enable = true,
                  },
                },
              },
            },
          })
        end,
      },
    })
  end,
}
```

Évidemment, libre à vous de modifier la liste des serveurs installés par défaut en fonction de vos préférences. Vous trouverez sur le site du plugin une [liste de tous les serveurs pris en charge](https://mason-registry.dev/registry/list).

### Ajout des LSP à l'autocomplétion

Nous avons fait le plus dur, il ne reste plus qu'à ajouter les LSP comme source de données de notre système de complétion `nvim-cmp`.

Éditez `lua/plugins/nvim-cmp.lua` et éditez le code pour y ajouter `nvim_lsp` comme source :

**`lua/plugins/nvim-cmp.lua`**

```lua
  -- … début du fichier

  -- sources pour l'autocompletion
  sources = cmp.config.sources({
    { name = "nvim_lsp" }, -- lsp
    { name = "nvim_lua" },
    { name = "luasnip" }, -- snippets
    { name = "buffer" }, -- texte du buffer courant
    { name = "path" }, -- chemins dy système de fichier
    { name = "emoji" }, -- emojis
  }),

  formatting = {
    -- Comportement par défaut
    expandable_indicator = true,
    -- Champs affichés par défaut
    fields = { "abbr", "kind", "menu" },
    format = lspkind.cmp_format({
      mode = "symbol_text",
      -- On suffixe chaque entrée par son type
      menu = {
        nvim_lsp = "[LSP]",
        buffer = "[Buffer]",
        luasnip = "[LuaSnip]",
        nvim_lua = "[Lua]",
        path = "[Path]",
        emoji = "[Emoji]",
      },
    }),
  },
```

Notez l'ajout de `{ name = "nvim_lsp" }, -- lsp` et de `nvim_lsp = "[LSP]",`.

Voilà, sauvegardez, quittez et relancez : la boucle est bouclée, vous devriez maintenant avoir un _Neovim_ avec les complétions automatiques et les raccourcis LSP configuré pour les langages de votre choix.

## Affichage des données et diagnostics des LSP : [`trouble.nvim`](https://github.com/folke/trouble.nvim)

Le LSP par défaut va déjà vous afficher des conseils et des diagnostics dans votre code, mais grâce à [`trouble.nvim`](https://github.com/folke/trouble.nvim) vous pourrez aller un cran plus loin : voir tous les soucis remonté par votre LSP dans tous vos fichiers, voir en un clin d'œil les définitions de fonctions de votre fichier, etc.

![Capture d'écran montrant Neovim avec trouble.nvim](/images/configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide/trouble.png "Capture d'écran montrant Neovim avec trouble.nvim")

Éditez `lua/plugins/trouble.lua` et placez-y le code suivant :

**`lua/plugins/trouble.lua`**

```lua
return {
  "folke/trouble.nvim",
  opts = {}, -- for default options, refer to the configuration section for custom setup.
  cmd = "Trouble",
  keys = {
    {
      "<leader>xx",
      "<cmd>Trouble diagnostics toggle<cr>",
      desc = "Diagnostics (Trouble)",
    },
    {
      "<leader>xX",
      "<cmd>Trouble diagnostics toggle filter.buf=0<cr>",
      desc = "Buffer Diagnostics (Trouble)",
    },
    {
      "<leader>cs",
      "<cmd>Trouble symbols toggle focus=false<cr>",
      desc = "Symbols (Trouble)",
    },
    {
      "<leader>cl",
      "<cmd>Trouble lsp toggle focus=false win.position=right<cr>",
      desc = "LSP Definitions / references / ... (Trouble)",
    },
    {
      "<leader>xL",
      "<cmd>Trouble loclist toggle<cr>",
      desc = "Location List (Trouble)",
    },
    {
      "<leader>xQ",
      "<cmd>Trouble qflist toggle<cr>",
      desc = "Quickfix List (Trouble)",
    },
  },
}
```

## Undefined global `vim`

Si comme moi vous avez installé le lsp `lua_ls` il va vous afficher un warning à chaque fois que vous éditez un fichier avec la variable globale `vim` dedans.

Pour faire disparaître cette erreur, configurez `lua_ls` dans `lua/plugins/lsp/mason.lua` pour y ajouter la variable globale `vim` :

```lua
        lua_ls = function()
          lspconfig.lua_ls.setup({
            settings = {
              Lua = {
                diagnostics = {
                  -- Force le LSP à reconnaître la variable globale `vim`
                  globals = { "vim" },
                },
              },
            },
          })
        end,
```

## Amélioration du formatage et formatage automatique : [`conform.nvim`](https://github.com/stevearc/conform.nvim)

Certains LSP proposent des options pour formatter automatiquement les fichiers à la sauvegarde mais ce n'est pas le cas pour tous et, quand ils le font, il le font généralement « mal » en remplaçant tout le contenu du buffer (ce qui va perdre vos folds par exemple). [`conform.nvim`](https://github.com/stevearc/conform.nvim) règle ce souci et permet en plus quelques configurations sympathiques comme le formattage automatique à la sauvegarde.

Éditez `lua/plugins/conform.lua` et placez-y le code suivant :

**`lua/plugins/conform.lua`**

```lua
return {
  "stevearc/conform.nvim",
  opts = {},
  event = { "BufReadPre", "BufNewFile" },
  config = function()
    local conform = require("conform")

    conform.setup({
      formatters_by_ft = {
        css = { "prettier" },
        elm = { "elm_format" },
        graphql = { "prettier" },
        json = { "prettier" },
        html = { "prettier" },
        liquid = { "prettier" },
        lua = { "stylua" },
        markdown = { "prettier" },
        python = { "ruff_fix", "ruff_format", "ruff_organize_import" },
        rust = { "rustfmt" },
        svelte = { "prettier" },
        javascript = { "prettier" },
        javascriptreact = { "prettier" },
        typescript = { "prettier" },
        typescriptreact = { "prettier" },
        yaml = { "prettier" },
      },
      format_on_save = {
        lsp_fallback = true,
        async = false,
        timeout_ms = 1000,
      },
    })

    vim.keymap.set({ "n", "v" }, "<leader>mp", function()
      conform.format({
        lsp_fallback = true,
        async = false,
        timeout_ms = 1000,
      })
    end, { desc = "Format file or range (in visual mode)" })
  end,
}
```

Encore une fois, c'est à configurer selon vos LSP. J'ai ajouté un raccourci qui permet de lancer le formatage via `<leader>mp`. Les formatteurs peuvent être installés directement sur votre système ou via `mason.nvim` comme nous allons le voir dans la section suivante.

## Installation automatique d'outils via [`mason-tools-installer.nvim`](https://github.com/WhoIsSethDaniel/mason-tool-installer.nvim)

Jusqu'ici nous avons utilisé `mason.nvim` pour installer des LSP. Mais comme nous l'avons vu c'est avant tout un gestionnaire de paquets et il est donc possible de l'utiliser pour installer d'autres choses que les LSP, notamment les formatteurs utilisés dans la section du dessus.

Pour ce faire éditez `lua/plugins/lsp/mason.lua` et ajoutez-y [`mason-tools-installer.nvim`](https://github.com/WhoIsSethDaniel/mason-tool-installer.nvim).

**`lua/plugins/lsp/mason.lua`**

```lua
return {
  "williamboman/mason.nvim",
  dependencies = {
    "williamboman/mason-lspconfig.nvim",
    "WhoIsSethDaniel/mason-tool-installer.nvim",
  },
  config = function()
    -- import de mason
    local mason = require("mason")

    -- import de mason-lspconfig
    local mason_lspconfig = require("mason-lspconfig")

    -- import de lspconfig
    local lspconfig = require("lspconfig")

    -- import de mason-tool-installer
    local mason_tool_installer = require("mason-tool-installer")

    -- active mason et personnalise les icônes
    mason.setup({
      ui = {
        icons = {
          package_installed = "✓",
          package_pending = "➜",
          package_uninstalled = "✗",
        },
      },
    })

    mason_tool_installer.setup({
      ensure_installed = {
        "elm-format", -- elm formater
        "prettier", -- prettier formatter
        "ruff", -- ruff formater (différent du LSP)
        "stylua", -- lua formater
      },
    })

    mason_lspconfig.setup({

-- … reste du fichier
```

## Amélioration du linting : [`nvim-lint`](https://github.com/mfussenegger/nvim-lint)

À l'instar de `conform.nvim`, [`nvim-lint`](https://github.com/mfussenegger/nvim-lint) va venir complémenter le linting fourni par les LSP. Il va vous permettre d'utiliser des outils de linting externes si celui de votre LSP n'est pas suffisant. Personnellement je n'en ai pas besoin car les LSP de Rust, Python et Elm fournissent tout ce qu'il faut. Mais si ce n'est pas votre cas, éditez `lua/plugins/nvim-lint.lua` et placez-y le code suivant :

**`lua/plugins/nvim-lint.lua`**

```lua
return {
  "mfussenegger/nvim-lint",
  event = { "BufReadPre", "BufNewFile" },
  config = function()
    local lint = require("lint")

    lint.linters_by_ft = {
      javascript = { "eslint_d" },
      typescript = { "eslint_d" },
      javascriptreact = { "eslint_d" },
      typescriptreact = { "eslint_d" },
      svelte = { "eslint_d" },
    }

    local lint_augroup = vim.api.nvim_create_augroup("lint", { clear = true })

    vim.api.nvim_create_autocmd({ "BufEnter", "BufWritePost", "InsertLeave" }, {
      group = lint_augroup,
      callback = function()
        lint.try_lint()
      end,
    })

    vim.keymap.set("n", "<leader>l", function()
      lint.try_lint()
    end, { desc = "Trigger linting for current file" })
  end,
}
```

N'oubliez pas de mettre à jour la liste des outils à installer automatiquement dans `lua/plugins/lsp/mason.lua` en ajoutant `eslint_d` par exemple :

```lua
    mason_tool_installer.setup({
      ensure_installed = {
        "elm-format", -- elm formater
        "prettier", -- prettier formatter
        "ruff", -- ruff formater (différent du LSP)
        "stylua", -- lua formater
        "eslint_d", -- eslint formater
      },
    })
```

## Intégration des différences Git : [`gitsigns.nvim`](https://github.com/lewis6991/gitsigns.nvim)

[`gitsigns.nvim`](https://github.com/lewis6991/gitsigns.nvim) va vous permettre d'afficher, dans la gouttière de gauche, les endroits où vous avez des ajouts ou des suppressions trackées avec Git.

**`lua/plugins/gitsigns.lua`**

```lua
return {
  "lewis6991/gitsigns.nvim",
  event = { "BufReadPre", "BufNewFile" },
  opts = {

    signs = {
      add = { text = "▎" },
      change = { text = "▎" },
      changedelete = { text = "▎" },
    },
    on_attach = function(bufnr)
      local gs = package.loaded.gitsigns

      local function map(mode, l, r, desc)
        vim.keymap.set(mode, l, r, { buffer = bufnr, desc = desc })
      end

      -- Navigation
      map("n", "]h", gs.next_hunk, "Next Hunk")
      map("n", "[h", gs.prev_hunk, "Prev Hunk")

      -- Actions
      map("n", "<leader>hs", gs.stage_hunk, "Stage hunk")
      map("n", "<leader>hr", gs.reset_hunk, "Reset hunk")
      map("v", "<leader>hs", function()
        gs.stage_hunk({ vim.fn.line("."), vim.fn.line("v") })
      end, "Stage hunk")
      map("v", "<leader>hr", function()
        gs.reset_hunk({ vim.fn.line("."), vim.fn.line("v") })
      end, "Reset hunk")

      map("n", "<leader>hS", gs.stage_buffer, "Stage buffer")
      map("n", "<leader>hR", gs.reset_buffer, "Reset buffer")

      map("n", "<leader>hu", gs.undo_stage_hunk, "Undo stage hunk")

      map("n", "<leader>hp", gs.preview_hunk, "Preview hunk")

      map("n", "<leader>hb", function()
        gs.blame_line({ full = true })
      end, "Blame line")
      map("n", "<leader>hB", gs.toggle_current_line_blame, "Toggle line blame")

      map("n", "<leader>hd", gs.diffthis, "Diff this")
      map("n", "<leader>hD", function()
        gs.diffthis("~")
      end, "Diff this ~")

      -- Text object
      map({ "o", "x" }, "ih", ":<C-U>Gitsigns select_hunk<CR>", "Gitsigns select hunk")
    end,
  },
}
```

## Améliorer les fenêtres de sélection et d'inputs : [`dressing.nvim`](https://github.com/stevearc/dressing.nvim)

Si vous ne savez pas pourquoi c'est une bonne idée, faites moi-confiance, ça en est une. Sinon, vous pouvez aussi allez voir la page de [`dressing.nvim`](https://github.com/stevearc/dressing.nvim) et comprendre le pourquoi du comment.

Éditez `lua/plugins/dressing.vim` et placez-y le code suivant :

**`lua/plugins/dressing.lua`**

```lua
return {
  "stevearc/dressing.nvim",
  event = "VeryLazy",
}
```

## Se souvenir de vos raccourcis : [`WhichKey`](https://github.com/folke/which-key.nvim)

[`WhichKey`](https://github.com/folke/which-key.nvim) est complètement incroyable pour se souvenir de vos raccourcis, notamment depuis la dernière version qui peut être lancée à la demande.

![Capture d'écran montrant Neovim avec whichkey](/images/configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide/whichkey.png "Capture d'écran montrant Neovim avec whichkey")

Éditez `lua/plugins/whichkey.lua` et placez-y le code suivant :

**`lua/plugins/whichkey.lua`**

```lua
return {
  "folke/which-key.nvim",
  event = "VeryLazy",
  opts = {},
  keys = {
    {
      "<leader>?",
      function()
        require("which-key").show({ global = true })
      end,
      desc = "Buffer Local Keymaps (which-key)",
    },
  },
}
```

Lorsque vous appuierez sur `<leader>?` il vous affichera tous les raccourcis possibles dans le contexte actuel. Ce plugin est juste complètement bluffant pour les personnes comme moi qui ont tendance oublier régulièrement les raccourcis qu'ils n'utilisent pas.

## Affichage des commentaires TODO, FIX, etc : [`todo-comments`](https://github.com/folke/todo-comments.nvim)

![Capture d'écran montrant Neovim avec todo-comments](/images/configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide/todo-comments.png "Capture d'écran montrant Neovim avec todo-comments")

Éditez `lua/plugins/todo-comments.lua` et placez-y le code suivant :

**`lua/plugins/todo-comments.lua`**

```lua
return {
  "folke/todo-comments.nvim",
  event = { "BufReadPre", "BufNewFile" },
  dependencies = { "nvim-lua/plenary.nvim" },
  config = function()
    local todo_comments = require("todo-comments")

    -- set keymaps
    local keymap = vim.keymap -- for conciseness

    keymap.set("n", "]t", function()
      todo_comments.jump_next()
    end, { desc = "Next todo comment" })

    keymap.set("n", "[t", function()
      todo_comments.jump_prev()
    end, { desc = "Previous todo comment" })

    todo_comments.setup()
  end,
}
```

## Ajout de guides d'indentation : [`indent-blankline`](https://github.com/lukas-reineke/indent-blankline.nvim)

**`lua/plugins/indent-blankline.lua`**

```lua
return {
  "lukas-reineke/indent-blankline.nvim",
  event = { "BufReadPre", "BufNewFile" },
  main = "ibl",
  opts = {
    indent = { char = "┊" },
  },
}
```

## Pairing des parenthèses automatique : [`nvim-autopairs`](https://github.com/windwp/nvim-autopairs)

**`lua/plugins/autopairs.lua`**

```lua
return {
  "windwp/nvim-autopairs",
  event = { "InsertEnter" },
  dependencies = {
    "hrsh7th/nvim-cmp",
  },
  config = function()
    -- import nvim-autopairs
    local autopairs = require("nvim-autopairs")

    -- configure autopairs
    autopairs.setup({
      check_ts = true, -- enable treesitter
      disable_filetype = { "TelescopePrompt" },
      ts_config = {
        lua = { "string" }, -- don't add pairs in lua string treesitter nodes
        javascript = { "template_string" }, -- don't add pairs in javscript template_string treesitter nodes
        java = false, -- don't check treesitter on java
      },
    })

    -- import nvim-autopairs completion functionality
    local cmp_autopairs = require("nvim-autopairs.completion.cmp")

    -- import nvim-cmp plugin (completions plugin)
    local cmp = require("cmp")

    -- make autopairs and completion work together
    cmp.event:on("confirm_done", cmp_autopairs.on_confirm_done())
  end,
}
```

## Parenthèses de couleurs différentes : [`rainbow-delimiters.nvim`](https://github.com/HiPhish/rainbow-delimiters.nvim)

**`lua/plugins/rainbow-delimiters.lua`**

```lua
return {
  "hiphish/rainbow-delimiters.nvim",
}
```

## Cerise sur le gâteau ou plugin de trop ? [`noice.nvim`](https://github.com/folke/noice.nvim) une interface repensée

[`noice.nvim`](https://github.com/folke/noice.nvim) est un plugin qui va changer l'affichage des erreurs et des notifications par défaut et va les mettre en notifications en haut à droite de votre _Neovim_. Il va aussi changer l'interface utilisateur pour la recherche ou encore les `cmdline` de _Neovim_ en vous affichant une popup au milieu de votre _Neovim_.

![Capture d'écran montrant Neovim avec noice](/images/configurer-neovim-comme-ide-a-partir-de-zero-tutoriel-guide/noice.png "Capture d'écran montrant Neovim avec noice")

**`lua/plugins/noice.lua`**

```lua
return {
  "folke/noice.nvim",
  event = "VeryLazy",
  opts = {
    -- add any options here
  },
  dependencies = {
    -- if you lazy-load any plugin below, make sure to add proper `module="..."` entries
    "MunifTanjim/nui.nvim",
    -- OPTIONAL:
    --   `nvim-notify` is only needed, if you want to use the notification view.
    --   If not available, we use `mini` as the fallback
    "rcarriga/nvim-notify",
  },

  config = function()
    local noice = require("noice")

    noice.setup({
      lsp = {
        -- override markdown rendering so that **cmp** and other plugins use **Treesitter**
        override = {
          ["vim.lsp.util.convert_input_to_markdown_lines"] = true,
          ["vim.lsp.util.stylize_markdown"] = true,
          ["cmp.entry.get_documentation"] = true, -- requires hrsh7th/nvim-cmp
        },
      },
      -- you can enable a preset for easier configuration
      presets = {
        bottom_search = true, -- use a classic bottom cmdline for search
        command_palette = true, -- position the cmdline and popupmenu together
        long_message_to_split = true, -- long messages will be sent to a split
        inc_rename = false, -- enables an input dialog for inc-rename.nvim
        lsp_doc_border = false, -- add a border to hover docs and signature help
      },
    })
  end,
}
```

## Conclusion

Nous avons fait le tour des principaux plugins que j'utilise pour faire de _Neovim_ mon IDE. N'hésitez pas à m'envoyer des retours sur [Mastodon vjousse@mamot.fr](https://mamot.fr/@vjousse) ou à contribuer directement à l'édition de ce tutoriel sur [Github](https://github.com/vjousse/emmett/blob/main/content/fr/tech/20240705-neovim-tutoriel-ide-a-partir-de-zero.md).

La [configuration complète est disponible sur Github](https://github.com/vjousse/neovim-from-scratch).

Et pour finir, si vous souhaitez diffuser la bonne parole au sujet de _Vim_ n'hésitez pas à télécharger et à partager mon livre [« Vim pour les humains »](https://vimebook.com).
