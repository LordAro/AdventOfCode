CXX?=clang++
CXXFLAGS:=-g -Wall -Wextra -pedantic -std=c++17 $(CXXFLAGS)

CUR_SLNS=$(shell echo $(subst day,, $(basename $(notdir $(wildcard src/day*.cpp)))) | sort -n)

all: $(CUR_SLNS)

perf%: %
	perf stat -r 1000 ./builds/day$* inputs/day$*.input >/dev/null

time%: %
	time ./builds/day$* inputs/day$*.input

run%: %
	./builds/day$* inputs/day$*.input

%:
	@echo -e '\x1b[1;34mCompiling day $@ solution\x1b[0m'
	$(CXX) -o builds/day$@ $(CXXFLAGS) $(CXXFLAGS_DAY_$@) src/day$@.cpp

.PHONY: all
