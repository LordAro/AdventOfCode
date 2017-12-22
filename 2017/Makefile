FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
FLAGS_DAY_8b=-C opt-level=3 -C debuginfo=0
FLAGS_DAY_16=-C opt-level=3 -C debuginfo=0
FLAGS_DAY_17=-C opt-level=3 -C debuginfo=0
FLAGS_DAY_21=-C opt-level=3 -C debuginfo=0

CUR_SLNS=$(shell ls src/bin/day[0-9].rs src/bin/day[0-9][0-9].rs | cut -d/ -f3 | cut -d. -f1 | cut -dy -f2 | sort -n)

all: ${CUR_SLNS}
%:
	@RUSTFLAGS='${FLAGS} ${FLAGS_DAY_$@}' cargo run -v --bin day$@ inputs/day$@.input

.PHONY: all
