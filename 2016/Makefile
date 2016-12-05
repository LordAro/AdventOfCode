FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused

CUR_SLNS=$(shell ls src/bin/day* | cut -d/ -f3 | cut -d. -f1 | cut -dy -f2 | sort -n)

all: ${CUR_SLNS}

%:
	@RUSTFLAGS='${FLAGS} ${FLAGS_DAY_$@}' cargo run -v --bin day$@ inputs/day$@.input

.PHONY: all
