FILEEXT=cpp
include ../common.mk

CXX?=clang++
FLAGS=-g -Wall -Wextra -pedantic -std=c++14
FLAGS_DAY_4=-lssl -lcrypto
FLAGS_DAY_24=-Ofast

builds/day%: src/day%.cpp
	@echo -e '\x1b[1;34mCompiling day $* solution\x1b[0m'
	$(CXX) -o $@ $(FLAGS) $(FLAGS_DAY_$*) $<
