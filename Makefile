golang:
	docker buildx build \
	--platform linux/amd64,linux/arm64,linux/arm/v7 \
  --tag tommady/helloserver:golang \
  --push ./go 

rust:
	docker buildx build \
  --platform linux/arm64 \
  --tag tommady/helloserver:rust \
  --push ./rust

.PHONY: golang rust


