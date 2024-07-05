# Emmett

A _reinvent the wheel_ project in Rust: static website generator for my blog.

## Configuration

Paths are configurable in `configuration.yaml`.

## Running

    cargo run

Then serve the `output/` directory using a web serve. For example, I'm using [simple-http-server](https://github.com/TheWaWaR/simple-http-server):

    simple-http-server -i output

If you want to publish drafts too (can be handy when working locally) you have to pass the `-p` flag:

    cargo run -- -p

## Watch for changes

    watchexec -e md cargo run

or to see drafts:

    watchexec -e md cargo run -- -p

## Deploy

    rsync -avz --delete output/ vjousse@emmett.jousse.org:/home/data/vincent.jousse.org

## Nginx rewrite rules

    rewrite ^/rss.xml$ /atom.xml permanent;

    # Do not rewrite the exact URL /blog/fr/
    if ($request_uri = /blog/fr/) {
      break;
    }

    # Do not rewrite URLs starting with /blog/fr/tech
    if ($request_uri ~* ^/blog/fr/tech) {
       break;
    }

    # Do not rewrite URLs starting with /blog/fr/perso
    if ($request_uri ~* ^/blog/fr/perso) {
       break;
    }

    # Rewrite URLs of type /blog/fr/something (with exactly one path component) to /blog/fr/perso/something
    # Avoid rewriting URLs that already contain /perso/ immediately after /fr/
    # Old blog urls
    rewrite ^/blog/fr/(?!perso/)([^/]+)/?$ /blog/fr/perso/$1 permanent;

## Goaccess configuration

    goaccess /var/log/nginx/vincent.jousse.org.access.log -o /home/data/report.html --log-format=COMBINED --ignore-crawlers --ignore-referrer=*.jousse.org --real-os --restore --persist --real-time-html --daemonize
