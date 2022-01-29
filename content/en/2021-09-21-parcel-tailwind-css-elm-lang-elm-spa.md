---
title: Using Parcel JS with Tailwind CSS, Elm Lang and Elm Spa
slug: 2021-09-21-parcel-tailwind-css-elm-lang-elm-spa
date: "2021-09-21 09:36:00+00:00"
tags: elm, tailwind, elm-spa, parcel
category: 
link: 
description: 
type: text
---

As I have been struggling to make [Parcel JS](https://v2.parceljs.org/), [Tailwind CSS](https://tailwindcss.com/), [PostCSS](https://postcss.org/), _whatever-the-hell-you-call-it_-CSS, [Elm Lang](https://elm-lang.org/) and [Elm SPA](https://www.elm-spa.dev/) work together I thought that sharing my journey could help some of you. Here it is!

<!-- TEASER_END -->

## TL;DR

The final [boilerplate source code is available on Github](https://github.com/vjousse/parcel-tailwindcss-elm-lang-boilerplate).

The bonus version with [elm-spa](https://www.elm-spa.dev/) is also [available on Github in the elm-spa branch](https://github.com/vjousse/parcel-tailwindcss-elm-lang-boilerplate/tree/elm-spa).

## Why?

My initial need was: using __Taiwind CSS__ with __Elm Lang__ and __Elm Spa__. It quickly turned into a journey through the classical [Javascript fatigue](https://medium.com/@ericclemmons/javascript-fatigue-48d4011b6fc4) tools and my new need was: find something that just get the job done. Spoiler: it's the case with __Parcel JS__.

## Parcel JS

This bundler just works. We will be using the v2 here (as I had a lot of problems to make the v1 work) so be sure to check the v2 documentation: [https://v2.parceljs.org/](https://v2.parceljs.org/) (note the v2 in the URL).

> Parcel is a compiler for all your code, regardless of the language or toolchain.

> Parcel takes all of your files and dependencies, transforms them, and merges them together into a smaller set of output files that can be used to run your code.

__package.json__
```json
{
  "name": "my-program",
  "version": "1.0.0",
  "devDependencies": {
    "parcel": "latest"
  }
}
```

Then just `npm install`.

Then let's create directories for every type of file we're going to have (`src/` will contain the elm files):

```
.
├── css
├── src
├── html
└── js
```

Put an example `index.html` file in your `html` folder:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8"/>
    <title>My First Parcel App</title>
    <link rel="stylesheet" href="../css/styles.css" />
    <script type="module" src="../js/app.js"></script>
  </head>
  <body>
    <h1>Hello, World!</h1>
  </body>
</html>
```

Then a `styles.css` file in your `css` folder:

```css
h1 {
  color: hotpink;
  font-family: cursive;
}
```

And an `app.js` file in your `js` folder:

```javascript
console.log('Hello world!');
```

Then modify your `package.json` to include some parcel scripts:

```json
{
  "name": "my-program",
  "version": "1.0.0",
  "scripts": {
    "start": "parcel html/index.html",
    "build": "parcel build html/index.html"
  },
  "devDependencies": {
    "parcel": "latest"
  }
}
```

Run `npm start` and visit [http://localhost:1234](http://localhost:1234) in your browser, you should get something like that:

![Parcel Hello World](/images/parcel_hello_world.png)

Let's add [Tailwind CSS](https://tailwindcss.com/)

## Tailwind CSS

Install [Tailwind CSS](https://tailwindcss.com/docs/installation) and its dependencies:


```
npm install -D tailwindcss@latest postcss@latest autoprefixer@latest
```

Add `tailwindcss` and `autoprefixer` as plugins in a `.postcssrc.json` file that you should create.

__.postcssrc.json__

```json
{
  "plugins": {
    "tailwindcss": {},
    "autoprefixer": {}
  }
}
```

Then, the next step is to create the configuration file for Tailwind with the following command:

```
npx tailwindcss init
```

This should create a `tailwind.config.js` file with the following content:

```javascript
module.exports = {
  purge: [],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
```

Finally, incude Tailwind in your `styles.css` file:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

h1 {
  color: hotpink;
  font-family: cursive;
}
```

Now you can modify your `index.html` page by adding some Tailwind CSS classes in it.

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8"/>
    <title>My First Parcel App</title>
    <link rel="stylesheet" href="../css/styles.css" />
    <script type="module" src="../js/app.js"></script>
  </head>
  <body class="bg-gray-200">
    <h1 class="text-4xl">Hello, World!</h1>
    <p class="text-green-500">Testing Tailwind</p>
  </body>
</html>
```

This should display something like that:

![Parcel Hello World Tailwind](/images/parcel_hello_world_tailwind.png)

Ok now, let's add some Elm!

## Elm

First, install the latest Elm version with npm:

```
npm install -D elm@latest-0.19.1
```

Then create an `elm.json` file yourself or let the init script do it for you using `npx elm init`. Here is my `elm.json` file:

```json
{
    "type": "application",
    "source-directories": [
        "src"
    ],
    "elm-version": "0.19.1",
    "dependencies": {
        "direct": {
            "elm/browser": "1.0.2",
            "elm/core": "1.0.5",
            "elm/html": "1.0.0"
        },
        "indirect": {
            "elm/json": "1.1.3",
            "elm/time": "1.0.0",
            "elm/url": "1.0.0",
            "elm/virtual-dom": "1.0.2"
        }
    },
    "test-dependencies": {
        "direct": {},
        "indirect": {}
    }
}

```

Then create a `Main.elm` file in your `src` directory with the following content:

```elm
module Main exposing (..)

import Browser
import Html exposing (Html, button, div, text)
import Html.Attributes exposing (class)
import Html.Events exposing (onClick)


main =
    Browser.sandbox { init = init, update = update, view = view }


type alias Model =
    Int


init : Model
init =
    0


type Msg
    = Increment
    | Decrement


update : Msg -> Model -> Model
update msg model =
    case msg of
        Increment ->
            model + 1

        Decrement ->
            model - 1


view : Model -> Html Msg
view model =
    div [ class "m-10 text-4xl" ]
        [ button [ onClick Decrement ] [ text "-" ]
        , div [ class "text-green-400" ] [ text (String.fromInt model) ]
        , button [ onClick Increment ] [ text "+" ]
        ]
```

Modify your `index.html`:

__html/index.html__

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8"/>
    <title>My First Parcel App</title>
    <link rel="stylesheet" href="../css/styles.css" />
    <script type="module" src="../js/app.js"></script>
  </head>
  <body>
    <div id="myapp"></div>
  </body>
</html>
```

And include your Elm program inside your javascript.

__js/app.js__

```javascript
import { Elm } from '../src/Main.elm';

Elm.Main.init({ node: document.getElementById('myapp') });
```

Run `npm start` and go to [http://localhost:1234/](http://localhost:1234/) you should see the Elm counter demo with some Tailwind styles:

![Parcel Hello World Tailwind Elm](/images/parcel_hello_world_tailwind_elm.png)

## Purging CSS classes

To avoid to include all the unused Tailwind CSS classes in the outputed CSS, configure your `tailwind.config.js` file by adding some directory to the purge entry:

```javascript
module.exports = {
  purge: [
    './src/**/*.elm',
    './js/app.js',
    './html/index.html',
    './css/styles.css',
  ],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
};

```

You can now build your code and you should find everything you need in the `dist/` folder.

```
npm run build
```

The output should be something like that:

```
> my-program@1.0.0 build
> parcel build html/index.html

✨ Built in 178ms

dist/index.html               250 B    646ms
dist/index.96cb94bc.css     3.67 KB    243ms
dist/index.e73152c4.js     15.89 KB    598ms
```

## Adding elm-spa

As a little bonus, we will see how to add [elm-spa](https://www.elm-spa.dev/) to our project.

First install the needed dependencies:

```
npm install -D chokidar-cli elm-spa elm-test npm-run-all
```

And change the `package.json` to add the scripts below:

```json
"scripts": {
  "start": "npm install && npm run build:dev && npm run dev",
  "test": "elm-test",
  "test:watch": "elm-test --watch",
  "build": "run-s build:elm-spa build:elm",
  "build:dev": "run-s build:elm-spa build:dev:elm",
  "dev": "run-p dev:elm-spa dev:elm",
  "build:elm": "parcel build html/index.html",
  "build:dev:elm": "elm make src/Main.elm --debug --output=public/dist/elm.compiled.js || true",
  "build:elm-spa": "elm-spa build .",
  "dev:elm": "parcel html/index.html",
  "dev:elm-spa": "chokidar elm/ -c \"npm run build:elm-spa\""
},
```

This will give you a lot of new commands to play with like `npm run dev`, `npm run build`, and so on.

Delete our example file in `elm/Main.elm` and init `elm-spa` in the current directory using:

```
npx elm-spa init
```

Answer yes to init the project. Remove the `public/index.html` that has been created and use the default `Main.elm` from `elm-spa`.


```
mv .elm-spa/defaults/Main.elm src
```

It should have changed part of your `elm.json` with some new dependencies and new source code directories.

And now, if everything goes well, `npm run build` should work as expected!

The [full code is available on Github](https://github.com/vjousse/parcel-tailwindcss-elm-lang-boilerplate/tree/elm-spa).
