LOCAL_RUSTCFLAGS = $(GLIB_L_FLAGS)

all-local: .libs/.built.grust

include $(top_srcdir)/build/rules.mk

.libs/.built.grust: $(srcdir)/lib.rs $(srcdir)/*.rs $(DEPS)

$(DEPS):
