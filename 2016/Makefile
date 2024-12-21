FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
CARGO_FLAGS=--release

FILEEXT=rs
SRCDIR=src/bin
BINDIR=builds/release
include ../common.mk

$(BINDIR)/day%: $(SRCDIR)/day%.rs
	@RUSTFLAGS='$(FLAGS) $(FLAGS_DAY_$*)' cargo build $(CARGO_FLAGS) --target-dir builds --bin day$*
	@RUSTFLAGS='$(FLAGS) $(FLAGS_DAY_$*)' cargo clippy $(CARGO_FLAGS) --target-dir builds --bin day$*
