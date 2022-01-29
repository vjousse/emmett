<!-- 
.. title: Server side syntax highlighting with Crystal lang, NOIR and Lucky framework
.. slug: crystal-lang-syntax-highlighting-with-noir-and-lucky.md
.. date: 2021-06-02 09:00:00+00:00
.. tags: crystal, luckyframework, noir
.. category: 
.. link: 
.. description: 
.. type: text
-->

I hate to rely on Javascript to highlight code. This is something that we can and should do on the backend IMHO: no point in overloading the browser for that. This blog is written in [Crystal and Lucky Framework](https://github.com/vjousse/lucky-blog) and I was using [Prism.js](https://prismjs.com/) to highlight my code as I didn't found a ready to be used solution at that time. It's not the case anymore: [NOIR](https://github.com/MakeNowJust/noir) and some custom renderer for [Github Flavored Mardown library cr-mark-gfm](https://github.com/amauryt/cr-cmark-gfm) did the trick. Here is a little tutorial on how I did it.

<!-- TEASER_END -->

## Adding NOIR to your project

[NOIR](https://github.com/MakeNowJust/noir) is a port of the famous [Rouge](https://github.com/rouge-ruby/rouge/), a pure Ruby syntax highlighter. NOIR is not as complete as Rouge but it works, and adding other languages should be pretty easy, just a matter of porting the corresponding lexer from Rouge.

First, you need to add [NOIR](https://github.com/MakeNowJust/noir) to your project. Unfortunately, the latest commit on NOIR is 3 years old and, by default, the project is not compatible with Crystal 1.0.0 due to the lack of a crystal version number in its `shard.yml`. I made a [fork here](https://github.com/vjousse/noir) so that you can directly include it in your crystal 1.0.0 project:

```yaml
dependencies:
  noir:
    github: vjousse/noir
```

## Writing a custom parser for cr-mark-gfm

Then, as I said above, I'm using [cr-mark-gfm](https://github.com/amauryt/cr-cmark-gfm) to convert my markdown content to HTML. To be able to highlight my code on the backend, I had to override the default `Cmark::HTMLRenderer` class and more especially the `def code_block(node)` method that is in charge of rendering the `<code>` blocks. The goal was to wire some code from NOIR here to highlight the content of the `<code>` block.

Here is the code I put on the top of my `src/models/post.cr` file.
```crystal
require "noir"
require "noir/themes/monokai"
require "noir/lexers/crystal"
require "noir/lexers/css"
require "noir/lexers/html"
require "noir/lexers/javascript"
require "noir/lexers/json"
require "noir/lexers/python"
require "noir/lexers/ruby"

class PostHTMLRenderer < Cmark::HTMLRenderer

  def code_block(node)
    cr
    out %(<pre class="code")
    sourcepos node
    fence_info = node.fence_info

    if fence_info.bytesize.zero?
      out "><code>"
      out escape_html(node.literal),
    else
      tags = fence_info.split(' ', remove_empty: true)
      language_name = tags[0]

      if @options.github_pre_lang?
        out %( lang="#{escape_html(tags.shift)})
        tags.each { |tag| out %(" data-meta="#{escape_html(tag)}) } if @options.full_info_string?
        out %("><code class="highlight">)
      else
        out %(><code class="highlight language-#{escape_html(tags.shift)})
        tags.each { |tag| out %(" data-meta="#{escape_html(tag)}) } if @options.full_info_string?
        out %(">)
      end

      theme = Noir.find_theme("monokai").not_nil!
      formatter_out : IO = IO::Memory.new

      if lexer = Noir.find_lexer(language_name)
        Noir.highlight node.literal,
          lexer: lexer,
          formatter: Noir::Formatters::HTML.new formatter_out
        out formatter_out.to_s
      else
        Log.info { "Lexer for '#{language_name}' not found." }
        out escape_html(node.literal)
      end
    end

    out "</code></pre>\n"
  end

end
```

What I did here is that I took the code from the [cr-mark-gfm HTML renderer](https://github.com/amauryt/cr-cmark-gfm/blob/4dd681983e6fe10c5e44ef7f38ed94e8a7a9b147/src/cmark/renderers/html_renderer.cr#L58) and added the one I found in the NOIR library. By default, I'm using the `monokai` theme from NOIR (there is also a solarized theme available).

Then I just have to convert my markdown to HTML as unusual. Here is the rest of my `src/models/post.cr` file:

```crystal
class Post < BaseModel

  avram_enum Lang do
    Fr # 0
    En # 1
  end

  table do
    column title : String
    column content : String
    column teaser : String?
    column slug : String
    column filename : String
    column published_at : Time
    column lang : Post::Lang
    column hash : String
  end

  def md_to_html(md : String): String
      options = Cmark::Option.flags(ValidateUTF8, Smart, Unsafe, GithubPreLang)
      extensions = Cmark::Extension.flags(Table, Tasklist)

      nodes = Cmark.parse_gfm(md.gsub("<p></p>", ""), options)

      renderer = PostHTMLRenderer.new(options, extensions)
      renderer.render(nodes)
  end

  def content_to_html : String
    md_to_html(self.content)
  end

  def teaser_to_html : String
    if teaser = self.teaser
      md_to_html(teaser)
    else
      ""
    end
  end

end
```

You can find the whole [file directly on Github](https://github.com/vjousse/lucky-blog/blob/3c007f9e364bb3bbca31f2e6d0c95b496ae2761e/src/models/post.cr).

## Add some nice CSS

Now that we have some HTML and classes that have been added to our output, we need to colorize them. Here is how the output looks like:

![NOIR syntax highlighting](/images/noir_syntax_hl.png)

NOIR provides some CSS that you can copy paste to get this result. Follow the instructions on the [NOIR's github README](https://github.com/MakeNowJust/noir) to compile the `ET NOIR` CLI tool and execute the following command to get the CSS for the `monokai` theme:

```shell
    ./bin/etnoir style monokai
```

This should output something like the image below.

![NOIR css](/images/noir_css.png)

Copy paste the content to your CSS file, and you're done. Here is [a direct link to the CSS I use for this blog](https://github.com/vjousse/lucky-blog/blob/3c007f9e364bb3bbca31f2e6d0c95b496ae2761e/src/css/app.scss#L139).

## Conclusion

Crystal and NOIR provide a __solid fundation to highlight code__ on the server side. Even if only 7 languages are available for now (`crystal`, `css`, `html`, `javascript`, `json`, `python`, `ruby`) it should be straightforward to port other languages using some [examples from Rouge](https://github.com/rouge-ruby/rouge/tree/master/lib/rouge/lexers).

I really enjoy playing around with the Crystal libs out there. It looks like Crystal just need a little bit more ❤️ to be the new defacto "__dynamic language with an useful compiler__".

That's all for today folks. Don't hesitate to [reach out to me](/about) if you have any question. Enjoy! 🎉
