
ifeq ($(FILEEXT),)
	$(error "FILEEXT not been set!")
endif

ifeq ($(SRCDIR),)
SRCDIR=src
endif
ifeq ($(BINDIR),)
BINDIR=builds
endif

# Shelling out to echo, tr & sort required for numerical sort
# TODO: renumber solutions to use 2 digits?
CUR_SLNS=$(shell echo $(subst day,, $(basename $(notdir $(wildcard $(SRCDIR)/day*.$(FILEEXT))))) | tr ' ' '\n' | sort -n)

.PHONY: all build_all run_all time_all perf_all
all: run_all
build_all: $(addprefix make,$(CUR_SLNS))
run_all: $(addprefix run,$(CUR_SLNS))
time_all: $(addprefix time,$(CUR_SLNS))
perf_all: $(addprefix perf,$(CUR_SLNS))

PERF_COUNT?=1000
perf%: $(BINDIR)/day% inputs/day%.input
	@echo -e '\x1b[1;32mRunning day $* solution (perf)\x1b[0m'
	perf stat -r$(PERF_COUNT) ./$(BINDIR)/day$* inputs/day$*.input >/dev/null

time%: $(BINDIR)/day% inputs/day%.input
	@echo -e '\x1b[1;32mRunning day $* solution (timed)\x1b[0m'
	@bash -c 'time ./$(BINDIR)/day$* inputs/day$*.input'

run%: $(BINDIR)/day% inputs/day%.input
	@echo -e '\x1b[1;32mRunning day $* solution\x1b[0m'
	./$(BINDIR)/day$* inputs/day$*.input

$(BINDIR)/day%: | $(BINDIR)
$(BINDIR)/day%: $(MAKEFILE_LIST)
.PRECIOUS: $(BINDIR)/day%

make%: $(BINDIR)/day%

clean:
	rm -f $(addprefix $(BINDIR)/day,$(CUR_SLNS))

$(BINDIR) inputs:
	mkdir -p $@
	@touch $@

inputs/day%.input: ../cookie.txt | inputs
	@echo "Fetching input for Y$(notdir $(CURDIR)) D$*..."
	@curl --fail-with-body --silent --user-agent "https://github.com/LordAro/AdventOfCode" --cookie ../cookie.txt -o $@ https://adventofcode.com/$(notdir $(CURDIR))/day/$*/input
.PRECIOUS: inputs/day%.input # Or it deletes it afterwards!

../cookie.txt:
	@echo "Session token invalid/missing! Get session token from browser cookie storage to proceed (see cookie.example.txt for format)." >&2
	exit 1
.PRECIOUS: ../cookie.txt # shouldn't be necessary, but just to make sure..
