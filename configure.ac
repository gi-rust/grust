AC_INIT([grust], [0.1.0])

AC_CONFIG_MACRO_DIR([m4])

AM_INIT_AUTOMAKE([foreign no-dependencies])
LT_INIT([disable-static])

AC_CHECK_TOOL([RUSTC], [rustc], [rustc])

PKG_CHECK_MODULES([GLIB], [glib-2.0 gobject-2.0])

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
