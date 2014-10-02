LOCAL_RUSTCFLAGS = \
  -L $(top_builddir)/src/.libs \
  -L $(builddir)/.libs \
  $(GLIB_L_FLAGS)

fauxgen_CRATES = GLib-2_0 GObject-2_0 Gio-2_0

.PHONY: $(fauxgen_CRATES)

all-local: $(fauxgen_CRATES)

include $(top_srcdir)/build/rules.mk

$(fauxgen_CRATES): %: .libs/libgrust-%.$(CRATEEXT)

DEPS = $(top_builddir)/src/.libs/libgrust.$(CRATEEXT)

.libs/libgrust-GLib-2_0.$(CRATEEXT): glib.rs $(DEPS) | .libs
	$(run-rustc)

.libs/libgrust-GObject-2_0.$(CRATEEXT): gobject.rs $(DEPS) | .libs GLib-2_0
	$(run-rustc)

.libs/libgrust-Gio-2_0.$(CRATEEXT): gio.rs $(DEPS) | .libs GLib-2_0 GObject-2_0
	$(run-rustc)

$(DEPS):
