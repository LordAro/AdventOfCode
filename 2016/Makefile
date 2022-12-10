FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
FLAGS_DAY_5=-C opt-level=3
FLAGS_DAY_14=-C opt-level=3 # md5ing it up

FILEEXT=rs
SRCDIR=src/bin
BINDIR=target/debug
include ../common.mk

target/debug/day%: src/bin/day%.rs
	@RUSTFLAGS='$(FLAGS) $(FLAGS_DAY_$*)' cargo build --bin day$*
