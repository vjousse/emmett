---
title: "Le guide complet du débutant avec FastAPI - Partie 1: installation et premier programme"
date: 2021-06-11 19:33:20+01:00
slug: le-guide-complet-du-debutant-avec-fastapi-partie-1
tags: python, framework, fastapi, web, fastapi-tutorial
---

_**Mise à jour le 26/01/2024**_

L'intégralité du guide est [disponible ici](/blog/fr/tags/tech/fastapi-tutorial).

## Pourquoi FastAPI ?

Commençons par le commencement.

FastAPI est un framework de **développement Web** écrit en Python. Il est très polyvalent et va permettre de développer :

- Des **sites internet « classiques »** avec juste du contenu
- Des **sites dynamiques** avec formulaires et gestion des utilisateurs
- La partie « cachée » des **applications mobiles iOS ou Android**, que l'on appelle une **API**. Vous entendrez aussi parler de **backend**, ce n'est pas tout à fait la même chose, mais on va faire comme-ci pour l'instant.
- Des applications en **temps réel** comme peuvent l'être les applications de discussion, les cours de la bourse, etc. Vous entendrez souvent parler de **WebSocket** pour ces applications, on y reviendra.
- Les interfaces d'accès des applications de **Machine Learning** et d'**Intelligence Artificielle** qui sont majoritairement développées en Python.

L'idée d'utiliser un framework va être de **ne pas réinventer la roue** et de se baser sur ce que d'autres personnes talentueuses ont fait avant nous pour résoudre les problèmes classiques des applications web : gestion de la base de données, des URL, de la sécurité des formulaires, des sessions, etc.

FastAPI est un framework assez **récent** puisque sa première version date de **décembre 2018**. C'est d'ailleurs pour cela que je le choisis maintenant pour la majorité de mes projets : il se base sur une **version récente de Python (minimum 3.6)** et en tire tous les bénéfices que nous verrons un peu plus tard (simplicité et rapidité notamment).

Il est entre autres utilisé dans de **grosses entreprises** comme [**Microsoft**](https://github.com/tiangolo/fastapi/pull/26#issuecomment-463768795), [**Uber**](https://eng.uber.com/ludwig-v0-2/) ou encore [**Netflix**](https://netflixtechblog.com/introducing-dispatch-da4b8a2a8072).

Et pourquoi pas **Django** ou **framework X** ? Tout simplement pour les raisons citées ci-dessus. **FastAPI** est pour moi le [Framework d'avenir en Python](/articles/le-meilleur-framework-web-python/). Donc quitte à apprendre quelque chose, autant apprendre quelque chose sur lequel vous pourrez **capitaliser pour votre futur** : FastAPI est parfait pour ça.

## Installation

### Installer Python 3.6+

Il va vous falloir une version récente de Python, a minima la 3.6. La manière de l'installer dépendra de votre système d'exploitation en fonction de si vous êtes sous Windows, Linux ou Mac OS X. Je ne vais pas rentrer dans les détails ici, mais l'idée est d'aller sur le [site officiel de Python](https://www.python.org/download/) et de télécharger puis installer la version la plus récente. À la date d'écriture de cet article c'est la version 3.12.1.

Il faudra que, dans votre terminal ou invite de commandes, la version de Python affichée avec la commande `python --version` soit celle que vous avez téléchargée. Dans mon cas :

```
$ python --version
Python 3.11.5
```

> **Attention** : dans tous les exemples de code comme celui ci-dessus, les commandes commenceront toujours par `$` et le résultat sera le contenu des lignes en dessous.
>
> Dans les exemples qui seront donnés, ne tapez donc pas le `$` mais uniquement ce qui se trouve après, dans notre cas `python --version`

### Installer `pip` et `virtualenv`

Dans tout projet, vous allez avoir besoin de dépendances : des programmes ou des librairies développées par d'autres et mises à votre disposition. En Python, ces dépendances sont généralement gérées grâce à un logiciel nommé [`pip`](https://pip.pypa.io/en/stable/).

Tout d'abord, vérifiez si vous avez `pip` d'installé :

```
$ python -m pip --version
pip 23.3.2 from /home/vjousse/.asdf/installs/python/3.11.5/lib/python3.11/site-packages/pip (python 3.11)
```

Si vous l'avez, il vous affichera un numéro de version comme dans l'exemple ci-dessus (le numéro de version en lui-même n'est pas très important). Si ce n'est pas le cas, téléchargez le fichier [`get-pip.py`](https://bootstrap.pypa.io/get-pip.py) et installez-le via Python :

```
$ sudo python get-pip.py
```

Maintenant que vous avez `pip` nous allons pouvoir créer un `virtualenv`. Un `virtualenv` va permettre d'isoler votre projet et ses dépendances au sein d'un environnement dédié, séparé de votre environnement système. Cela va avoir l'avantage de **ne pas créer de conflits** avec les versions et les dépendances Python de votre système, mais aura aussi l'avantage de rendre votre projet facilement utilisable/installable sur une autre machine.

Il est maintenant temps de créer notre premier environnement virtuel et notre premier projet. Créez un répertoire pour votre projet puis déplacez-vous dedans. Créez ensuite votre environnement virtuel avec la commande ci-dessous.

```
$ python -m venv venv
```

Une fois votre environnement virtuel créé, il faut que vous l'activiez grâce à la commande suivante :

```
$ source ./venv/bin/activate
```

Cela devrait vous ajouter `(venv)` au début de votre ligne de commandes, cf ci-dessous.

![Activation du virtualenv](/images/le-guide-complet-du-debutant-avec-fastapi-partie-1/virtualenv_activation.png)

> **Attention** : vous devrez l'activer à chaque fois que vous ouvrez un nouveau terminal. Pour pouvoir exécuter les commandes dont nous parlerons plus tard, vous **devez** avoir le `(venv)` qui s'affiche au début de votre ligne.

Les commandes ci-dessus ont créé un répertoire nommé `venv` dans votre répertoire courant. Ce répertoire contient une **copie locale de Python et `pip`**. Toutes les dépendances que vous installerez via `pip` seront installées dans ce répertoire à côté de votre copie locale de Python. Cela évitera de venir perturber l'installation globale de Python sur votre système.

Vous pouvez désactiver l'environnement virtuel en fermant votre terminal ou en utilisant la commande ci-dessous :

```
$ deactivate
```

Gardons-le activé pour l'instant.

### Installer FastAPI

Les choses sérieuses commencent. Assurez vous que vous avez bien activé votre virtualenv (notez le `(venv)` avant votre ligne de commande) et tapez la commande suivante :

```
(venv) $ pip install fastapi[all]
```

Et voilà ! FastAPI et toutes ses dépendances (le `[all]`] sont installés. Il ne nous reste plus qu'à créer notre premier programme de test pour s'assurer que tout fonctionne bien.

## Hello World

### Notre premier programme

Le _Hello World_ est souvent le premier programme que l'on réalise lorsque l'on teste un langage/framework, le but étant juste d'afficher _Hello World_ à l'écran. Créez un fichier `main.py` et mettez-y le contenu ci-dessous :

```python
# main.py
from fastapi import FastAPI

app = FastAPI()


@app.get("/")
async def root():
    return {"message": "Hello World"}
```

Pour pouvoir éxécuter le code ci-dessus et afficher le message _Hello World_ dans une fenêtre de votre navigateur, il va falloir pour ce faire lancer un **serveur Web**. Le serveur Web est ce qui va faire l'interface entre votre navigateur et votre programme FastAPI. On dit qu'il va recevoir les **_requêtes HTTP_** envoyées par votre navigateur (notamment lorsque vous entrez une adresse dans la barre d'adresse) et qu'il va les transférer à FastAPI. FastAPI va ensuite être en charge de construire la **_réponse HTTP_** qui sera envoyée en retour au navigateur. Votre navigateur affichera finalement le contenue de cette **_réponse HTTP_**.

Dans notre cas, nous allons utiliser un serveur web nommé `uvicorn`, parfaitement adapté pour FastAPI.

```
(venv) $ uvicorn main:app --reload
```

Voici ce que vous devriez voir s'afficher dans votre terminal :

```
INFO:     Uvicorn running on http://127.0.0.1:8000 (Press CTRL+C to quit)
INFO:     Started reloader process [3173] using watchgod
INFO:     Started server process [3175]
INFO:     Waiting for application startup.
INFO:     Application startup complete.
```

Vous pouvez maintenant vous rendre à l'adresse [http://127.0.0.1:8000](http://127.0.0.1:8000) et voir s'afficher :

```json
{ "message": "Hello World" }
```

![Hello World](/images/le-guide-complet-du-debutant-avec-fastapi-partie-1/hello_world.png)

Si c'est le cas, vous venez d'exécuter votre première application web écrite avec FastAPI. Félicitations ! 🎉

### Explications pas à pas

Prenons le temps de décortiquer ce petit bout de code Python.

- Import de la classe `FastAPI` du package `fastapi` installé plus haut avec `pip`

```python
from fastapi import FastAPI
```

- Création de l'objet `FastAPI` que l'on stocke sous le nom `app`

```python
app = FastAPI()
```

- Paramétrage de la fonction nommée `root` via la ligne `@app.get("/")` (on parle de [décorateur](https://python.doctor/page-decorateurs-decorator-python-cours-debutants) en Python). Ce paramétrage permet de dire : _lorsque mon application FastAPI reçoit une requête pour l'adresse `/` et la méthode HTTP `get`, exécuter la fonction `root()` définie en dessous_. Nous verrons un peu plus tard ce que sont les méthodes HTTP, ne vous inquiétez pas.

```python
@app.get("/")
async def root():
```

- Pour finir, nous retournons un dictionnaire Python classique `{"message": "Hello World"}`. FastAPI se chargera de transformer, par défaut, ce dictionnaire en objet et en réponse JSON.

```python
    return {"message": "Hello World"}
```

Lorsque l'on lance ce fichier avec la commande `uvicorn` suivante :

```
uvicorn main:app --reload
```

On précise donc à `uvicorn` d'aller chercher dans le fichier `main.py` (la première partie de `main:app`) l'objet nommé `app` (qui est, comme nous venons de le voir notre application FastAPI) et de rediriger toutes les requêtes qu'il reçoit vers cet objet. Le `--reload` permet juste à `uvicorn` de recharger automatiquement l'application lorsqu'il détecte un changement dans le code.

## Conclusion

Nous venons de voir l'importance des environnements virtuels, comment installer FastAPI et venons de décortiquer notre premier programme FastAPI.

La prochaine étape consistera à voir les fondamentaux en développant un exemple d'application de A à Z. Stay tuned !

N.B. : J'ai mis en ligne [le code sur Github](https://github.com/vjousse/fastapi-beginners-guide/tree/part1)

Pour la partie 2, c'est par ici : [templates html, base de données et documentation](/blog/fr/tech/le-guide-complet-du-debutant-avec-fastapi-partie-2/).
