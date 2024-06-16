---
title: Comment cloner un dépôt Git privé sur un serveur sans clé ssh locale
date: 2021-05-12 14:40:13+01:00
slug: comment-cloner-un-depot-git-prive-sur-un-serveur-sans-cle-ssh
tags: git, ssh, astuce
---

Vous vous connectez en SSH sur un serveur distant mais vous ne **pouvez pas cloner un de vos dépôt Git** car Git vous dit que vous n'avez pas les droits d'accès et pourtant, vous avez les droits en local sur votre machine.

<!-- TEASER_END -->

## Le problème

```bash
$ git clone git@github.com:pereprogramming/blog.git
Cloning into 'blog'...
Warning: Permanently added the RSA host key for IP address '140.82.121.3' to the list of known hosts.
git@github.com: Permission denied (publickey).
fatal: Could not read from remote repository.

Please make sure you have the correct access rights
and the repository exists.
```

Et pour cause : lorsque vous êtes sur un serveur en SSH, **vos clés SSH locales ne sont pas transférées** sur le serveur distant et vous n'héritez pas des droits que vous avez en local.

Vous pourriez être tenté de copier vos clés SSH locales sur le serveur mais je vous le déconseille. D'un point du vue sécurité, vos clés SSH sont mieux en local sur votre machine plutôt que je ne sais sur quel serveur distant.

## La solution

Heureusement, SSH a déjà tout ce qu'il faut. Pour transférer vos clés SSH locales en session sur votre serveur distant, vous avez juste à utiliser **l'option `-A`** dans votre commande SSH :

```bash
ssh -A user@machine.com
```

Et le tour est joué. **Vos clées SSH locales ne seront pas copiées sur le serveur mais pourtant, vous aurez les accès qui y correspondent**. Pour vérifier que tout est ok, lorsque vous être en SSH sur le serveur, faites un :

```bash
env | grep SSH_AUTH
```

Vous devriez voir une ligne de ce genre :

```bash
SSH_AUTH_SOCK=/tmp/ssh-h89iLqNaiw/agent.6994
```

Si c'est le cas, c'est que c'est gagné, sinon, n'hésitez pas [à me contacter sur Twitter](https://twitter.com/pereprogramming) pour en discuter.

Amusez-vous bien ! 🎉
