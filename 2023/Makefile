ifdef DEBUG
FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
DIR=debug
else
FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
CARGO_FLAGS=--release
DIR=release
endif

FILEEXT=rs
SRCDIR=src/bin
BINDIR=builds/$(DIR)
include ../common.mk

$(BINDIR)/day%: src/bin/day%.rs
	@echo -e '\x1b[1;34mCompiling day $* solution\x1b[0m'
	@RUSTFLAGS='$(FLAGS) $(FLAGS_DAY_$@)' cargo build $(CARGO_FLAGS) --tests --target-dir builds --bin day$*
	@RUSTFLAGS='$(FLAGS) $(FLAGS_DAY_$@)' cargo test $(CARGO_FLAGS) --target-dir builds --bin day$*
