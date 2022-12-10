FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
FLAGS_DAY_8b=-C opt-level=3 -C debuginfo=0
FLAGS_DAY_16=-C opt-level=3 -C debuginfo=0
FLAGS_DAY_17=-C opt-level=3 -C debuginfo=0
FLAGS_DAY_21=-C opt-level=3 -C debuginfo=0

FILEEXT=rs
SRCDIR=src/bin
BINDIR=target/debug
include ../common.mk

target/debug/day%: src/bin/day%.rs
	@RUSTFLAGS='$(FLAGS) $(FLAGS_DAY_$*)' cargo build --bin day$*
