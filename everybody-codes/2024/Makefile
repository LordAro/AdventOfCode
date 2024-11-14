ifdef DEBUG
BINDIR=builds/debug
else
CARGO_FLAGS=--release
BINDIR=builds/release
endif
FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused

FILEEXT=rs
SRCDIR=src/bin

CUR_SLNS=$(subst quest,, $(basename $(notdir $(wildcard $(SRCDIR)/quest[0-9][0-9].rs))))

.PHONY: all build_all run_all time_all perf_all
all: run_all
build_all: $(addprefix make,$(CUR_SLNS))
run_all: $(addprefix run,$(CUR_SLNS))
time_all: $(addprefix time,$(CUR_SLNS))
perf_all: $(addprefix perf,$(CUR_SLNS))

PERF_COUNT?=1000
perf%: $(BINDIR)/quest%
	@echo -e '\x1b[1;32mRunning quest $* solution (perf)\x1b[0m'
	perf stat -r$(PERF_COUNT) ./$(BINDIR)/quest$* > /dev/null

time%: $(BINDIR)/quest%
	@echo -e '\x1b[1;32mRunning quest $* solution (timed)\x1b[0m'
	@bash -c 'time ./$(BINDIR)/quest$*

run%: $(BINDIR)/quest%
	@echo -e '\x1b[1;32mRunning quest $* solution\x1b[0m'
	./$(BINDIR)/quest$*

$(BINDIR)/quest%: src/bin/quest%.rs
	@echo -e '\x1b[1;34mCompiling quest $* solution\x1b[0m'
	@RUSTFLAGS='$(FLAGS) $(FLAGS_QUEST_$@)' cargo build $(CARGO_FLAGS) --tests --target-dir builds --bin quest$*
	@RUSTFLAGS='$(FLAGS) $(FLAGS_QUEST_$@)' cargo test $(CARGO_FLAGS) --target-dir builds --bin quest$*
