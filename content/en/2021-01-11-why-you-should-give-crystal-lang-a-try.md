---
title: Why you should give Crystal Lang a try: a quick review
slug: why-you-should-give-crystal-lang-a-try-a-quick-review
date: 2021-01-11 16:00:00+00:00
tags: crystal, beginner, lucky
category: 
link: 
description: 
type: text
---

I've been trying out [Crystal Lang](https://crystal-lang.org/) for a few months now and I have to admit that, I am __really happy__ with it, even if it's not a functional language nor has a very strict compiler. The reason why is pretty simple: it's as __easy to write as any dynamic language__ like Ruby, Python or PHP while still providing a __compiler that is helpful__. Kudos to the Crystal team for that!

<!-- TEASER_END -->

## Coming from Rust and Python

Last time, I wrote a blog post where [I explained how I came from Python to Rust](/blog/from-python-to-go-to-rust). Here is a quick summary: I love when a compiler has my back. Since this blog post, I spent quite some time coding with Rust but, at some point, I always came to the same conclusion: the compiler is way too much on my back!

I know I should try to understand lifetimes and ownership and that once I will be mastering those topics my life as a Rust programmer will be perfect… and blah blah bah. But, for whatever reason, it doesn't seem to work for me. And one day, while looking for a Web Framework, I came across [Crystal Lang](https://crystal-lang.org/), and more especially, the [Lucky Framework](https://www.luckyframework.org/). That's how it all began.

## Being productive

I'm a doer. I've been a doer of web stuff since 20 years now (😱) and I love when tools are helping me to achieve what I want, and that's what Crystal Lang has to offer. It's been a long time since I've enjoyed programming with a langage so much. Here is why:

- __Strongly typed__. You have a compiler that helps you and don't get too much on your way.
- __Familiar__. It looks like Ruby. I've never done a lot of Ruby, but its syntax looks very natural.
- __Easy to learn__. Most of the concepts are familiar to everyone as it respects the "everything is an object"/OOP philosophy.
- __Handles null value__. You don't have to worry about `Null Pointer Exceptions` and his friends.

## What I do and don't love

### The pros

So for me, here are the main selling points of Crystal Lang:

- Has an __useful compiler__
- Handles __null values__
- __Manage concurrency__ smoothly with fibers
- __Feels like a dynamic language__ to write
- Has really __good and mature web frameworks__ ([Lucky](https://www.luckyframework.org/), [Kemal](https://kemalcr.com/))
- Almost as __fast__ as C
- Is __easy__ to learn
- Has a __welcoming__ friendly community

### The cons

And here are the main difficulties (IMHO) with the language right now:

- Still young, so it's __missing a lot of libraries__
- __Small community__. Even if it's a friendly one, it's hard to find people writing Crystal Lang code

## The reality

Was it a show stopper for me? Not at all because :
- You already have a __lot of [awesome crystal libs](https://github.com/veelenga/awesome-crystal)__ available
- When you don't, you just to __find the Ruby one you would like to use__ and port it to Crystal (code is often the same)
- People are always __happy to help__

Here are some __concrete examples__:
- I wanted to __parse markdown files__ coming from [Nikola](https://getnikola.com/). I used the [markd shard](https://shardbox.org/shards/markd) (that's how libs are called in Crystal, _shards_) with some [custom parsing](https://github.com/vjousse/lucky-blog/blob/master/src/markdown/parser.cr).
- I wanted to __highlight code__ on the backend in my posts. I plan to use [noir](https://shardbox.org/shards/noir) a port of a Ruby Gem called Rouge, the one used for Jekyll. If it doesn't support the language I want to highlight, I will just have to port the lexer from Rouge.
- I had troubles with my Lucky project regarding many2many relations. I asked a question on the Discord server, and the problem was solved very fast.

## Conclusion

Should you give it a try? Definitively! It's __very fun__ to write Crystal Lang code. I've rewritten the [engine of this blog](https://github.com/vjousse/lucky-blog) in a few days from scratch and it was a real pleasure to do so.

Here are my feeling compared to other popular languages:

- Compared to __Go__: I really prefer __the syntax of Crystal__.
- Compared to __Python__: I love __Crystal's compiler__ (no, mypy is not quite the same)! I miss Python's ecosystem and tooling.
- Compared to __Elm__: I wish I could write elm code on the backend!
- Compared to __Elixir__: __Crystal's compiler__ is so helpful. Even if fibers are great with Crystal, Elixir's concurrency and reliability is way better (especially the actor system and let it crash philosophy).

You should give Crystal Lang a try if you want a language that helps you to write code that is fast and reliable. If you love the web, you should also give [Lucky Framework](https://www.luckyframework.org/) a try. The documentation is very well done and people on the discord server are very (very) welcoming.
