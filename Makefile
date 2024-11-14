###############################################################################
###                                 Protobuf                                ###
###############################################################################

DOCKER := $(shell which docker)

protoVer=0.14.0
protoImageName=ghcr.io/cosmos/proto-builder:$(protoVer)
protoImage=$(DOCKER) run --rm -v $(CURDIR):/workspace --workdir /workspace $(protoImageName)

proto-code-gen:
    # This runs ./scripts/protocgen-pulsar.sh as well, under the hood.
	@echo "🤖 Generating Rust code from protobuf..."
	@$(protoImage) sh ./scripts/protocgen.sh;
	@echo "✅ Finished code generation!"