CXX?=clang++
FLAGS=-g -Wall -Wextra -pedantic -std=c++14
FLAGS_DAY_4=-lssl -lcrypto
FLAGS_DAY_24=-Ofast

FILEEXT=cpp
include ../common.mk

%:
	@echo -e '\x1b[1;34mCompiling day $@ solution\x1b[0m'
	$(CXX) -o builds/day$@ $(FLAGS) $(FLAGS_DAY_$@) src/day$@.cpp
