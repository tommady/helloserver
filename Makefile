build-and-push-image-golang:
	docker buildx build \
	--platform linux/amd64,linux/arm64,linux/arm/v7 \
  --tag tommady/helloserver:golang \
  --push ./go 

build-and-push-image-rust:
	docker buildx build \
  --platform linux/arm64 \
  --tag tommady/helloserver:rust \
  --push ./rust

deploy-golang:
	kubectl create namespace helloserver-golang
	kubectl kustomize ./overlays/golang/. | kubectl apply -f -

deploy-rust:
	kubectl create namespace helloserver-rust
	kubectl kustomize ./overlays/rust/. | kubectl apply -f -	

delete-golang:
	kubectl delete deployment helloserver-golang --namespace=helloserver-golang
	kubectl delete service helloserver-golang --namespace=helloserver-golang
	kubectl delete namespace helloserver-golang --namespace=helloserver-golang

delete-rust:
	kubectl delete deployment helloserver-rust --namespace=helloserver-rust
	kubectl delete service helloserver-rust --namespace=helloserver-rust
	kubectl delete namespace helloserver-rust --namespace=helloserver-rust

.PHONY: build-and-push-image-golang build-and-push-image-rust deploy-golang deploy-rust delete-golang delete-rust
