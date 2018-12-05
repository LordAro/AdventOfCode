DFLAGS=-w -de -O -release
#FLAGS_DAY_4=-lssl -lcrypto

CUR_SLNS=$(sort $(patsubst src/day%.d,%,$(wildcard src/day*.d)))

all: $(CUR_SLNS)
timed: $(addprefix time,$(CUR_SLNS))

.PRECIOUS: builds/day%
builds/day%: src/day%.d
	@echo -e '\x1b[1;34mCompiling day $* solution\x1b[0m'
	ldmd -of=$@ $(DFLAGS) $(FLAGS_DAY_$@) $<

%: builds/day%
	@echo -e '\x1b[1;32mRunning day $* solution\x1b[0m'
	@./builds/day$* inputs/day$*.input

SHELL:=/bin/bash
time%: builds/day%
	@echo -e '\x1b[1;32mRunning day $* solution (timed)\x1b[0m'
	@time ./builds/day$* inputs/day$*.input

clean:
	rm -f $(addprefix builds/day,$(CUR_SLNS))
.PHONY: all
