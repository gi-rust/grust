.PHONY: all
.PHONY: check
.PHONY: clean clean-libs clean-recursive

RECURSIVE_TARGETS = clean-recursive

$(RECURSIVE_TARGETS): %-recursive:
	for dir in $(SUBDIRS); do \
	  $(MAKE) -C $$dir $*; \
	done

clean: clean-libs

clean-libs:
	rm -f $(OUT_DIR)/.built.*
	rm -f $(OUT_DIR)/*.so

$(OUT_DIR)/.built.%: %.rc | $(OUT_DIR)
	$(RUSTC) $(RUSTCFLAGS) --out-dir $(OUT_DIR) $< && touch $@

$(OUT_DIR):
	mkdir -p $(OUT_DIR)
