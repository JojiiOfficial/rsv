default: release

build: 
	cargo build

release: 
	cargo build --release

clean: 
	rm -rf target

run: build
	./target/debug/rsv

all: build release

install:
	@if ! test -f target/release/rsv;then echo 'run "make release" first'; exit 1; fi
ifneq ($(shell id -u), 0)
	@echo "You must be root to perform this action."
	@exit 1
endif
	cp ./target/release/rsv /bin/

