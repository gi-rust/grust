%: $(srcdir)/%.rs
	$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) -o $@ $<

.libs/.built.%: $(srcdir)/%.rc | .libs
	$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) --out-dir $(builddir)/.libs $< && \
	touch $@

.libs:
	mkdir -p .libs
