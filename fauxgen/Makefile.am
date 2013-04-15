LOCAL_RUSTCFLAGS := -L $(top_builddir)/grust/.libs -L .libs $(GLIB_L_FLAGS)

fauxgen_crates = glib gobject gio

.PHONY: $(fauxgen_crates)

all-local: $(fauxgen_crates)

include $(top_srcdir)/build/rules.mk

$(fauxgen_crates): %: .libs/.built.%

grust_dep = $(top_builddir)/grust/.libs/.built.grust

.libs/.built.glib: $(grust_dep)

.libs/.built.gobject: $(grust_dep) | glib

.libs/.built.gio: $(grust_dep) | glib gobject
