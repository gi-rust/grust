define run-rustc
$(RUSTC) $(LOCAL_RUSTCFLAGS) $(RUSTCFLAGS) -o $@ $<
endef

%: $(srcdir)/%.rs
	$(run-rustc)

.libs:
	mkdir -p .libs

clean-local:
	-rm -rf .libs
