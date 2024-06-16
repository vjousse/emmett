---
title: "FastAPI : Le meilleur framework web Python en 2021"
slug: le-meilleur-framework-web-python
date: 2021-06-11 08:30:55+01:00
tags: python, framework, fastapi, web
---

Oui ce titre est racoleur, oui il fait preuve d'une opinion totalement biaisée. Mais bordel, quand quelque chose est **bien**, il faut le dire ! Quitte à utiliser de falacieux moyens dignes du meilleur _growth hacker_, à savoir, je vous le donne en mille : le titre putaclick.

Passée cette intro de qualité, venons-en aux faits : [FastAPI](https://fastapi.tiangolo.com/) est le meilleur framework Web Python que j'ai pu utiliser depuis de nombreuses années, loin devant **Django**, **DRF**, **Flask** ou autres **Pyramid**.

<!-- TEASER_END -->

## Sur les épaules des géants

On pourrait se dire, mais pourquoi encore un framework de plus ? C'est justement ce que j'aime chez FastAPI, ce n'est **pas juste un Framework de plus**. Sa conception a été **réfléchie**, **documentée**, et **est construit sur l'existant**. Il y a même [une page complète qui explique la démarche](https://fastapi.tiangolo.com/alternatives/) et ce qui a été pris des différents frameworks existants et pourquoi.

> FastAPI wouldn't exist if not for the previous work of others.
>
> There have been many tools created before that have helped inspire its creation.
>
> I have been avoiding the creation of a new framework for several years. First I tried to solve all the features covered by FastAPI using many different frameworks, plug-ins, and tools.
>
> But at some point, there was no other option than creating something that provided all these features, **taking the best ideas from previous tools**, and combining them in the best way possible, using **language features that weren't even available before** (Python 3.6+ type hints).

Et pour chaque framework/outil il y a une explication de ce qui a été gardé pour FastAPI.

![Inspiration FastAPI DRF](/images/le-meilleur-framework-web-python/fastapi_drf_inspiration.png)

C'est pour moi un **point très important** : c'est un Framework qui a été **pensé**, **réfléchi** et qui est **construit sur ce qui existe déjà**.

## À la pointe des évolutions de Python

Qu'on aime ou pas Python a, ces dernières années, intégré deux évolutions majeures :

- La **programmation asynchrone** avec _asyncio_
- Un début de **typage** avec les _typehints_

Pour faire simple, la programmation asynchrone va permettre à des programmes Python de faire **plus de choses en même temps**, sans être obligé d'attendre que certaines tâches soient finies pour passer à d'autres. Vous l'avez naturellement dans Node.JS, Go, Rust (plus récemment) et donc maintenant Python. Pour plus d'explications, n'hésitez pas à aller lire [cet article sur Zeste de savoir](https://zestedesavoir.com/articles/1568/decouvrons-la-programmation-asynchrone-en-python/).

Les _typehints_ quant à eux, vont **aider le compilateur** et donc vous, par la même occasion, à savoir ce que vous voulez faire de votre programme. Ci-dessous par exemple, on spécifie que la fonction `greeting` doit prendre une chaîne de caractères (`str`) en paramètre. Si vous lui donnez ce qu'il sait être un `int`, le compilateur vous en informera.

```python
def greeting(name: str) -> str:
    return 'Hello ' + name
```

PHP a fait cette mutation il y a quelques années, javascript/typescript aussi, et c'est donc maintenant aussi le cas de Python depuis la version 3.6. On pourrait débattre des heures sur le typage des langages de programmation dynamiques, mais ce n'est pas le sujet ici.

Le sujet est que **FastAPI l'utilise partout**, l'encourage [dans sa documentation](https://fastapi.tiangolo.com/python-types/) et par l'utilisation de [Pydantic](https://pydantic-docs.helpmanual.io/) pour valider vos données.

Cela va permettre :

- À votre éditeur de code de vous afficher la **complétion des méthodes**

![Typehints python complétion](/images/le-meilleur-framework-web-python/typehints_python.png)

- À votre éditeur de code de vous **afficher des erreurs utiles**

![Typehints python exemple 2](/images/le-meilleur-framework-web-python/typehints_python2.png)

- À **vos modèles d'être validés automatiquement**, c'est à dire de savoir si les données que vous lui soumettez sont valides ou non, juste en utilisant la déclaration des types
- Générer **automatiquement la documentation OpenAPI** de votre API

[Documentation officielle python](https://docs.python.org/3/library/typing.html)

[Cheat sheet mypy](https://mypy.readthedocs.io/en/latest/cheat_sheet_py3.html)

## Orienté API, mais pas que

De plus en plus, dès que vous avez une application web (avec un framework javascript en front) ou mobile à faire, tout ce dont vous avez besoin, c'est une **API Json**. FastAPI est **parfait** pour ça. Vraiment. Beaucoup plus simple que DRF, pas besoin d'apprendre je ne sais quelle syntaxe pour transformer vos objets en JSON, tout est simple et utilisable tel quel (notamment grâce à _Pydantic_ et les _typehints_).

```python
from typing import Optional

from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()

class Item(BaseModel):
    name: str
    price: float
    is_offer: Optional[bool] = None

@app.get("/")
def read_root():
    return {"Hello": "World"}

@app.get("/items/{item_id}")
def read_item(item_id: int, q: Optional[str] = None):
    return {"item_id": item_id, "q": q}

@app.put("/items/{item_id}")
def update_item(item_id: int, item: Item):
    return {"item_name": item.name, "item_id": item_id}
```

Et avec ça vous avez en plus une documentation auto-générée.

![FastAPI swagger](/images/le-meilleur-framework-web-python/fastapi_swagger.png)

Je ne vais pas rentrer dans les détails, vous pourrez les trouver sur [le site web de FastAPI](https://fastapi.tiangolo.com/). Mais vraiment, si vous avez une **API Json à faire, ne cherchez pas plus loin**.

Et si je dois faire un **site internet « classique »** ? Et bien ça se fait très bien aussi. J'ai personnellement réalisé un site avec :

- Pour **écrire mon HTML** : [Jinja](https://fastapi.tiangolo.com/advanced/templates/) comme moteur de template (le même que Django)
- Pour **gérer mes formulaires** : le classique [WTForms](https://wtforms.readthedocs.io/en/2.3.x/) beaucoup utilisé avec Flask et notamment son intégration avec [starlette](https://www.starlette.io/) (sur lequel est basé FastAPI), nommée [starlette-wtf](https://github.com/muicss/starlette-wtf).
- Pour la **gestion des utilisateurs** : [fastapi-users](https://github.com/frankie567/fastapi-users)
- Pour **la base de données** : [Tortoise ORM](https://tortoise-orm.readthedocs.io/en/latest/). C'est moins le bazar que SQLAlchemy (totalement subjectif), c'est asynchrone par défaut et ça marche très bien avec `fastaupi-users`.
- Pour **les migrations** : [Aerich](https://github.com/tortoise/aerich). C'est fait par les développeurs de Tortoise et ça fait très bien le taf.

## Simple, efficace et plein de fonctionnalités

FastAPI combine pour moi le meilleur de tous les mondes :

- **Léger** comme _Flask_
- **Rapide** comme _Go_ ou _Node.js_
- **Fait pour les API** comme _DRF_
- Parfait pour les **Websockets**
- **Typé** pour se faire aider par le compilateur
- Basé sur les **dernières avancées de Python**
- **Facile** à prendre en main

Voici une liste à la Prévert de ses fonctionnalités issues du [site web de FastAPI](https://fastapi.tiangolo.com/features/):

- Respecte des **standards ouverts** : [OpenAPI](https://github.com/OAI/OpenAPI-Specification) & [JSON Schema](https://json-schema.org/)
- **Génère automatiquement la documentation** et permet l'exploration via des interfaces web avec [Swagger UI](https://github.com/swagger-api/swagger-ui) et [ReDoc](https://github.com/Rebilly/ReDoc)
- Se base sur du **Python 3.6+ standard**, sans autre nouvelle syntaxe à apprendre
- Permet de la **complétion de code** (grâce aux types) dans les éditeurs classiques comme [Visual Studio Code](https://code.visualstudio.com/) ou [PyCharm](https://www.jetbrains.com/pycharm/)
- **Fonctionne par défaut** mais peut être entièrement configuré de manière optionnelle
- A des fonctionnalités de **validation** pour les types Python par défaut, mais aussi pour des types plus exotiques comme _URL_, _email_, _UUID_, etc.
- Intègre des **mécanismes de sécurité par défaut** : HTTP Basic, **OAuth2 & JWT Tokens**, **cookies de session**, clés d'API supportées dans les Headers, les paramètres de requêtes et les cookies
- Dispose d'un mécanisme d'**injection de dépendance** très facile à utiliser qui va vous permettre de disposer des objets dont vous avez besoin dans vos méthodes très facilement (comme la connexion à la base de données). Et je confirme, ça fait du bien cette simplicité après avoir du subir^W^W^W^Wgérer ça en _Symfony/PHP_ :sweat_smile:
- Ne nécessite **pas de plugins** en particulier, et pour cause : il suffit just d'importer du code Python et de l'utiliser tel quel
- **Testé** : couverture de test de 100%, base de code annotée avec des types à 100%, utilisé en production

FastAPI étant [basé sur Starlette](https://www.starlette.io/), il dispose aussi de toutes ses fonctionnalités :

- **Performances impressionnantes**. Un des [frameworks python le plus rapide, faisant jeu égal avec **NodeJS** et **Go**](https://github.com/encode/starlette#performance).
- Support des **Websocket**
- Support de **GraphQL**
- Exécution de **tâches de fond**
- Évènements de startup/shutdown
- Client de test basé sur [`requests`](https://docs.python-requests.org/en/master/)
- **CORS**, GZip, fichiers statiques et réponses en streaming
- Support des **sessions et des cookies**

## Conclusion

Clairement, pour moi, si je devais commencer un projet **web de type API en python**, choisir **FastAPI me paraitrait être une évidence**. Il est **moderne**, **rapide** et développé par quelqu'un qui **réfléchit** avant de coder.

Si je devais commencer un **projet web plus classique** avec besoin de templating, de formulaires, etc. je partirais **aussi sur FastAPI**. Il permet tout ça très bien avec une touche de **modernité et de simplicité** qui est la bienvenue.

Si en revanche vous avez absolument **besoin de l'admin Django**, partir avec **FastAPI n'est pas forcément une bonne idée**. Même s'il existe une libraire [d'administration FastAPI](https://fastapi-admin-docs.long2ice.cn/) elle est assez récente et n'a rien à voir avec celle de Django. Dans ce cas là ça vaut quand même le coup de se poser la question : est-ce que vous avez vraiment besoin de l'admin ? Il est des fois plus rapide de re-développer soi-même que de passer du temps à configurer un truc déjà tout prêt.

Quoiqu'il en soit, FastAPI est le framework Python qui m'a le plus impressionné depuis des années. Pour moi, **l'essayer, c'est l'adopter** ! Pour commencer, vous pouvez [suivre le tutorial du site web officiel](https://fastapi.tiangolo.com/tutorial/).

Vous pouvez aussi consulter le guide complet que je suis en train d'écrire à ce sujet :

- [Le guide complet du débutant avec FastAPI - Partie 1 : installation et premier programme](http://localhost:1313/articles/le-guide-complet-du-debutant-avec-fastapi-partie-1/)

Stay tuned pour la suite ! 🎉
