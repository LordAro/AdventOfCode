FILEEXT=zig
include ../common.mk

builds/day%: src/day%.zig $(MAKEFILE_LIST) | builds
	zig build-exe $< -femit-bin=$@ -O ReleaseFast

%: builds/day%
