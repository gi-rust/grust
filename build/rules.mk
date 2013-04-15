.libs/.built.%: %.rc | .libs
	$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) --out-dir .libs $< \
	  && touch $@

.libs:
	mkdir -p .libs
