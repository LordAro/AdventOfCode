CXX=clang++
FLAGS=-Wall -Wextra -pedantic -std=c++14
FLAGS_DAY_4=-lssl -lcrypto

CUR_SLNS=$(shell ls src/day* | cut -d/ -f2 | cut -d. -f1 | cut -dy -f2 | sort -n)

all: ${CUR_SLNS}

%:
	@echo -e "\e[1;34mCompiling day $@ solution\e[0m"
	@${CXX} -o builds/day$@ ${FLAGS} ${FLAGS_DAY_$@} src/day$@.cpp
	@./builds/day$@ inputs/day$@.input

.PHONY: all
