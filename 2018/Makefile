DC?=gdc
ifdef DEBUG
#DFLAGS=-w -de -debug -g -unittest
DFLAGS=-Wall -Wdeprecated -funittest
else
DFLAGS=-Wall -Wdeprecated -O3 -frelease -fno-debug
#DFLAGS=-w -de -O -frelease -Winline
endif

FILEEXT=d
include ../common.mk

builds/day%: src/day%.d
	@echo -e '\x1b[1;34mCompiling day $* solution\x1b[0m'
	$(DC) -o builds/day$* $(DFLAGS) $(FLAGS_DAY_$*) $<
