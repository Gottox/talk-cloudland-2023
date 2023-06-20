######################################################################
# @author      : Enno Boland (mail@eboland.de)
# @file        : Makefile
# @created     : Tuesday Jun 20, 2023 11:15:02 CEST
######################################################################

docker-build:
	podman build -v /work/tox/cl-cargo:/cargo .

nodescript-crd.yaml:
	cargo run -- crd > $@

run:
	cargo run

clean:
	rm -f nodescript-crd.yaml

crd: nodescript-crd.yaml

.PHONY: docker-build run clean crd
