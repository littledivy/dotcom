ifeq ($(shell uname), Darwin)
	CACHE_DIR := $(HOME)/Library/Caches
else
	CACHE_DIR := $(HOME)/.cache
endif

default: sync-typst-package webbundle build
	cp web-components/dist/* dist

release: webbundle build-release

install-web-deps:
	cd web-components && pnpm install

webbundle:
	cd web-components && pnpm build

build:
	cargo run -- -v build

build-release:
	cargo run --release -- build --minify

fmt:
	cargo fmt
	deno fmt --ignore=dist/ main.ts
	typstyle -i routes/

sync-typst-package:
	rsync -a ./typst/lib/html-shim $(CACHE_DIR)/typst/packages/preview
