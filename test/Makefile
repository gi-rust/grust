top_srcdir = ..

include $(top_srcdir)/build/params.mk

RUSTCFLAGS += -L ../na -L ../grust/$(OUT_DIR) -L ../$(OUT_DIR) --test

TEST_ENVIRONMENT = GIO_USE_VFS=local LD_LIBRARY_PATH=../na RUST_THREADS=1

$(OUT_DIR)/grust-proof: grust-proof.rs | $(OUT_DIR)
	$(RUSTC) $(RUSTCFLAGS) -o $@ $<

include $(top_srcdir)/build/rules.mk

check: $(OUT_DIR)/grust-proof
	$(TEST_ENVIRONMENT) $(OUT_DIR)/grust-proof

clean:
	rm -f $(OUT_DIR)/grust-proof
