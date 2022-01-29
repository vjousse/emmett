<!-- 
.. title: Adding tailwindcss to your lucky project
.. slug: adding-tailwindcss-to-lucky-crystal-lang
.. date: 2020-08-09 20:00:00+02:00
.. tags: crystal, beginner, tailwindcss
.. category: 
.. link: 
.. description: 
.. type: text
-->

A short post about how to add [tailwindcss](https://tailwindcss.com/) to your [lucky](https://www.luckyframework.org/) project.
_Disclaimer_: I'm new to the [crystal programming language](https://crystal-lang.org/), to the [lucky framework](https://www.luckyframework.org/) and all the frontend stuff including [webpack](https://webpack.js.org/) and [tailwindcss](https://tailwindcss.com/).
<!-- TEASER_END -->

## Install

Install `tailwindcss` using npm or yarn:

```shell
# Using npm
npm install tailwindcss

# Using Yarn
yarn add tailwindcss
```

Modify your `src/css/app.scss` file to include the following code (for example at the top of the file):

```css
@tailwind base;

@tailwind components;

@tailwind utilities;
```

Modify your `webpack.mix.js` by adding a `postCss` plugin for tailwind in the `options` function like below:

```js
  .options({
    // If you want to process images, change this to true and add options from
    // https://github.com/tcoopman/image-webpack-loader
    imgLoaderOptions: { enabled: false },
    // Stops Mix from clearing the console when compilation succeeds
    clearConsole: false,
    postCss: [require('tailwindcss')]
  })
```

## Enjoy

You can test it by adding a button to some of your lucky pages. Adding the following code:

```crystal
button class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full" do
  text " Button "
end
```

Should result in such a nice button:


![Tailwind button](/images/tailwind_button.png)
