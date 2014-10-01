LOCAL_RUSTCFLAGS = \
  -L $(top_builddir)/grust/.libs \
  -L $(builddir)/.libs \
  $(GLIB_L_FLAGS)

fauxgen_crates = glib gobject gio

.PHONY: $(fauxgen_crates)

all-local: $(fauxgen_crates)

include $(top_srcdir)/build/rules.mk

$(fauxgen_crates): %: .libs/.built.%

DEPS = $(top_builddir)/grust/.libs/.built.grust

.libs/.built.glib: $(DEPS)

.libs/.built.gobject: $(DEPS) | glib

.libs/.built.gio: $(DEPS) | glib gobject

$(DEPS):
