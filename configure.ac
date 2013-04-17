AC_INIT([grust], [0.1.0])

AC_CONFIG_MACRO_DIR([m4])

AM_INIT_AUTOMAKE([foreign no-dependencies])
LT_INIT([disable-static])

AC_ARG_ENABLE([test-runs],
  [AS_HELP_STRING([--enable-test-runs=(serial|parallel)],
    [Select serial or parallel test runs (default=both)])],
  [], [enable_test_runs=both])
test_run_serial=no
test_run_parallel=no
AS_CASE([$enable_test_runs],
  [yes|both],[
    test_run_serial=yes
    test_run_parallel=yes
  ],
  [ser*],[
    test_run_serial=yes
  ],
  [par*],[
    test_run_parallel=yes
  ],
  [no],[],
  [AC_MSG_ERROR([unrecognized parameter value --enable-test-runs=$enable_test_runs])]
)
AM_CONDITIONAL([TEST_RUN_SERIAL],   [test .$test_run_serial = .yes])
AM_CONDITIONAL([TEST_RUN_PARALLEL], [test .$test_run_parallel = .yes])

AC_CHECK_TOOL([RUSTC], [rustc], [rustc])

AC_PROG_CC

AS_IF([test ."$GCC" = .yes], [
  CFLAGS="$CFLAGS -Wall -Werror"
])

PKG_CHECK_MODULES([GLIB], [glib-2.0 >= 2.32 gobject-2.0])

GLIB_L_FLAGS=$($PKG_CONFIG --libs-only-L glib-2.0 gobject-2.0 gio-2.0)
AC_SUBST([GLIB_L_FLAGS])

AC_CONFIG_FILES([
  Makefile
  na/Makefile
  grust/Makefile
  fauxgen/Makefile
  test/Makefile
])
AC_OUTPUT
