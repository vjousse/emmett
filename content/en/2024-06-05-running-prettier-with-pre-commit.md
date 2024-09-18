---
title: Running prettier with pre-commit
slug: running-prettier-with-pre-commit
date: "2024-09-17 09:36:00+00:00"
tags: prettier, javascript, python, pre-commit
category:
link:
description:
type: text
---

By default, [pre-commit](https://pre-commit.com/) doesn't provide hooks for using it with [prettier](https://prettier.io/) to format javascript files, but it's pretty simple to use your already existing npm scripts to do so. That's what we will see here.

If you don't know why you should consider using `pre-commit` you should really read about it. It will allow you to run linters, formatters, and everything that you may want every time you stage a file to git, very useful when working in a team.

You may ask, why use [pre-commit](https://pre-commit.com/) instead of [husky](https://typicode.github.io/husky/) and [lint-staged](https://github.com/lint-staged/lint-staged) for example that work out of the box with prettier? Well because `lint-staged` is very javascript centric and doesn't play well with Python and its virtual environments.

<!-- TEASER_END -->

## TL;DR

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
          bash -c 'npm run fix:prettier "${@}"' --
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

> **Update**: you can also direclty use [this `pre-commit` hook](https://github.com/rbubley/mirrors-prettier) if you prefer.

## Explaining the `pre-commit` hook

We use a custom hook with the `system` language defined. It will allow us to run a system command in the hook, namely a bash command/script. This hook will be run on every staged file thanks to the `--` at the end of the line meaning that arguments passed to the hook should be forwarded to the `bash` command).

The magic happens with the bash instruction `"${@}"`. In bash, it means "all positional arguments", so bash will pass to prettier all the positional arguments (the file path in fact) that the hook gave to it. And that's it 🎉

## Explaining the `package.json` scripts

`lint:prettier` is the base script. You can call it with a file name `npm run lint:prettier path/to/file.js` and it will run prettier on that file. `fix-prettier` just calls `lint:prettier` with an additional argument `--write` to effectively write the detected changes (you can use `--` to pass extra arguments between scripts).

So to sum-up:

- The hook calls bash with the staged file name as parameter
- Bash calls `npm run fix:prettier` with the file name as parameter too
- `npm run fix:prettier` calls `npm run lint:prettier` with the file name and an extra `--write` argument

## Bonus: multi-directories/mono-repo projects

If your project is a mono repo with multiple apps in it and that, for example, your project and your `package.json` are located in `app/`, just change the hook entry by:

```yaml
bash -c 'npm --prefix app run fix:prettier "${@#*/}"' --
```

The added `#*/` tells bash to remove the first part of the file name (because `prettier` is run from `app/` where the `package.json` is located and the hook is run from the root of your project).
