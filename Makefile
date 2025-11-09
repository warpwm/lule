SHELL := /bin/bash

MAKEFLAGS := $(filter-out w --print-directory,$(MAKEFLAGS))
MAKEFLAGS += --no-print-directory

PROJECT_NAME := $(shell grep -Po '^name = "\K[^"]+' Cargo.toml)
PROJECT_CAP  := $(shell echo $(PROJECT_NAME) | tr '[:lower:]' '[:upper:]')
LATEST_TAG   ?= $(shell git describe --tags --abbrev=0 2>/dev/null)
TOP_DIR      := $(CURDIR)
BUILD_DIR    := $(TOP_DIR)/target

SHELL := /bin/bash

ifeq ($(PROJECT_NAME),)
$(error Error: project_name not found in Cargo.toml)
endif

$(info ------------------------------------------)
$(info Project: $(PROJECT_NAME))
$(info ------------------------------------------)

.PHONY: build b config c reconfig run r test t help h clean docs release

build:
	@cargo build --release

b: build

config:
	@cargo check

reconfig:
	@cargo clean
	@cargo check

c: config

run:
	@./target/release/$(PROJECT_NAME)

r: run

test:
	@cargo test --verbose

t: test

help:
	@echo
	@echo "Usage: make [target]"
	@echo
	@echo "Available targets:"
	@echo "  build        Build project"
	@echo "  config       Check project configuration"
	@echo "  reconfig     Clean and reconfigure project"
	@echo "  run          Run the main executable"
	@echo "  test         Run tests"
	@echo "  docs         Build documentation (TYPE=mdbook)"
	@echo "  release      Create a new release (TYPE=patch|minor|major)"
	@echo

h : help

clean:
	@echo "Cleaning build directory..."
	@cargo clean
	@echo "Build directory cleaned."

docs:
ifeq ($(TYPE),mdbook)
	@command -v mdbook >/dev/null 2>&1 || { echo "mdbook is not installed. Please install it first."; exit 1; }
	@mdbook build $(TOP_DIR)/book --dest-dir $(TOP_DIR)/docs
	@git add --all && git commit -m "docs: building website/mdbook"
else
	$(error Invalid documentation type. Use 'make docs TYPE=mdbook')
endif

release:
	@if [ -z "$(TYPE)" ]; then \
		echo "Release type not specified. Use 'make release TYPE=[patch|minor|major]'"; \
		exit 1; \
	fi; \
	CURRENT_VERSION=$$(grep '^version = ' Cargo.toml | sed -E 's/version = "(.*)"/\1/'); \
	IFS='.' read -r MAJOR MINOR PATCH <<< "$$CURRENT_VERSION"; \
	case "$(TYPE)" in \
		major) MAJOR=$$((MAJOR+1)); MINOR=0; PATCH=0 ;; \
		minor) MINOR=$$((MINOR+1)); PATCH=0 ;; \
		patch) PATCH=$$((PATCH+1)); ;; \
		*) echo "Invalid release type. Use patch, minor or major."; exit 1 ;; \
	esac; \
	version="$$MAJOR.$$MINOR.$$PATCH"; \
	if [ -n "$(LATEST_TAG)" ]; then \
		changelog=$$(git cliff $(LATEST_TAG)..HEAD --strip all); \
		git cliff --tag $$version $(LATEST_TAG)..HEAD --prepend CHANGELOG.md; \
	else \
		changelog=$$(git cliff --unreleased --strip all); \
		git cliff --tag $$version --unreleased --prepend CHANGELOG.md; \
	fi; \
	sed -i "s/^version = \".*\"/version = \"$$version\"/" Cargo.toml; \
	git add -A && git commit -m "chore(release): prepare for $$version"; \
	echo "$$changelog"; \
	git tag -a $$version -m "$$version" -m "$$changelog"; \
	git push --follow-tags --force --set-upstream origin develop; \
	gh release create $$version --notes "$$changelog"