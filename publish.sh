rm -rf output/ && cargo run && rsync -avz --delete output/ vjousse@emmett.jousse.org:/home/data/vincent.jousse.org
