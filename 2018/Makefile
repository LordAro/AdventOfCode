DC?=ldmd2
ifdef DEBUG
DFLAGS=-w -de -debug -g -unittest
else
DFLAGS=-w -de -O -release
endif

CUR_SLNS=$(sort $(patsubst src/day%.d,%,$(wildcard src/day*.d)))

all: $(CUR_SLNS)
timed: $(addprefix time,$(CUR_SLNS))
perfd: $(addprefix perf,$(CUR_SLNS))

.PRECIOUS: builds/day%
builds/day%: src/day%.d
	@echo -e '\x1b[1;34mCompiling day $* solution\x1b[0m'
	$(DC) -of=$@ $(DFLAGS) $(FLAGS_DAY_$@) $<

%: builds/day%
	@echo -e '\x1b[1;32mRunning day $* solution\x1b[0m'
	@./builds/day$* inputs/day$*.input

SHELL:=/bin/bash
time%: builds/day%
	@echo -e '\x1b[1;32mRunning day $* solution (timed)\x1b[0m'
	@time ./builds/day$* inputs/day$*.input

perf%: builds/day%
	@echo -e '\x1b[1;32mRunning day $* solution (timed)\x1b[0m'
	@perf stat -r1000 ./builds/day$* inputs/day$*.input 2>&1 > /dev/null | grep elapsed

clean:
	rm -f $(addprefix builds/day,$(CUR_SLNS))
.PHONY: all
