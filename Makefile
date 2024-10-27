.PHONY: generate-go
generate-go:
	@echo "Generating go bindings"
	cd proto && buf generate
	@echo "Done"

.PHONY: generate-rs
generate-rs:
	@echo "Generating rust bindings"
	cargo build
	@echo "Done"

.PHONY: generate
generate: generate-go generate-rs