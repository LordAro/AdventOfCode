CXX=clang++
FLAGS=-Wall -Wextra -pedantic -std=c++14

%:
	${CXX} -o builds/day$@ ${FLAGS} src/day$@.cpp
	./builds/day$@ inputs/day$@.input

4:
	${CXX} -o builds/day$@ ${FLAGS} -lssl -lcrypto src/day$@.cpp
	./builds/day$@ inputs/day$@.input
