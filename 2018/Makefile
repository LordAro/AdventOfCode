DFLAGS=-w
#FLAGS_DAY_4=-lssl -lcrypto

CUR_SLNS=$(sort $(patsubst src/day%.d,%,$(wildcard src/day*.d)))

all: $(CUR_SLNS)

builds/day%: src/day%.d
	@echo -e '\x1b[1;34mCompiling day $* solution\x1b[0m'
	dmd -of=$@ $(DFLAGS) $(FLAGS_DAY_$@) $<

%: builds/day%
	@echo -e '\x1b[1;32mRunning day $* solution\x1b[0m'
	@./builds/day$* inputs/day$*.input

SHELL:=/bin/bash
time%: builds/day%
	@echo -e '\x1b[1;32mRunning day $* solution (timed)\x1b[0m'
	@time ./builds/day$* inputs/day$*.input

.PHONY: all
