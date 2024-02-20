# Emmett

A _reinvent the wheel_ project in Rust: static website generator for my blog.

## Configuration

Paths are configurable in `configuration.yaml`.

## Running

    cargo run

Then serve the `output/` directory using a web serve. For example, I'm using [simple-http-server](https://github.com/TheWaWaR/simple-http-server):

    simple-http-server -i output

## Watch for changes

    watchexec -e md cargo run

## Deploy

    rsync -avz --delete output/ vjousse@emmett.jousse.org:/home/data/vincent.jousse.org
