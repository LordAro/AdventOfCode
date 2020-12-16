DISABLED_HINT_FLAGS=$(patsubst %,--hint[%]:off,Conf Exec Link SuccessX)
ifdef PROFILE
FLAGS=--warnings:on $(DISABLED_HINT_FLAGS) --profiler:on --stacktrace:on
else
ifdef DEBUG
FLAGS=--warnings:on $(DISABLED_HINT_FLAGS) --stacktrace:on
else
FLAGS=--warnings:on $(DISABLED_HINT_FLAGS) -d:release
endif
endif

CUR_SLNS=$(sort $(patsubst src/day%.nim,%,$(wildcard src/day*.nim)))

all: $(CUR_SLNS)

.PRECIOUS: builds/day%
builds/day%: src/day%.nim Makefile
	@echo -e '\x1b[1;34mCompiling day $* solution\x1b[0m'
	nim compile -o:$@ $(FLAGS) $(FLAGS_DAY_$*) $<

%: builds/day%
	@echo -e '\x1b[1;32mRunning day $* solution\x1b[0m'
	@./builds/day$* inputs/day$*.input

time%: builds/day%
	bash -c 'time for i in {1..1000}; do ./builds/day$* inputs/day$*.input; done' > /dev/null

time: $(addprefix time,$(CUR_SLNS))

PERF_COUNT?=10000

perf%: builds/day%
	@echo -e '\x1b[1;32mRunning day $* solution (perf)\x1b[0m'
	@perf stat -r$(PERF_COUNT) ./builds/day$* inputs/day$*.input 2>&1 > /dev/null

clean:
	rm -f $(addprefix builds/day, $(CUR_SLNS))

.PHONY: all time
