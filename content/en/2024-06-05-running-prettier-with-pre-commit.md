---
title: Running prettier with pre-commit
slug: 2024-06-05-running-prettier-with-pre-commit
date: "2024-05-05 09:36:00+00:00"
tags: prettier, javascript, python, pre-commit
category:
link:
description:
type: text
status: draft
---

By default, [pre-commit](https://pre-commit.com/) doesn't provide hooks for using it with [prettier](https://prettier.io/) to format javascript files, but it's pretty simple to use your already existing npm scripts to do so. That's what we will see here.

You may ask, why use [pre-commit](https://pre-commit.com/) instead of [husky](https://typicode.github.io/husky/) and [lint-staged](https://github.com/lint-staged/lint-staged) for example that work out of the box with prettier? Well because `lint-staged` is very javascript centric and doesn't play well with Python and its virtual environments.

<!-- TEASER_END -->

## TL;DR

The final [boilerplate source code is available on Github](https://github.com/vjousse/parcel-tailwindcss-elm-lang-boilerplate).

The `pre-commit` hook:

```yaml
repos:
  - repo: local
    hooks:
      - id: app-prettier
        name: run prettier
        language: system
        files: ^.*$
        types_or: [javascript, json]
        entry: |
          bash -c 'npm run fix:prettier --write "${@}"' --
```

The corresponding `package.json` script:

```json
{
  "scripts": {
    "fix:prettier": "npm run lint:prettier -- --write",
    "lint:prettier": "prettier --config .prettierrc --check"
  },
  "devDependencies": {
    "prettier": "^3.2.5"
  }
}
```

## Explaining the `pre-commit` hook

We use a custom hook with the `system` language defined. It will allow us to run a system command in the hook, namely a bash command/script.

## Explaining the `package.json` scripts

## Bonus: multi-directories projects
