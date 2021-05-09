git submodule update --recursive --remote
docker build -t jq2rs .
docker run -it --rm -v "$(pwd)/jq2rs":/home/docker/jq2rs jq2rs 