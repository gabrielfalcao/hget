INSTALL_PATH		:=$(HOME)/usr/libexec/
HGET_NAME		:=hget
HGET_VERSION		:=$(shell cargo run -- --version | awk '{ print $$NF }')
HGET_DEBUG_EXEC		:=target/debug/$(HGET_NAME)
HGET_RELEASE_EXEC	:=target/release/$(HGET_NAME)
HGET_EXEC		:=$(HGET_DEBUG_EXEC)
HGET_RUN		:=$(HGET_DEBUG_EXEC)
HGET_RUN		:=cargo run --bin $(HGET_NAME) --

all: test debug release

$(INSTALL_PATH):
	mkdir -p $@

$(HGET_RELEASE_EXEC): $(INSTALL_PATH)
	cargo build --release

$(HGET_DEBUG_EXEC): $(INSTALL_PATH)
	cargo build

release: check fix | $(HGET_RELEASE_EXEC)
	install $(HGET_RELEASE_EXEC) $(INSTALL_PATH)/$(HGET_NAME)-$(HGET_VERSION)
	install $(HGET_RELEASE_EXEC) $(INSTALL_PATH)

debug: check fix | $(HGET_DEBUG_EXEC)
	install $(HGET_DEBUG_EXEC) $(INSTALL_PATH)/$(HGET_NAME)-$(HGET_VERSION)
	install $(HGET_DEBUG_EXEC) $(INSTALL_PATH)

clean: cls
	@rm -rf target

cleanx:
	@rm -rf $(HGET_DEBUG_EXEC)
	@rm -rf $(HGET_RELEASE_EXEC)

cls:
	-@reset || tput reset

fix:
	cargo fix

fmt:
	rustfmt --edition 2021 src/*.rs

check:
	cargo check --all-targets

build test: check
	cargo $@

run:
	$(HGET_RUN) -s apple.com abc.xyz amazon.com meta.com snpc.cv


.PHONY: e2e all clean cls release debug fix fmt check build test
