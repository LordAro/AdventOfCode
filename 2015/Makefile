CXX=clang++
FLAGS=-g -Wall -Wextra -pedantic -std=c++14
FLAGS_DAY_4=-lssl -lcrypto
FLAGS_DAY_24=-Ofast

CUR_SLNS=$(shell ls src/day* | cut -d/ -f2 | cut -d. -f1 | cut -dy -f2 | sort -n)

all: ${CUR_SLNS}

perf%: %
	perf stat -r 1000 ./builds/day$* inputs/day$*.input >/dev/null

time%: %
	time ./builds/day$* inputs/day$*.input

%:
	@echo -e '\x1b[1;34mCompiling day $@ solution\x1b[0m'
	${CXX} -o builds/day$@ ${FLAGS} ${FLAGS_DAY_$@} src/day$@.cpp


.PHONY: all
