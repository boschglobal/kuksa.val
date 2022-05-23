#!/bin/bash
#********************************************************************************
# Copyright (c) 2022 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Apache License 2.0 which is available at
# http://www.apache.org/licenses/LICENSE-2.0
#
# SPDX-License-Identifier: Apache-2.0
#*******************************************************************************/
# shellcheck disable=SC2181
# shellcheck disable=SC2086
# shellcheck disable=SC2230

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BASEDIR="$SCRIPT_DIR/.."

print_usage() {
	echo "USAGE: $0 [OPTIONS] TARGETS"
	echo
	echo "Standalone build helper for databroker container."
	echo
	echo "OPTIONS:"
	echo "  -l, --local      local docker import (does not export tar)"
	echo "      --help       show help"
	echo
	echo "TARGETS:"
	echo "  x86_64, aarch64  Target arch to build for, if not set - defaults to multiarch"
	echo
}

LOCAL=0
while [ $# -gt 0 ]; do
	if [ "$1" = "--local" ] || [ "$1" = "-l" ]; then
		LOCAL=1
	elif [ "$1" = "--help" ]; then
		print_usage
		exit 0
	else
		TARGET="$1"
		break
	fi
	shift
done

target_arch() {
	local target="$1"
	case "$target" in
	"x86_64")
		echo "amd64"
		;;
	"aarch64")
		echo "arm64"
		;;
	"")
		echo "multiarch"
		;;
	*)
		return 1
		;;
	esac
	return 0
}

build_release() {
	local arch="$1"

	cd "$BASEDIR" || return 1

	echo "-- Building release for: $arch ..."
	### NOTE: Dockerfile expects vehicle_data_broker/bin_release_databroker_${arch}.tar.gz,
	#         but cross uses container that build in $BASEDIR/target, where the crate is
	if [ "$arch" = "aarch64" ]; then
		# install cross if missing
		[ -z "$(which cross)" ] && cargo install cross
		target_dir="target/aarch64-unknown-linux-gnu"
		#CARGO_TARGET_DIR="./$target_dir"
		RUSTFLAGS='-C link-arg=-s' cross build -j 8 --release --bins --examples --target aarch64-unknown-linux-gnu
	elif [ "$arch" = "x86_64" ]; then
		target_dir="target"
		#CARGO_TARGET_DIR="./$target_dir"
		RUSTFLAGS='-C link-arg=-s' cargo build -j 8 --release --bins --examples
	else
		# FIXME: handle other arch?
		echo "Unsupported arch: $arch"
		return 1
	fi

	echo "-- Checking [$arch] binaries: $(pwd)/${target_dir} ..."
	file "${target_dir}/release/vehicle-data-cli" \
		"${target_dir}/release/vehicle-data-broker" \
		"${target_dir}/release/examples/perf_setter" \
		"${target_dir}/release/examples/perf_subscriber"

	echo "-- Building bin_release_databroker_${arch}.tar.gz ..."

	tar -czvf "vehicle_data_broker/bin_release_databroker_${arch}.tar.gz" \
		"${target_dir}/release/vehicle-data-cli" \
		"${target_dir}/release/vehicle-data-broker" \
		"${target_dir}/release/examples/perf_setter" \
		"${target_dir}/release/examples/perf_subscriber"
}

if [ -z "$TARGET" ] && [ $LOCAL -eq 1 ]; then
	echo "Multiarch archives are not supported for local builds, removing --local flag ..."
	LOCAL=0
fi

DOCKER_ARCH=$(target_arch "$TARGET")
DOCKER_EXPORT="$BASEDIR/${DOCKER_ARCH}_databroker.tar"

cd "$BASEDIR" || exit 1
# DOCKER_BUILDKIT=1 docker build -f vehicle_data_broker/Dockerfile -t databroker .
# Dockerfile requires both bin_vservice_seat_release_* artifacts
echo "-- Building databroker container ..."
if [ "$DOCKER_ARCH" = "multiarch" ] || [ "$DOCKER_ARCH" = "amd64" ]; then
	build_release x86_64 || exit 1
fi
if [ "$DOCKER_ARCH" = "multiarch" ] || [ "$DOCKER_ARCH" = "arm64" ]; then
	build_release aarch64 || exit 1
fi

if [ "$DOCKER_ARCH" = "multiarch" ]; then
	DOCKER_ARGS="--platform linux/amd64,linux/arm64 -t $DOCKER_ARCH/databroker --output type=oci,dest=$DOCKER_EXPORT"
else
	if [ $LOCAL -eq 1 ]; then
		DOCKER_ARGS="--load -t $DOCKER_ARCH/databroker"
		DOCKER_EXPORT="(local)"
	else
		DOCKER_ARGS="--platform linux/$DOCKER_ARCH -t $DOCKER_ARCH/databroker --output type=oci,dest=$DOCKER_EXPORT"
	fi
fi

echo "# docker buildx build $DOCKER_ARGS -f vehicle_data_broker/Dockerfile vehicle_data_broker/"
DOCKER_BUILDKIT=1 docker buildx build $DOCKER_ARGS -f vehicle_data_broker/Dockerfile vehicle_data_broker/
[ $? -eq 0 ] && echo "# Exported $DOCKER_ARCH/databroker in $DOCKER_EXPORT"
