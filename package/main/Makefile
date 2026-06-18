.PHONY: build build-clean deploy eslint format \
	lint lint-ci readme test test-debug ts-node

BUNX      := bunx
TSC       := $(BUNX) tsc
TSC_ALIAS := $(BUNX) tsc-alias
BIOME     := $(BUNX) biome
ESLINT    := $(BUNX) eslint
JEST      := $(BUNX) jest
TYPEDOC   := $(BUNX) typedoc
GH_PAGES  := $(BUNX) gh-pages
TS_NODE   := $(BUNX) ts-node

ESM_DIR     := module

TSCONFIG     := tsconfig.json

build:
	$(TSC)
	$(TSC_ALIAS) -p $(TSCONFIG)

build-clean:
	rm -rf $(ESM_DIR)
	$(MAKE) build

deploy:
	$(TYPEDOC)
	$(GH_PAGES) -d doc

eslint:
	$(ESLINT) src

format:
	$(BIOME) format . --write

lint:
	$(ESLINT) src --fix
	$(BIOME) check . --write
	$(TSC)

lint-ci:
	$(ESLINT) src
	$(BIOME) ci .
	$(TSC)

readme:
	bun run scripts/generateReadmeFunctionList.ts

test:
	$(JEST) $(JEST_ARGS)

test-debug:
	cd test && $(TSC)

ts-node: build
	$(TS_NODE) --project tmp/tsconfig.json tmp/src/index.ts
