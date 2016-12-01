FLAGS=

CUR_SLNS=$(shell ls src/day* | cut -d/ -f2 | cut -d. -f1 | cut -dy -f2 | sort -n)

all: ${CUR_SLNS}

%:
	@echo -e '\x1b[1;34mCompiling day $@ solution\x1b[0m'
	rustc -o builds/day$@ ${FLAGS} ${FLAGS_DAY_$@} src/day$@.rs
	@./builds/day$@ inputs/day$@.input

.PHONY: all
