SHELL       := /bin/bash
.SHELLFLAGS += -e -u -o pipefail

export REPOSITORY_ROOT := $(CURDIR)

.PHONY: ALWAYS_RUN

default: ALWAYS_RUN run

run: ALWAYS_RUN
	@ $(REPOSITORY_ROOT)/scripts/combine.sh content

test: ALWAYS_RUN
	@ cargo run -- tests/env.sample.yaml tests/env.sample.md tests/env.sample.env
