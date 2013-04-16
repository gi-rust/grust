LOCAL_RUSTCFLAGS = -L $(top_builddir)/na/.libs $(GLIB_L_FLAGS)

all-local: .libs/.built.grust

include $(top_srcdir)/build/rules.mk

DEPS = $(top_builddir)/na/libgrustna.la

.libs/.built.grust: grust.rc *.rs $(DEPS)

$(DEPS):
