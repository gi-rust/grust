%: $(srcdir)/%.rs
	$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) -o $@ $<

.libs/.built.%: $(srcdir)/%.rs | .libs
	$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) --out-dir $(builddir)/.libs $< && \
	touch $@

.libs/.built.%: $(srcdir)/lib.rs | .libs
	$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) --out-dir $(builddir)/.libs $< && \
	touch $@

.libs:
	mkdir -p .libs

clean-local:
	-rm -rf .libs
