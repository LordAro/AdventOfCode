DFLAGS=-w
#FLAGS_DAY_4=-lssl -lcrypto

CUR_SLNS=$(shell ls src/day* | cut -d/ -f2 | cut -d. -f1 | cut -dy -f2 | sort -n)

all: ${CUR_SLNS}

%:
	@echo -e '\x1b[1;34mCompiling day $@ solution\x1b[0m'
	dmd -of=builds/day$@ $(DFLAGS) $(FLAGS_DAY_$@) src/day$@.d
	@./builds/day$@ inputs/day$@.input

.PHONY: all
