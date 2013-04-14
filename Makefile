top_srcdir = .

include $(top_srcdir)/build/params.mk

RUSTCFLAGS += -L grust/$(OUT_DIR) -L $(OUT_DIR)

SUBDIRS = grust test

sham_crates = glib gobject gio

.PHONY: grust $(sham_crates)

all: grust $(sham_crates)

include $(top_srcdir)/build/rules.mk

grust:
	$(MAKE) -C grust

$(sham_crates): %: $(OUT_DIR)/.built.%

$(OUT_DIR)/.built.glib: | grust

$(OUT_DIR)/.built.gobject: | grust glib

$(OUT_DIR)/.built.gio: | grust glib gobject

check: all
	$(MAKE) -C test check

clean: clean-recursive
