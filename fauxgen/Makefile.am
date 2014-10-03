LOCAL_RUSTCFLAGS = \
  -L $(top_builddir)/src/.libs \
  -L $(builddir)/.libs \
  $(GLIB_L_FLAGS)

glib_crate	= .libs/libgrust-GLib-2_0.$(CRATEEXT)
gobject_crate	= .libs/libgrust-GObject-2_0.$(CRATEEXT)
gio_crate	= .libs/libgrust-Gio-2_0.$(CRATEEXT)

fauxgen_CRATES = $(glib_crate) $(gobject_crate) $(gio_crate)

all-local: $(fauxgen_CRATES)

include $(top_srcdir)/build/rules.mk

DEPS = $(top_builddir)/src/.libs/libgrust.$(CRATEEXT)

$(glib_crate): glib.rs $(DEPS) | .libs
	$(run-rustc)

$(gobject_crate): gobject.rs $(glib_crate) $(DEPS) | .libs
	$(run-rustc)

$(gio_crate): gio.rs $(gobject_crate) $(DEPS) | .libs
	$(run-rustc)

$(DEPS):
