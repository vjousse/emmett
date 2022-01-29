<!-- 
.. title: Elm and Phoenix/Elixir in production for France TV
.. slug: elm-phoenix-elixir-production
.. date: 2016-10-09 08:00:00+02:00
.. tags: functional programming, phoenix, elm, elixir
.. category: 
.. link: 
.. description: 
.. type: text
-->

Are `Elm` and `Phoenix/Elixir` ready for prime time? I'll let you decide: they were both used in live during the __main French political show__ called "[L'émission Politique](https://twitter.com/lepolitique)" to help generate a word cloud based on the guest speech. To the date of this writing, the guests were __Nicolas Sarkozy__ (12 millions viewers), __Arnaud Montebourg__ (9 millions) and lately __Alain Juppé__ (13 millions).
<!-- TEASER_END -->

__tl;dr__: __Elm__ allowed me to code a __reliable frontend app__ in a very short time,  __Phoenix__ was so __easy to grasp__ that I felt young again while coding the backend administration.

![Word cloud Alain Juppe](/images/nuage_juppe.png)


## Context

I'm the CEO (CTO, CSO, _<put any cool name here\>_) of [Voxolab](https://voxolab.com), a company providing __speech recognition analytics__ for businesses. Lately I've been working for [Voxygen](https://voxygen.fr), where I had the opportunity to develop a system to generate a live word cloud, based on the most frequent words pronounced by the guest of a political show.

The requirements:

1. Record/transcribe the speech of the guest microphone in real time
2. Generate a word cloud based on the words frequency in real time
3. If possible, use a computer already available at France TV. It can be a different computer for each show.
4. Technical people of the show should be autonomous

So, starting from here, I decided to develop a "web solution" that would allow me to record the sound from every computer having a browser installed (to be totally honest, having Chrome or Firefox installed) and to send it to a remote server for transcription.

Here is what I already had:

1. Google Chrome installed on the computers of France TV with the guest microphone plugged in
2. A javascript library to send microphone input to the remote server
3. A live speech recognition system on a remote server
4. A python script generating word clouds based on text data

Here is what I needed to do:

1. A __human ready interface__ in order to manage the connection between the browser and the speech recognition server
2. A __backend administration__ to manage the different shows, the stop words list, …

I decided to use Elm for #1 and Phoenix for #2. I've been writing `Elm` and `Phoenix` code in my side projects for only 3 months. Usually, for my business projects, I'm using [Python/Flask](https://palletsprojects.com/p/flask/) for my backends and [Mithril.js](https://mithril.js.org/) for my frontends. So, why Elm and Phoenix this time?

## Why Elm?

In fact this was not the question I did ask to myself. The question was: __why Javascript__? And to be honest, I couldn't show up with a satisfying answer. So I decided to go with `Elm`, mainly for those reasons:

1. I needed __something I could rely on__. I can rely on Elm compiler, I can't rely on Javascript runtime errors.
2. I knew I would have to __refactor the code a lot__ during a short period of time. I wanted to be confident while breaking things.
3. As a business owner/recruiter, I wanted something with a __low entry barrier__ for my future hirings. Elm has a __low entry barrier__, the JS ecosystem/fatigue/mess doesn't.
4. I knew I could never get stuck. __Interoperation between Javascript and Elm__ could always save me.

I was not disappointed: it's the first time I felt so confident when running a frontend app in production.

## Why Phoenix?

The need for the __backend app__ was pretty simple: some basic [CRUD](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete) operations. I could have used `Python/Flask` for this without any problem. But I knew that it was potentially something that would need a lot of live/websockets connections in a near future. Phoenix shines in this domain and provides easy backend generation for CRUD operations. Last but not least, the entry barrier is pretty low too. The community is awesome, and the available resources are really significant now (thanks to [Programming Phoenix book](https://pragprog.com/book/phoenix/programming-phoenix)).

I placed a bet on Phoenix/Elixir because its ability to scale is rooted at the language level thanks to the Erlang VM.

## Conclusion

So yes, `Elm` and `Phoenix` are production ready for my needs and were used in a live show with more than 10M viewers. Most importantly, I'm confident I can build my company products using both of them because:

1. They answer perfectly the main problems in web development: __reliability, scalability, ease of use__.
2. Hiring people should be easy: the __entry barrier is very low__, despite both of them being functional languages.
