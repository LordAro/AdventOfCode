ifdef DEBUG
FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
DIR=debug
else
FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
CARGO_FLAGS=--release
DIR=release
endif
CARGO_FLAGS+=--target-dir builds

FILEEXT=rs
SRCDIR=src/bin
BINDIR=builds/$(DIR)
include ../common.mk

builds/$(DIR)/day2: src/intcode.rs
builds/$(DIR)/day5: src/intcode.rs
builds/$(DIR)/day7: src/intcode.rs
builds/$(DIR)/day9: src/intcode.rs
builds/$(DIR)/day11: src/intcode.rs
builds/$(DIR)/day13: src/intcode.rs
builds/$(DIR)/day15: src/intcode.rs
builds/$(DIR)/day17: src/intcode.rs

builds/$(DIR)/day%: src/bin/day%.rs
	@echo -e '\x1b[1;34mCompiling day $* solution\x1b[0m'
	@RUSTFLAGS='$(FLAGS) $(FLAGS_DAY_$@)' cargo build $(CARGO_FLAGS) --tests --bin day$*
	@RUSTFLAGS='$(FLAGS) $(FLAGS_DAY_$@)' cargo clippy $(CARGO_FLAGS) --bin day$*
	@RUSTFLAGS='$(FLAGS) $(FLAGS_DAY_$@)' cargo test $(CARGO_FLAGS) --bin day$*
