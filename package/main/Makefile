.PHONY: build build-cjs build-babel build-cjs-babel build-full \
	build-clean build-clean-full deploy eslint format \
	lint lint-ci test test-debug ts-node

BUNX      := bunx
TSC       := $(BUNX) tsc
TSC_ALIAS := $(BUNX) tsc-alias
BABEL     := $(BUNX) babel
BIOME     := $(BUNX) biome
ESLINT    := $(BUNX) eslint
JEST      := $(BUNX) jest
TYPEDOC   := $(BUNX) typedoc
GH_PAGES  := $(BUNX) gh-pages
TS_NODE   := $(BUNX) ts-node

ESM_DIR     := module
CJS_DIR     := common-module
ES5_SUBDIR  := es5
ESM_ES5_DIR := $(ESM_DIR)/$(ES5_SUBDIR)
CJS_ES5_DIR := $(CJS_DIR)/$(ESM_DIR)/$(ES5_SUBDIR)

TSCONFIG     := tsconfig.json
TSCONFIG_CJS := tsconfig.cjs.json

build:
	$(TSC)
	$(TSC_ALIAS) -p $(TSCONFIG)

build-cjs:
	$(TSC) -p $(TSCONFIG_CJS)
	$(TSC_ALIAS) -p $(TSCONFIG_CJS)
	bun run cjs.build.mjs
	cp LICENSE README.md .npmignore $(CJS_DIR)/

build-babel:
	$(BABEL) src --extensions '.ts' --out-dir $(ESM_ES5_DIR)
	$(TSC) --emitDeclarationOnly --outDir $(ESM_ES5_DIR)
	$(TSC_ALIAS) -p $(TSCONFIG) --outDir $(ESM_ES5_DIR)

build-cjs-babel:
	$(BABEL) src --extensions '.ts' --out-dir $(CJS_ES5_DIR)
	$(TSC) --emitDeclarationOnly --outDir $(CJS_ES5_DIR)
	$(TSC_ALIAS) -p $(TSCONFIG_CJS) --outDir $(CJS_ES5_DIR)

build-full: build build-cjs build-babel build-cjs-babel

build-clean:
	rm -rf $(ESM_DIR)
	$(MAKE) build

build-clean-full:
	rm -rf $(ESM_DIR) $(CJS_DIR)
	$(MAKE) build-full

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

test:
	$(JEST) $(JEST_ARGS)

test-debug:
	cd test && $(TSC)

ts-node: build
	$(TS_NODE) --project tmp/tsconfig.json tmp/src/index.ts
