FLAGS=--warnings:on --hint[Conf]:off --hint[Link]:off --hint[SuccessX]:off -d:release

CUR_SLNS=$(sort $(patsubst src/day%.nim,%,$(wildcard src/day*.nim)))

all: $(CUR_SLNS)

.PRECIOUS: builds/day%
builds/day%: src/day%.nim
	@echo -e '\x1b[1;34mCompiling day $* solution\x1b[0m'
	nim compile -o:$@ $(FLAGS) $(FLAGS_DAY_$*) $<

%: builds/day%
	@echo -e '\x1b[1;32mRunning day $* solution\x1b[0m'
	@./builds/day$* inputs/day$*.input

time%: builds/day%
	bash -c 'time for i in (1..1000); do ./builds/day$* inputs/day$*.input; done' > /dev/null

time: $(addprefix time,$(CUR_SLNS))

clean:
	rm -f $(addprefix builds/day, $(CUR_SLNS))

.PHONY: all time
