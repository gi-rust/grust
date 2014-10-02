TESTS_ENVIRONMENT = \
  export G_DEBUG=fatal-criticals;			\
  export GIO_USE_VFS=local;				\
  #

TESTS_ENVIRONMENT_SERIAL = \
  $(TESTS_ENVIRONMENT)		\
  export G_SLICE=debug-blocks;	\
  export RUST_TEST_TASKS=1;

TESTS_ENVIRONMENT_PARALLEL = \
  $(TESTS_ENVIRONMENT)

LOCAL_RUSTCFLAGS := \
  -L $(top_builddir)/src/.libs \
  -L $(top_builddir)/fauxgen/.libs \
  --test

check-local: grust-proof
if TEST_RUN_SERIAL
	$(MAKE) check-serial
endif
if TEST_RUN_PARALLEL
	$(MAKE) check-parallel
endif
if !TEST_RUN_SERIAL
if !TEST_RUN_PARALLEL
	@echo Test runs are disabled per --enable-test-runs=no
endif
endif

check-serial: grust-proof
	@echo Running tests serially
	@$(TESTS_ENVIRONMENT_SERIAL) ./grust-proof

check-parallel: grust-proof
	@echo Running tests in parallel
	@$(TESTS_ENVIRONMENT_PARALLEL) ./grust-proof

.PHONY: check-serial check-parallel

CLEANFILES = grust-proof

include $(top_srcdir)/build/rules.mk

DEPS = \
  $(top_builddir)/src/.libs/libgrust.$(CRATEEXT) \
  $(top_builddir)/fauxgen/.libs/libgrust-Gio-2_0.$(CRATEEXT)

grust-proof: grust-proof.rs $(DEPS)

$(DEPS):
