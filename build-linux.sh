docker buildx build --platform linux/amd64 -t linux-builder . --load
docker run --rm -it -v "$(pwd)":/usr/src/app linux-builder