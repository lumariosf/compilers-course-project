.PHONY: test golden

test:
	cargo test
	./tests/run-golden.sh

golden:
	./tests/run-golden.sh