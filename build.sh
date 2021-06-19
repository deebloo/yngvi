
#!/bin/bash

# build debian
IMAGE_NAME=debian/acurite-weather

docker build -f builders/Debian.Dockerfile -t $IMAGE_NAME .

ID=$(docker create $IMAGE_NAME)

docker cp $ID:/target/release/acurite-weather builds/debian/acurite-weather

docker rm -v $ID
