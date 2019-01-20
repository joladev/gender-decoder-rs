.PHONY: help

BUILD ?= `git rev-parse --short HEAD`

help:
		@echo "gender-decoder-rs:$(BUILD)"
		@perl -nle'print $& if m{^[a-zA-Z_-]+:.*?## .*$$}' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

build: ## Build the Docker image
		docker build \
				-t yuhama/gender-decoder-rs-$(BUILD) \
				-t yuhama/gender-decoder-rs:latest .

run: ## Run the app in Docker
		docker run --expose 4000 -p 8090:8090 \
				--volume "uploads:/opt/app/uploads:" \
				--rm -it yuhama/gender-decoder-rs:latest
