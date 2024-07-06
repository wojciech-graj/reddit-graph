#!/bin/bash

docker container run \
	--rm \
	--volume $(pwd):/docker/src/ \
	--user 1000:1000 \
	--tty \
	--interactive \
	${CONTAINER:-reddit} \
	bash -c \
	"trap 'sleep 1; exit' SIGINT \
	&& cd /docker/src \
	&& $*"
