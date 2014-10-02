AC_INIT([grust], [0.1.0])

AC_CONFIG_MACRO_DIR([m4])

AM_INIT_AUTOMAKE([foreign no-dependencies])

AM_EXTRA_RECURSIVE_TARGETS([check-serial check-parallel])

AC_ARG_ENABLE([test-runs],
  [AS_HELP_STRING([--enable-test-runs=(serial|parallel)],
    [Perform serial or parallel test runs at check (default=both)])],
  [], [enable_test_runs=both])
test_run_serial=no
test_run_parallel=no
AS_CASE([$enable_test_runs],
  [yes|both],[
    test_run_serial=yes
    test_run_parallel=yes
  ],
  [serial],[
    test_run_serial=yes
  ],
  [parallel],[
    test_run_parallel=yes
  ],
  [no|none],[],
  [AC_MSG_ERROR([unrecognized parameter value --enable-test-runs=$enable_test_runs])]
)
AM_CONDITIONAL([TEST_RUN_SERIAL],   [test .$test_run_serial = .yes])
AM_CONDITIONAL([TEST_RUN_PARALLEL], [test .$test_run_parallel = .yes])

AC_CHECK_TOOL([RUSTC], [rustc], [rustc])
AC_ARG_VAR([RUSTCFLAGS], [Rust compiler flags])

AC_ARG_ENABLE([debug],
  [AS_HELP_STRING([--enable-debug],
    [Compile with debug information])],
  [], [enable_debug=no])
AS_IF([test $enable_debug = yes],[
  RUSTCFLAGS="$RUSTCFLAGS -Z debug-info"
  CFLAGS="$CFLAGS -g"
])

PKG_CHECK_EXISTS([glib-2.0 >= 2.36 gobject-2.0 gio-2.0 >= 2.36])

GLIB_L_FLAGS=$($PKG_CONFIG --libs-only-L glib-2.0 gobject-2.0 gio-2.0)
AC_SUBST([GLIB_L_FLAGS])

AC_CONFIG_FILES([
  Makefile
  src/Makefile
  fauxgen/Makefile
  tests/Makefile
])
AC_OUTPUT
