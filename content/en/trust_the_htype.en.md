---
title: Trust the h^Wtype
slug: trust-the-htype
date: 2012-04-26 20:39:20+02:00
tags: functional programming
category: 
link: 
description: 
type: text
---


I was used to write a lot of PHP code, mostly using the symfony framework. When I discovered unit testing with PHP, I was like I had discovered a new way of programming.<!-- TEASER_END --> 

## Unit testing: your own compiler

It was not required anymore to open a web browser to check if my code was working, I just had to launch a command line, and the red or green lights were telling me if my code was working (or at least, not throwing trivial warnings, notices or exceptions). It was a revolution for me.

It took me a while to realize that for each and every project that I was doing in PHP, half of my tests was doing the job of a compiler (checking the expected output, checking that methods names have been changed everywhere when refactoring, ...). What was the point in re-inventing the wheel?

## Type inference: a taste of dynamic language

I was re-inventing the wheel because languages with a compiler were (to me) really verbose for a little added value. Having to tell to Java that _"foo"_ must be treated as a string and _2_ as an integer, and _3.2_ as a float number was adding a lot of boilerplate to the code:

```java
String test = "foo";
int two = 2;
float three = 3.2;
```

Using PHP it was more easy:

```php
$test = "foo";
$two = 2;
$three = 3.2;
```

But what I loosed in PHP was that substracting a number to a string should not be permitted:

```php
$bar = $test - $two;
```

But obviously in PHP it just works. I mean, it does not complain:

```php
echo $bar;
```

And it displays ... the number -2 (seriously? ;) ).

So what about something like that (example in Scala):

```scala
val test = "foo"
val two = 2
val three = 3.2
```

You don't have to specify the types (like in PHP), but it complains when you try to do so:

```scala
val bar = test - two
```

Here is the error message when you try to compile:

```scala
error: value - is not a member of java.lang.String
val bar = test - two
            ^
one error found
```

You can see that the compiler as inferred the type of _test_ as a String without having to tell it explicitly.

## The compiler is your friend

If you are using a language like PHP (or some other dynamic language) and you are writing tons of unit tests to cover your (hypothetical? ;) ) refactoring, specifying types hints in function parameters (typical in PHP those days) you should really ask yourself if you are using the right tool for the right job.

Using a srongly typed language that is using type inference (like Scala or Haskell) could save you plenty of time. You'll have the unique sensation that: "once it compiles, it will most likely work". You should think twice about it.
