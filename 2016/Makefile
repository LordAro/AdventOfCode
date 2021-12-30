FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
FLAGS_DAY_5=-C opt-level=3
FLAGS_DAY_14=-C opt-level=3 # md5ing it up

CUR_SLNS=$(shell ls src/bin/day* | cut -d/ -f3 | cut -d. -f1 | cut -dy -f2 | sort -n)

all: ${CUR_SLNS}

%:
	@RUSTFLAGS='${FLAGS} ${FLAGS_DAY_$@}' cargo run --bin day$@ inputs/day$@.input

.PHONY: all
