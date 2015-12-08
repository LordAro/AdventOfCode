CXX=clang++
FLAGS=-g -Wall -std=c++14

%:
	${CXX} -o builds/day$@ ${FLAGS} day$@.cpp
	./builds/day$@ inputs/day$@.input

4:
	${CXX} -o builds/day$@ ${FLAGS} -lssl -lcrypto day$@.cpp
	./builds/day$@ inputs/day$@.input
