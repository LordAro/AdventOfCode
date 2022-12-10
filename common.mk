ifeq ($(FILEEXT),)
	$(error "FILEEXT not been set!")
endif
# Echo & sort required for numerical sort
# TODO: renumber solutions to use 2 digits?
CUR_SLNS=$(shell echo $(subst day,, $(basename $(notdir $(wildcard src/day*.$(FILEEXT))))) | sort -n)

.PHONY: all
all: $(CUR_SLNS)

perf%: % inputs/day%.input
	perf stat -r1000 ./builds/day$* inputs/day$*.input >/dev/null

time%: % inputs/day%.input
	time ./builds/day$* inputs/day$*.input

run%: % inputs/day%.input
	./builds/day$* inputs/day$*.input

inputs:
	mkdir -p $@
	@touch $@

# Or it deletes it afterwards!
inputs/day%.input: ../cookie.txt | inputs
	curl --fail-with-body --silent --user-agent "https://github.com/LordAro/AdventOfCode" --cookie ../cookie.txt -o $@ https://adventofcode.com/$(notdir $(CURDIR))/day/$*/input
.PRECIOUS: inputs/day%.input

../cookie.txt:
	@echo "Session token invalid/missing! Get session token from browser cookie storage to proceed (see cookie.example.txt for format)." >&2
	exit 1
.PRECIOUS: ../cookie.txt # shouldn't be necessary, but just to make sure..
