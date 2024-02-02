LETLANGC := cargo run --bin letlangc --

.PHONY: dummy
dummy:
	@$(LETLANGC) --type=exe --rpath ./sources/runtime ./examples/dummy.let
	@./dummy.exe

.PHONY: hello
hello:
	@$(LETLANGC) --type=lib --rpath ./sources/runtime ./lib/stdio.let
	@$(LETLANGC) --type=exe --rpath ./sources/runtime ./examples/hello.let -L. -lstdio
	@./hello.exe
