CXX?=clang++
CXXFLAGS:=-g -Wall -Wextra -pedantic -std=c++17 -march=native $(CXXFLAGS)

FILEEXT=cpp
include ../common.mk

%:
	@echo -e '\x1b[1;34mCompiling day $@ solution\x1b[0m'
	$(CXX) -o builds/day$@ $(CXXFLAGS) $(CXXFLAGS_DAY_$@) src/day$@.cpp
