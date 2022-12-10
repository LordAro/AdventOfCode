
FILEEXT=nim
include ../common.mk

DISABLED_HINT_FLAGS=$(patsubst %,--hint[%]:off,Conf Exec Link SuccessX)
ifdef PROFILE
FLAGS=--warnings:on $(DISABLED_HINT_FLAGS) --profiler:on --stacktrace:on
else
ifdef DEBUG
FLAGS=--warnings:on $(DISABLED_HINT_FLAGS) --stacktrace:on
else
FLAGS=--warnings:on $(DISABLED_HINT_FLAGS) -d:release
endif
endif

.PRECIOUS: builds/day%
builds/day%: src/day%.nim Makefile
	@echo -e '\x1b[1;34mCompiling day $* solution\x1b[0m'
	nim compile -o:$@ $(FLAGS) $(FLAGS_DAY_$*) $<

%: builds/day%
