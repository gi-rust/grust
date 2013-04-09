top_srcdir = ..

include $(top_srcdir)/build/params.mk

RUSTCFLAGS = -L ../grust/$(OUT_DIR) -L ../$(OUT_DIR) --test

$(OUT_DIR)/grust-proof: grust-proof.rs | $(OUT_DIR)
	$(RUSTC) $(RUSTCFLAGS) -o $@ $<

include $(top_srcdir)/build/rules.mk

check: $(OUT_DIR)/grust-proof
	$(OUT_DIR)/grust-proof

clean:
	rm -f $(OUT_DIR)/grust-proof
