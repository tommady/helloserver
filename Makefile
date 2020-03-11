golang:
	docker buildx build \
  --platform linux/amd64,linux/arm64,linux/arm/v7 \
  --tag starkandwayne/hello-multiarch:golang \
  --push ./go 

rust:

.PHONY: golang rust


