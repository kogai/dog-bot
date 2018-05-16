NAME := dog-bot
BIN := ./target/release/$(NAME)
SRC := $(shell find ./src -type f -name '*.rs')
PWD := $(shell pwd)
GROUP_ID := "${GROUP_ID}"
CHANNEL_ACCESS_TOKEN := "${CHANNEL_ACCESS_TOKEN}"
OS := $(shell uname)

bin/$(NAME): Cargo.toml $(SRC)
	docker build -t $(NAME) .
	docker run --rm -v `pwd`/target:/app/target \
		-e GROUP_ID=$(GROUP_ID) \
		-e CHANNEL_ACCESS_TOKEN=$(CHANNEL_ACCESS_TOKEN) \
		-t $(NAME)
	mkdir -p bin
	cp target/release/$(NAME) bin/$(NAME)
