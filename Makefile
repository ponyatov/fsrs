# dir
CAR = $(HOME)/.cargo

# tool
RUSTUP = $(CAR)/bin/rustup
CARGO  = $(CAR)/bin/cargo

# src
R += $(wildcard src/*.rs) Cargo.toml

# all
.PHONY: all run
all: $(R)
	cargo build
run: $(R) 
	cargo run

# install
.PHONY: install update
install: $(RUSTUP) $(CARGO)
	$(MAKE) update
	cargo install cargo-watch
	rustup target install x86_64-unknown-linux-gnu
	rustup target install wasm32-unknown-unknown
update: $(RUSTUP) $(CARGO)
	sudo apt update
	sudo apt install -uy `cat apt.Debian`
	rustup self update && rustup update

$(RUSTUP) $(CARGO):
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
