top_srcdir = ..

include $(top_srcdir)/build/params.mk

RUSTCFLAGS = -L ../na

all: $(OUT_DIR)/.built.grust

$(OUT_DIR)/.built.grust: grust.rc *.rs

include $(top_srcdir)/build/rules.mk
