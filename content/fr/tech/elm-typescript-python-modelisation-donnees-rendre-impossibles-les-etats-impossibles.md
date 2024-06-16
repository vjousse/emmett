---
title: "Python, Typescript, Elm et modélisation de données : rendre impossibles les états impossibles"
date: "2022-06-02 08:00:03+01:00"
slug: elm-typescript-python-modelisation-donnees-rendre-impossibles-les-etats-impossibles
tags: elm, python, typescript
---

_Cet article est fortement inspiré de la vidéo de [Richard Feldman - Making Impossible States Impossible](https://www.youtube.com/watch?v=IcgmSRJHu_8)_

Quand on a le choix entre :

- Vérifier que notre modèle est bien dans un état cohérent quand on le met à jour
- Rendre impossible les états incohérents via la modélisation elle-même, ça empêchera de devoir le faire via le code / la logique

On devrait toujours privilégier la deuxième solution. Comme disait ma grand-mère « mieux vaut prévenir que guérir ! » 👵.

Cet article va vous donner quelques exemples en _Elm_ et en _Python_ sur comment modéliser au mieux vos données pour ne pas rendre possible l'impossible.

<!-- TEASER_END -->

## Le problème

J'ai des fois tendance à me retrouver avec une modélisation de données qui peut être dans des états qui devraient être impossibles.

Par exemple, prenons une liste de questions (représentée par une liste de strings), puis une liste de réponses (représentée par une liste de strings ou d'absence de valeur) associées à ces questions.

Imaginons en _Python_ ça pourrait donner ça :

```python
questions: list[str] = ["question 1", "question 2", "question 3"]
responses: list[str|None] = ["response 1", "response 2", None]
```

> Note: si vous n'avez pas l'habitude des _type annotations_ en Python, `List[str|None]` signifie _une liste contenant des strings ou des None_. Le `|` a été ajouté avec Python 3.10, avant Python 3.10 vous pouvez obtenir la même chose avec `Union[str, None]`

Et en _Elm_ ça :

```elm
{ questions : List String
, responses : List (Maybe String)
}

{ questions =
    [ "question 1",
    , "question 2"
    , "question 3"
    ]
, responses =
    [ Just "response 1"
    , Just "response 2"
    , Nothing
    ]
}
```

Enfin, en Typescript :

```typescript
const questions: [string] = ["question 1", "question 2", "question 3"];
const responses: [string | null] = ["response 1", "response 2", null];
```

Le souci ici, c'est que rien dans notre modélisation ne nous empêche d'avoir des réponses sans questions.

_Python_

```python
questions: list[str] = []
responses: list[str|None] = ["response 1", "response 2", None]
```

_Elm_

```elm
{ questions = []
, responses =
    [ Just "response 1"
    , Just "response 2"
    , Nothing
    ]
}
```

_Typescript_

```typescript
const questions: [string] = [];
const responses: [string | null] = ["response 1", "response 2", null];
```

Ça sent le bug à plein nez non ? Qu'est-ce que notre application est censée faire de ça ? Vous allez me dire « oui mais bon, je ferai attention quand je mettrai à jour mes questions de bien mettre à jour mes réponses aussi en fonction ». Lorsque votre cerveau vous propose ce type de solution, voici la bonne posture à adopter :

![Gandalf : fuyez pauvres fous](/images/elm-typescript-python-modelisation-donnees-rendre-impossibles-les-etats-impossibles/fuyez_pautres_fous.jpg "Gandalf : fuyez pauvres fous")

Forcément, vous allez oublier de mettre à jour. Forcément, un jour, un truc ne se passera pas comme prévu. Le mainteneur du projet ça ne sera plus vous et la personne qui prendra votre relève fera la bêtise à votre place.

En programmation, j'ai fini par apprendre que plus on part du fait qu'on fera des conneries, plus la qualité de notre programme augmente.

## Rendre impossibles les états impossibles

Comment pourrions-nous changer notre modélisation pour que, quoiqu'il se passe, ces incohérences ne puissent pas arriver ?

Rien de plus simple, il nous suffirait d'avoir une classe `Question` qui pourrait modéliser ce qu'est une question : un libellé et une possible réponse.

_Python_

```python
from dataclasses import dataclass


@dataclass
class Question:
    prompt: str
    response: str | None


questions: list[Question] = [
    Question(prompt="question 1", response="response 1"),
    Question(prompt="question 2", response="response 2"),
    Question(prompt="question 3", response=None),
]
```

_Elm_

```elm
type alias Question =
    { prompt : String
    , response : Maybe String
    }


questions =
    [ { prompt = "question 1", response = "response 1" }
    , { prompt = "question 2", response = "response 2" }
    , { prompt = "question 3", response = Nothing }
    ]
```

_Typescript_

```typescript
type Question = {
  prompt: string;
  response: string | null;
};
const questions: [Question] = [
  { prompt: "question 1", response: "response 1" },
  { prompt: "question 2", response: "response 2" },
  { prompt: "question 3", response: null },
];
```

La modélisation de nos données rend maintenant impossible le fait d'avoir une question sans réponse !

Cet exemple est assez simple mais vous comprenez le principe : à chaque fois qu'on modélise quelque chose, il est bon de se poser la question si notre modélisation permet, ou pas, des états qui devraient être impossibles.

## Bonus : modéliser un historique

Essayons d'aller un peu plus loin dans notre modélisation. Imaginons maintenant que nous voulions modéliser un historique de questions. On aimerait connaître quelle est la question actuelle, quelles sont les questions passées et quelles sont les questions à venir.

On pourrait imaginer quelque chose comme cela :

_Python_

```python
from dataclasses import dataclass


@dataclass
class Question:
    prompt: str
    response: str | None


@dataclass
class History:
    questions: list[Question]
    current: Question


questions: list[Question] = [
    Question(prompt="question 1", response="response 1"),
    Question(prompt="question 2", response="response 2"),
    Question(prompt="question 3", response=None),
]

history: History = History(
    questions=questions, current=Question(prompt="question 1", response="response 1")
)
```

_Elm_

```elm
type alias History =
    { questions : List Question
    , current : Question
    }

-- Rest of the code

{ questions = [question1, question2, question3]
, current = question1
}
```

_Typescript_

```typescript
type History = {
    questions: [Question],
    current : Question,
}

-- Rest of the code

{
    questions: [question1, question2, question3],
    current: question1,
}
```

Le problème ici, c'est que rien ne nous empêche d'avoir ce type d'état :

_Python_

```python
history: History = History(questions=[], current=Question(prompt="question 1", response="response 1"))
```

_Elm_

```elm
{ questions = []
, current = question1
}
```

_Typescript_

```typescript
{
    questions = [],
    current = question1,
}
```

Et vous en conviendrez, avoir une question courante qui n'est pas dans la liste des questions possibles est un problème assez fâcheux… Commençons par empêcher le fait d'avoir zéro question via notre modèle. Là normalement vous devriez me dire, « mais comment c'est possible » ? En effet, une liste, que ça soit en Python, en Elm ou en ce que vous voulez, rien ne l'empêche d'être vide !

Nous allons utiliser un idiome assez courant en programmation fonctionnelle, nous allons considérer qu'une liste est en fait composée de son premier élément, puis du reste de la liste. Voici ce que ça donnerait :

_Python_

```python
@dataclass
class History:
    first: Question
    other_questions: list[Question]
    current: Question
```

_Elm_

```elm
type alias History =
    { first : Question,
    , otherQuestions : List Question
    , current : Question
    }
```

_Typescript_

```typescript
type History = {
  first: Question;
  otherQuestions: [Question];
  current: Question;
};
```

Bon c'est mieux car on ne peut plus avoir de liste vide. MAIS ⚠️ (car évidemment il y a un mais), ça ne nous empêche toujours pas d'avoir une question courante qui ne fait pas partie des questions possibles.

Ce qui donnerait ça par exemple en python :

_Python_

```python
other_questions: list[Question] = [
    Question(prompt="question 2", response="response 2"),
    Question(prompt="question 3", response=None),
]

history: History = History(
    first=Question(prompt="question 1", response="response 1"),
    other_questions=other_questions,
    current=Question(prompt="unknown question", response="unknown response"),
)
```

Et quelque chose comme ça en Elm :

_Elm_

```elm
{ first: question1
  otherQuestions = [question2, question3]
, current = unknown_question
}
```

Enfin en Typescript :

_Typescript_

```typescript
type History = {
  first: Question;
  otherQuestions: [Question];
  current: Question;
};
```

Pour pallier à ce problème, nous allons utiliser la modélisation suivante :

_Python_

```python
from dataclasses import dataclass

@dataclass
class History:
    previous_questions: list[Question]
    current: Question
    remaining_questions: list[Question]
```

_Elm_

```elm
type alias History =
    { previousQuestions : List Question,
    , current : Question
    , remainingQuestions : List Question
    }
```

_Typescript_

```typescript
type History = {
  previousQuestion: [Question];
  current: Question;
  remainingQuestions: [Question];
};
```

La liste complète des questions sera alors obtenue par la concaténation des questions précédentes, de la courante et de celles qui reste. L'idée étant de faire `previous_questions + [current] + remaining_questions` pour constituer notre liste de questions.

Avec une modélisation comme celle-ci, il est impossible d'avoir une liste vide car `current` est forcément requis, et il est aussi impossible d'avoir une question courante qui ne fait pas partie de la liste !

Un exemple complet en Python donnerait cela :

_Python_

```python
from dataclasses import dataclass

@dataclass
class Question:
    prompt: str
    response: str | None

@dataclass
class History:
    previous_questions: list[Question]
    current: Question
    remaining_questions: list[Question]

question1: Question = Question(prompt="question 1", response="response 1")
question2: Question = Question(prompt="question 2", response="response 2")
question3: Question = Question(prompt="question 3", response=None)
question4: Question = Question(prompt="question 4", response="response 4")

history: History = History(
    previous_questions=[question1, question2],
    current=question3,
    remaining_questions=[question4],
)

history_as_list: list[Question] = (
    history.previous_questions + [history.current] + history.remaining_questions
)
```

Et voilà 🎉

La modélisation que nous avons choisie nous assure que :

- Notre liste ne sera jamais vide
- La question courante fait forcément partie des questions possibles

Évidemment ce n'est pas toujours aussi simple que ça et toujours possible facilement, mais il est toujours bon d'essayer au maximum d'éviter les états impossibles grâce à nos choix de modélisation. Moins nous avons de vérifications à faire en code, plus notre programme sera robuste.

Tout ce qui est normalement impossible devrait l'être par le choix de notre modélisation autant que possible !

Happy coding, et n'hésitez pas à me faire des retours sur mon [compte Mastodon](https://mamot.fr/@vjousse).
