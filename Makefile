NAME := dog-bot
CLUSTER_NAME := $(NAME)
PROJECT_ID=dog-bot-204307
CONTAINER_TAG := gcr.io/$(PROJECT_ID)/simple:v1

BIN := ./target/release/$(NAME)
SRC := $(shell find ./src -type f -name '*.rs')
PWD := $(shell pwd)
GROUP_ID := "${GROUP_ID}"
CHANNEL_ACCESS_TOKEN := "${CHANNEL_ACCESS_TOKEN}"
OS := $(shell uname)

.PHONY: local
local:
	docker build -t $(NAME) .
	docker run --rm \
		-e GROUP_ID=$(GROUP_ID) \
		-e CHANNEL_ACCESS_TOKEN=$(CHANNEL_ACCESS_TOKEN) \
		-t $(NAME)

.PHONY: remote
remote:
	docker build -t $(CONTAINER_TAG) .
	gcloud docker -- push $(CONTAINER_TAG) # Exist until setup container registry
	kubectl apply -f app.yaml
	# Or choose imperative way
	# kubectl run app --image=$(CONTAINER_TAG) --port 3000
	# kubectl expose deployment app --type=LoadBalancer --port 80 --target-port 3000

# Assume to execute only once
.PHONY: init
init:
	gcloud config set project $(PROJECT_ID)
	gcloud config set compute/zone asia-northeast1-a

.PHONY: create
create:
	gcloud container clusters create $(CLUSTER_NAME) --num-nodes=2

.PHONY: clean
clean:
	gcloud container clusters delete $(CLUSTER_NAME)
