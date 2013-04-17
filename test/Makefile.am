TESTS_ENVIRONMENT = \
  export G_DEBUG=fatal-criticals; \
  export GIO_USE_VFS=local; \
  export LD_LIBRARY_PATH=$(top_builddir)/na/.libs; \
  #

LOCAL_RUSTCFLAGS := \
  -L $(top_builddir)/na/.libs \
  -L $(top_builddir)/grust/.libs \
  -L $(top_builddir)/fauxgen/.libs --test

check-local: grust-proof
if TEST_RUN_SERIAL
	@echo Running tests serially
	@$(TESTS_ENVIRONMENT) RUST_THREADS=1 ./grust-proof
endif
if TEST_RUN_PARALLEL
	@echo Running tests in parallel
	@$(TESTS_ENVIRONMENT) ./grust-proof
endif
if !TEST_RUN_SERIAL
if !TEST_RUN_PARALLEL
	@echo Test runs are disabled per --enable-test-runs=no
endif
endif

CLEANFILES = grust-proof

include $(top_srcdir)/build/rules.mk

DEPS = \
  $(top_builddir)/grust/.libs/.built.grust \
  $(top_builddir)/fauxgen/.libs/.built.gio

grust-proof: grust-proof.rs $(DEPS)

$(DEPS):
