ifdef DEBUG
BINDIR=target/debug
else
CARGO_FLAGS=--release
BINDIR=target/release
endif
FLAGS=-Wwarnings -Wbad-style -Wfuture-incompatible -Wunused

SRCDIR=src/bin

PERF?=perf

CUR_SLNS=$(basename $(notdir $(wildcard $(SRCDIR)/e*_q[0-9][0-9].rs)))

.PHONY: all build_all run_all time_all perf_all
all: run_all
build_all: $(addprefix make_,$(CUR_SLNS))
run_all: $(addprefix run_,$(CUR_SLNS))
time_all: $(addprefix time_,$(CUR_SLNS))
perf_all: $(addprefix perf_,$(CUR_SLNS))
list:
	@echo $(CUR_SLNS)

PERF_COUNT?=1000
perf_%: $(BINDIR)/%
	@echo -e '\x1b[1;32mRunning quest $* solution (perf)\x1b[0m'
	$(PERF) stat -r$(PERF_COUNT) ./$(BINDIR)/$* > /dev/null

time_%: $(BINDIR)/%
	@echo -e '\x1b[1;32mRunning quest $* solution (timed)\x1b[0m'
	@bash -c 'time ./$(BINDIR)/$*'

run_%: $(BINDIR)/%
	@echo -e '\x1b[1;32mRunning quest $* solution\x1b[0m'
	./$(BINDIR)/$*

.PRECIOUS: $(BINDIR)/%
$(BINDIR)/%: src/bin/%.rs $(MAKEFILE_LIST)
	@echo -e '\x1b[1;34mCompiling quest $* solution\x1b[0m'
	@RUSTFLAGS='$(FLAGS) $(FLAGS_QUEST_$@)' cargo build $(CARGO_FLAGS) --tests --bin $*
	@RUSTFLAGS='$(FLAGS) $(FLAGS_QUEST_$@)' cargo test $(CARGO_FLAGS) --bin $*
