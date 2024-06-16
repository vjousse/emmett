---
title: "Comment préserver ses variables d'environnement et sa configuration vim avec sudo"
date: "2021-08-25 08:00:03+01:00"
slug: sudo-comment-preserver-ses-variables-d-environnement
tags: linux, terminal, astuce
---

Vous en conviendrez, il n'y a rien de plus énervant que de faire un `sudo vim` et de se retrouver sans sa configuration `vim` préférée car votre utilisateur root n'a pas de configuration pour vim.

Ou alors de voir vos alias ne pas fonctionner lorsque vous faites un `sudo`. Il y a peut-être plus énervant (comme de taper `sl` à la place de `ls` par exemple), mais ce truc est dans le top 5 à coup sûr.

Ne vous inquiétez pas, j'ai la solution.

<!-- TEASER_END -->

Placez l'alias suivant dans la configuration de votre shell (`~/.bashrc`, `~/.zshrc`, …) :

```bash
alias _='sudo -E '
```

Notez bien l'espace après le `-E` il est indispensable pour préserver l'interprétation des alias que votre commande pourrait contenir (cf. [la documentation de bash](http://www.gnu.org/savannah-checkouts/gnu/bash/manual/bash.html#Aliases)).

Ensuite, au lieu de faire un `sudo` comme d'habitude, préfixez juste votre commande par `_` à la place :

```bash
_ vim
```

Magie, votre configuration vim a été gardée et vim est bien lancé en root ! 🎉
