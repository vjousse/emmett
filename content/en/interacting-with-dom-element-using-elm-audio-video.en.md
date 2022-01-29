---
title: Interacting with a DOM element using Elm (audio/video tag example)
slug: interacting-with-dom-element-using-elm-audio-video
date: "2016-08-03 14:25:20+02:00"
tags: elm
category: 
link: 
description: 
type: text
---

So, you want to write some [Elm](http://elm-lang.org/) code because you're a Hipster and want to be _in_. Fair enough. But being a Hipster has some downsides too. You soon realize that, even if __Elm__ is cool, it doesn't always provide all the things you may need. For example, how can you __interact with the HTML Audio element or any element not yet covered by the Elm core modules__? Don't worry, uncle Vince is here.

<!-- TEASER_END -->

## Preamble

The goal of the [@elm-lang organization](https://github.com/elm-lang) is to cover the entire [webplatform](https://platform.html5.org/) as described in [this blog post](http://elm-lang.org/blog/farewell-to-frp#what-is-next-). But in the meantine, how should we interact with basic elements such as the Audio element?  
We could do everything using [JS ports](http://guide.elm-lang.org/interop/javascript.html). But as we want to stay in the Elm world as much as we can, we will _read_ the values of the element using __DOM events__ inside Elm. Unfortunately, for _writing/mutating_ values (calling functions and/or updating a DOM element property) we have no choice but using __JavaScript port interop__.

_Note_: Another alternative would be writing [Native modules](https://github.com/elm-lang/core/tree/master/src/Native) to wrap the missing parts into some Elm greatness. But as doing so should be avoided (Native is subject to change and is not documented), this will not be covered here.

## One file example

For this tutorial, we are using Elm `0.17`.

## Reading element values: DOM events

We will take the [`<audio />` tag](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio) as an example for this blog post, but keep in mind that the techniques described here apply to every DOM element.

### App skeleton

Let's start with a minimal Elm program:

`Main.elm` [(view code)](https://github.com/vjousse/blog-nikola/blob/master/code/elm-audio/skeleton/Main.elm) 


```elm
module Main exposing (..)

import Html exposing (Attribute, Html, audio, div, text)
import Html.Attributes exposing (class, controls, type', src, id)
import Html.App as App
import Debug

main =
    App.program
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }

-- MODEL

type alias Model =
    { mediaUrl : String
    , mediaType : String
    }

-- MSG

type Msg
    = NoOp

-- INIT

init : ( Model, Cmd Msg )
init =
    { mediaUrl = "http://developer.mozilla.org/@api/deki/files/2926/=AudioTest_(1).ogg"
    , mediaType = "audio/ogg"
    }
        ! []

-- UPDATE

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        _ ->
            Debug.log "Unknown message" ( model, Cmd.none )

-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none

-- VIEW

view : Model -> Html Msg
view model =
    div [ class "elm-audio-player" ]
        [ audio
            [ src model.mediaUrl
            , id "audio-player"
            , type' model.mediaType
            , controls True
            ]
            []
        ]
```

Compile it using:

    elm make Main.elm

Open the generated `index.html` in your browser. You should see the default audio player of your browser showing up.

### Reading the currentTime property


Let's say that we want to display the `currentTime` property of the audio element just below it. Let's add it to the model as a `Float`:

```elm
type alias Model =
    { mediaUrl : String
    , mediaType : String
    , currentTime : Float
    }
```

We could as well use a `Maybe Float` here (and we certainly should). It would allow us to differenciate between _no value_ and the value _0_. But let's keep that for later.

Then init the currentTime to `0`:

```elm
init =
    { mediaUrl = "http://developer.mozilla.org/@api/deki/files/2926/=AudioTest_(1).ogg"
    , mediaType = "audio/ogg"
    , currentTime = 0.0
    }
        ! []
```

And display it in the view:

```elm

view : Model -> Html Msg
view model =
    div [ class "elm-audio-player" ]
        [ audio
            [ src model.mediaUrl
            , type' model.mediaType
            , controls True
            ]
            []
        , div [] [ text (toString model.currentTime) ]
        ]
```

Compile your program and you should see a `0` displayed below the audio player. That's cool, but how should we do to update it? By writing a [__custom event handler__](http://package.elm-lang.org/packages/elm-lang/html/1.1.0/Html-Events#custom-event-handlers).

Everytime the `timeupdate` event of the `audio` tag will be triggered, we will catch it and read the value of the `currentTime` attribute. The magic trick here is that every event contains the DOM element that triggered the event as the `target` attribute.

Start by importing the needed module:

```elm
import Html.Events exposing (on)
```

Create a new message type that will be triggered at each timeupdate:

```elm
-- MSG

type Msg
    = NoOp
    | TimeUpdate Float
```

Update the model when such a message is received:

```elm
-- UPDATE

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        TimeUpdate time ->
            ( { model | currentTime = time }, Cmd.none )

        _ ->
            Debug.log "Unknown message" ( model, Cmd.none )
```

Then add the custom event handler and the JSON decoder below your update function:

```elm
-- Custom event handler

onTimeUpdate : (Float -> msg) -> Attribute msg
onTimeUpdate msg =
    on "timeupdate" (Json.map msg targetCurrentTime)

-- A `Json.Decoder` for grabbing `event.target.currentTime`.


targetCurrentTime : Json.Decoder Float
targetCurrentTime =
    Json.at [ "target", "currentTime" ] Json.float

```

Here we write a custom event handler called `onTimeUpdate` using the [`on` function of the `Html.Events` module](http://package.elm-lang.org/packages/elm-lang/html/1.1.0/Html-Events#custom-event-handlers).

This custom event handler uses the Json decoder `targetCurrentTime` to read a `Float` value from the event located at `target.currentTime`.

Finally, make use of this new event handler in your `view`:

```elm
-- VIEW


view : Model -> Html Msg
view model =
    div [ class "elm-audio-player" ]
        [ audio
            [ src model.mediaUrl
            , type' model.mediaType
            , controls True
            , onTimeUpdate TimeUpdate
            ]
            []
        , div [] [ text (toString model.currentTime) ]
        ]
```

Now, compile your file and you should see the `currentTime` value updating when you play the file.

View the code of the resulting `Main.elm` [on github.](https://github.com/vjousse/blog-nikola/blob/master/code/elm-audio/reading-values/Main.elm) 

## Calling functions : Javascript ports

Now that we can read values coming from DOM elements, let's interact with the DOM elements from Elm. As I said in the preamble, the goal of Elm is to cover all the Web Platform. But in the meantime, we need to use javascript ports to communicate with elements not covered by pure Elm.

Let's say we want to add a button that will set the current time of the player at `2 seconds`.

### Elm side

First, let's declare that our module will contain some ports:

```elm
port module Main exposing (..)
```

Usually, you will only declare one `port module` in your application, registering all your ports in it. It will ease further debugging.

Then, as we will add a `button` and manage an `onClick` event, add the needed imports at the top of your file:

```elm
import Html exposing (Attribute, Html, audio, div, text, button)
import Html.Attributes exposing (class, controls, type', src, id)
import Html.App as App
import Html.Events exposing (on, onClick)
import Json.Decode as Json
```

We will add a new message for the set time functionality:

```elm
type Msg
    = NoOp
    | TimeUpdate Float
    | SetPlayerTime Float
```

And we will emit this message when clicking on a button in our view:

```elm
view : Model -> Html Msg
view model =
    div [ class "elm-audio-player" ]
        [ audio
            [ src model.mediaUrl
            , type' model.mediaType
            , controls True
            , onTimeUpdate TimeUpdate
            ]
            []
        , div [] [ text (toString model.currentTime) ]
        , button [ onClick (SetPlayerTime 2.0) ] [ text "Set current time to 2s" ]
        ]

```

We now need to handle this message in our update function like this:

```elm
update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        TimeUpdate time ->
            ( { model | currentTime = time }, Cmd.none )

        SetPlayerTime newTime ->
            ( model, setCurrentTime newTime )

        _ ->
            Debug.log "Unknown message" ( model, Cmd.none )

```

Notice that, in the `SetPlayerTime newTime` branch, we are not updating the model, we are just producing a `Cmd Msg` using a function called `setCurrentTime` with our number of seconds (`newTime`) as parameter. The model will be updated automatically by the `TimeUpdate` branch when the DOM event will be fired.

This `setCurrentTime` function is actually a port, that we need to define somewhere:

```elm

-- PORT


port setCurrentTime : Float -> Cmd msg
```

This port is used to send information on the Javascript side. In our case, it will tell javascript that we want to set the current time of the player to some `Float` value. We will of course need to implement this behaviour on the JS side. Let's do that now.

View the code of the resulting `Main.elm` [on github.](https://github.com/vjousse/blog-nikola/blob/master/code/elm-audio/ports/Main.elm) 


### Javascript side

In order to communicate between JS and Elm, we will need to add some Javascript code in our `index.html` file. But if you try to open the `index.html` file generated by `elm make` you will notice that it contains a lot of unreadable JS code. Let's put this code in a separate `elm.js` file, by compiling this way:

    elm make Main.elm --output=elm.js

Then, open your `index.html` file and change it as follows:

```html
<!DOCTYPE HTML>
<html>
    <head>
        <meta charset="UTF-8">
        <title>Main</title>
    </head>
    <body>
        <div id="elm"></div>
        <script src="elm.js"></script>
        <script>
        var node = document.getElementById('elm');
        var app = Elm.Main.embed(node);
        app.ports.setCurrentTime.subscribe(function(time) {
            var audio = document.getElementById('audio-player');
            audio.currentTime = time;
        });
        </script>

    </body>
</html>
```

Nothing fancy here. First we are including our newly generated `elm.js` file and we add some code to load our elm app into the `#elm` div.

Then we are using the special `setCurrentTime.subscribe` function created by our Elm port to get the `time` value sent by Elm on the JS side. We get the DOM element by it's id (the same we used in our Elm view) and we update the `currentTime` property of the audio element with the value previously sent using our Elm port.

Open the `index.html` in your browser, and you should be able to force the current time to 2s using our newly created button.

View the code of the resulting `index.html` [on github.](https://github.com/vjousse/blog-nikola/blob/master/code/elm-audio/ports/index.html) 


## Example using components

I'm always frustrated with blog posts (like this one) giving simple examples, but without showing how to integrate it in a *more complex application*. So I took the time to integrate the above code using child/parent components and the elm architecture.

I will not discuss the child/parent communication because [other people like Brian Hicks are already doing it very well](https://www.brianthicks.com/post/2016/06/23/candy-and-allowances-parent-child-communication-in-elm/). I will just give you the link to the code so that you can play with it by yourself : [code example on github](https://github.com/vjousse/blog-nikola/tree/master/code/elm-audio/ports-elm-arch).

## Wrapping Up

We just saw how to communicate with an HTML audio tag using Elm and javascript ports. This technique can of course be applied to every DOM element. You will find a [nice article by Søren Debois](https://medium.com/@debois/elm-the-dom-8c9883190d20#.mcmenrms8) explaining how he is using it to get the dimensions of DOM elements on the page.

Of course if you have any question, feel free to ping me on Twitter [@vjousse](http://twitter.com/vjousse) or directly on the Elm slack channel.

Have a nice day!
