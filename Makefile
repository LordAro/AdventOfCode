%:
	clang++ -o builds/day$@ -g -Wall -std=c++14 day$@.cpp
	./builds/day$@

4:
	clang++ -o builds/day$@ -g -Wall -lssl -lcrypto -std=c++14 day$@.cpp
	./builds/day$@
