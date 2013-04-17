%: $(srcdir)/%.rs
	$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) -o $@ $<

.libs/.built.%: $(srcdir)/%.rc | .libs
	$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) --out-dir .libs $< \
	  && touch $@

.libs:
	mkdir -p .libs
