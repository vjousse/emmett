---
title: "Configurer Neovim comme IDE/éditeur de code à partir de zéro"
date: "2024-07-05 09:33:20+01:00"
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

## Gestionnaire de plugins : [`lazy.nvim`](https://lazy.folke.io/)

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

## Barre de statut dopée au stéroïdes : [`lualine`](https://github.com/nvim-lualine/lualine.nvim)

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

## Amélioration des fenêtres de sélection et d'inputs : [`dressing.nvim`](https://github.com/stevearc/dressing.nvim)

Si vous ne savez pas pourquoi c'est une bonne idée, faites moi-confiance, ça en est une. Sinon, vous pouvez aussi allez voir la page de [`dressing.nvim`](https://github.com/stevearc/dressing.nvim) et comprendre le pourquoi du comment.

Éditez `lua/plugins/dressing.vim` et placez-y le code suivant :

**`lua/plugins/dressing.lua`**

```lua
return {
  "stevearc/dressing.nvim",
  event = "VeryLazy",
}
```

## Installation de [`nvim-treesitter`](https://github.com/nvim-treesitter/nvim-treesitter)

[tree-sitter](https://tree-sitter.github.io/tree-sitter/) est un outil incroyable (non spécifique à _Neovim_) qui va permettre de parser et de « comprendre » la syntaxe d'un grand nombre de langages de programmation. Son intégration dans _Neovim_ à l'aide de [`nvim-treesitter`](https://github.com/nvim-treesitter/nvim-treesitter) va permettre une meilleure coloration syntaxique, de l'indentation plus intelligente, des tags automatiques, des sélections intelligentes en fonction du langage de programmation et du contexte, j'en passe et des meilleures. Bref, c'est un plugin indispensable.

Éditez `lua/plugins/treesitter.lua` :

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
