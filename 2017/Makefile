FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
FLAGS_DAY_8b=-C opt-level=3 -C debuginfo=0
FLAGS_DAY_16=-C opt-level=3 -C debuginfo=0
FLAGS_DAY_17=-C opt-level=3 -C debuginfo=0
FLAGS_DAY_21=-C opt-level=3 -C debuginfo=0

FILEEXT=rs
SRCDIR=src/bin
include ../common.mk

# cargo build --out-dir is unstable :(
%:
	@RUSTFLAGS='$(FLAGS) $(FLAGS_DAY_$@)' cargo build --bin day$@
	cp target/debug/day$* builds/day$*
