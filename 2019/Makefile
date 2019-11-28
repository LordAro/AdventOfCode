ifdef DEBUG
FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
DIR=debug
else
FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused
CARGO_FLAGS=--release
DIR=release
endif

CUR_SLNS=$(shell echo $(patsubst src/bin/day%.rs,%,$(wildcard src/bin/day*.rs)) | tr ' ' '\n' | sort -n)

all: ${CUR_SLNS}

.PRECIOUS: builds/${DIR}/day%
builds/${DIR}/day%: src/bin/day%.rs
	@echo -e '\x1b[1;34mCompiling day $* solution\x1b[0m'
	@RUSTFLAGS='${FLAGS} ${FLAGS_DAY_$@}' cargo build ${CARGO_FLAGS} --target-dir builds --bin day$*

%: builds/${DIR}/day%
	@echo -e '\x1b[1;32mRunning day $* solution\x1b[0m'
	@./builds/${DIR}/day$* inputs/day$*.input
.PHONY: all
