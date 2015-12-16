CXX=clang++
FLAGS=-Wall -Wextra -pedantic -std=c++14

CUR_SLNS=$(shell ls src/day* | cut -d/ -f2 | cut -d. -f1 | cut -dy -f2 | sort -n)

all: ${CUR_SLNS}

%:
	${CXX} -o builds/day$@ ${FLAGS} src/day$@.cpp
	./builds/day$@ inputs/day$@.input

4:
	${CXX} -o builds/day$@ ${FLAGS} -lssl -lcrypto src/day$@.cpp
	./builds/day$@ inputs/day$@.input

.PHONY: all
