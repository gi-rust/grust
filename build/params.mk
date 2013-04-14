BUILD_OPTIONS =

RUSTC = rustc

OUT_DIR = obj

ifneq (,$(findstring debug,$(BUILD_OPTIONS)))
  RUSTCFLAGS += -Z debug-info
endif
