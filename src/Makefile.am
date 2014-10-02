LOCAL_RUSTCFLAGS = $(GLIB_L_FLAGS)

all-local: .libs/libgrust.$(CRATEEXT)

include $(top_srcdir)/build/rules.mk

.libs/libgrust.$(CRATEEXT): $(srcdir)/lib.rs $(srcdir)/*.rs | .libs
	$(run-rustc)
