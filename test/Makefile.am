TESTS = grust-proof

TESTS_ENVIRONMENT = \
  export GIO_USE_VFS=local; \
  export LD_LIBRARY_PATH=$(top_builddir)/na/.libs; \
  export RUST_THREADS=1;

LOCAL_RUSTCFLAGS := \
  -L $(top_builddir)/na/.libs \
  -L $(top_builddir)/grust/.libs \
  -L $(top_builddir)/fauxgen/.libs --test

$(TESTS): %: $(srcdir)/%.rs
	$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) -o $@ $<
