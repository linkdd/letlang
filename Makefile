.PHONY: dist
dist:
	@bash scripts/dist.sh

.PHONY: examples/dummy
examples/dummy: dist
	@./dist/bin/letlangc.exe --type=exe -r ./dist/lib ./examples/dummy.let
	@./dummy.exe

.PHONY: examples/hello
examples/hello: dist
	@./dist/bin/letlangc.exe --type=lib -r ./dist/lib ./lib/stdio.let
	@./dist/bin/letlangc.exe --type=exe -r ./dist/lib ./examples/hello.let -L. -lstdio
	@./hello.exe
