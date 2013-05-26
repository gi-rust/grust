TESTS_ENVIRONMENT = \
  export G_DEBUG=fatal-criticals;			\
  export GIO_USE_VFS=local;				\
  export LD_LIBRARY_PATH=$(top_builddir)/na/.libs; 	\
  #

TESTS_ENVIRONMENT_SERIAL = \
  $(TESTS_ENVIRONMENT)		\
  export G_SLICE=debug-blocks;	\
  export RUST_THREADS=1;

TESTS_ENVIRONMENT_PARALLEL = \
  $(TESTS_ENVIRONMENT)

LOCAL_RUSTCFLAGS := \
  -L $(abs_top_builddir)/na/.libs \
  -L $(abs_top_builddir)/grust/.libs \
  -L $(abs_top_builddir)/fauxgen/.libs \
  --link-args -lgrustna \
  --link-args '$(strip $(GLIB_LIBS))' \
  --link-args '$(strip $(GIO_LIBS))' \
  --test

check-local: grust-proof
if TEST_RUN_SERIAL
	@echo Running tests serially
	@$(TESTS_ENVIRONMENT_SERIAL) ./grust-proof
endif
if TEST_RUN_PARALLEL
	@echo Running tests in parallel
	@$(TESTS_ENVIRONMENT_PARALLEL) ./grust-proof
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
