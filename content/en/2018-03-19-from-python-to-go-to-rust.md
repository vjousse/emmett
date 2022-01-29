---
title:  "From python to Go to Rust: an opinionated journey"
date: 2018-03-22 09:00:00+01:00
categories: point of view
slug: from-python-to-go-to-rust
tags: rust go elm
---

When looking for a new backend language, I naturally went from [Python] to the new cool kid: [Go]. But after only one week of Go, I realised that Go was only half of a progress. Better suited to my needs than Python, but too far away from the __developer experience__ I was enjoying when doing [Elm] in the frontend. So I gave [Rust] a try.
<!-- TEASER_END -->

## Away from Python,

For backend development, I've mainly been using Python 3 for the past three years. From admin scripts to machine learning to [Flask]/[Django] applications, I've done a lot of Python lately, but at some point, __it didn't feel right anymore__. Well, to be honest, it's not really at "some totally random point" that it started not to feel right anymore, it was when I started to enjoy programming with a strongly typed language: [Elm].

I had the famous feeling "when it compiles it works", and once you've experienced that, __there is no way back__. You try stuff, you follow the friendly compiler error messages, you fix things, and then _tada_, it works!

Ok so, at this point I knew what I wanted from the "perfect" backend language:

1. __Static__ and __strong__ typing
2. Most of the stuff checked at __compile time__ (please, no exceptions!)
3. __No `null`__
4. __No mutability__
5. Handle __concurrency__ nicely

I see you coming: "hey, this is [Haskell]"! Yeah indeed, but for whatever reason, I've never managed to get anything done with [Haskell] (and I've been trying a lot). This is maybe only me, but from an outsider, the Haskell mindset seems elitist, the documentation and practical examples are lacking and it's hardly accessible to a beginner. [Learn you a Haskell for great good](http://learnyouahaskell.com/) is awesome but very long to read and too abstract for me (you don't build anything _for real_ during the book).

"Hey, and what about [Scala]?!". What do you mean by Scala? The better Java? The functional programming language with [Scalaz]? The Object Orienting Programming Functional language that may or may not fail at runtime with a `java.lang.NullPointerException` and needs a 4GB JVM running? I tried it some years ago and definitely, this is a no go for me.

After discussing with a few people, I decided to give __Go__ a try. It has a __compiler__, __no exceptions__, no `null` (but __null values__) and can handle __concurrency__ nicely.

## Into Go,

I decided to rewrite an internal project that was already done in Python using Go. Just to get a feeling of the differences between the two.

First feeling: learning __Go__ was so easy. In __one evening__, I was able to compile a Proof Of Concept of the project with basic features developed and some tests written. This was a very pleasant feeling, I was adding features very fast. The compiler messages were helpful, everything was fine.

And at some point, the tragedy started. I needed to add a field to some struct, so I just modified the struct and was ready to analyze the compiler messages to know where this struct was used in order to add the field where it was needed.

I compiled the code and … no error message. Everything went fine. But?! I just added a field to a struct, the compiler should say that my code is not good anymore because I'm not initializing the value where it should be!

The problem is that, not providing a value to a struct is not a problem in __Go__. This value will default to it's [zero value](https://tour.golang.org/basics/12) and everything will compile. This was the __show stopper__ for me. I realized that I __couldn't rely on the compiler__ to get my back when I was doing mistakes. At this point, I was wondering: why should I bother learning __Go__ if the compiler can't do much better than [Python and mypy](http://mypy-lang.org/)? Of course concurrency is much better with __Go__, but the downside of not being able to rely on the compiler was too much for me.

Don't get me wrong, I still think that __Go__ is a __progress compared to Python__ and I would definitively recommend people to learn Go instead of Python if they had to pick one of the two. But for my personal case, as someone who already knew Python and wanted something a lot safer, Go didn't bring enough to the table in that specific domain.

## Into Rust.

So __Go__ was not an option anymore as I realized that what I really needed was a __useful compiler__: a compiler that __should not rely on the fact that I know how to code__ (as it has been proven to be false a lot of times). That's why I took a look at __Rust__.

Rust was not my first choice because it advertises itself as a "system language", and I'm more of a web developer than a system one. But it had some very compelling selling points:

- No `null` values but an `Option` type (checked at compile time)
- No `exceptions` but a `Result` type (checked at compile time)
- Variables are __immutable__ by default
- Designed with concurrency in mind
- Memory safe by design, no garbage collector

I decided to rewrite the __same program__ than the one I did in Python and Go. The __onboarding was a lot harder__ than with Go. As I did with Go, I tried to go head first, but it was too hard: I needed some new concepts specific to Rust like __ownership__ or __lifetimes__ to understand the code I was seeing on StackOverflow. So I had no choice but to read the [Rust Book](https://doc.rust-lang.org/book/second-edition/), and it took me two weeks before I could start writing some code (remember that with Go it took me one evening).

But after this steep initial learning curve, I was enjoying writing Rust code, and I'm still enjoying it. With Rust, I don't have to trust myself, I just have to __follow the compiler__ and if I do so, it will most likely work if it compiles. In the end, this is the main feeling I was looking for when searching for a new backend language.


Of course, Rust has a lot of __downsides__:
- It's pretty __new and things are moving very fast__. I'm using [futures-rs](https://docs.rs/futures/) and [hyper.rs](https://hyper.rs/) in my project, and finding good documentation was really hard (kudos to the people on [irc.mozilla.org#rust-beginners](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners) for the help).
- It forces you to think of things you're not used to when coming from more _high-level_ languages: __how is the memory managed__ (with lifetimes and ownership).
- Compiler messages are not always straightforward to understand, especially when you're combining futures and their strange long types.
- Mutability is allowed, so you can get smashed with side effects

But, it also has a lot of __upsides__:
- It's __amazingly fast__
- __Tooling is good__ (cargo, rustfmt)
- Most of the things are __checked at compile time__
- You can potentially __do whatever you want with it__, from a browser, to a web app, to some game.
- Community is welcoming
- It's backed by Mozilla

## Wrapping up

__Go__ is cool but doesn't provide enough type safety __for me__. I would rather stick with __Python__ and its ecosystem than risking re-writing stuff in __Go__ if I don't need concurrency. If I need concurrency I would still not use __Go__ as its lack of type safety will surely hit me back at some point.

__Rust__ is the perfect candidate for concurrency and safety, even if the [futures-rs](https://crates.io/crates/futures) crate (this is how we call libs in Rust) is still early stage. I suspect that __Rust__ could become the defacto standard for a lot of backend needs in the future.

For a more in depth blog post discussing the differences between Go and Rust, be sure to check this amazing post by [Ralph Caraveo (@deckarep)](https://twitter.com/deckarep) : [Paradigms of Rust for the Go developer](https://medium.com/@deckarep/paradigms-of-rust-for-the-go-developer-210f67cd6a29).

At the very least, I think that I've found in Rust __my__ new favorite language for the backend.

[Python]: http://python.org/
[Go]: https://golang.org/
[Elm]: http://elm-lang.org/
[Rust]: https://www.rust-lang.org/
[Flask]: http://flask.pocoo.org/
[Django]: https://www.djangoproject.com/
[Haskell]: https://www.haskell.org/
[Scala]: https://www.scala-lang.org/
[Scalaz]: https://github.com/scalaz/scalaz
