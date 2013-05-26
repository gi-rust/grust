%: $(srcdir)/%.rs
	cd $(srcdir) && \
	$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) -o $(abs_builddir)/$@ \
		$(patsubst $(srcdir)/%,%,$<)

.libs/.built.%: $(srcdir)/%.rc | .libs
	cd $(srcdir) && \
	$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) --out-dir $(abs_builddir)/.libs \
		$(patsubst $(srcdir)/%,%,$<) && \
	touch $(abs_builddir)/$@

.libs:
	mkdir -p .libs
