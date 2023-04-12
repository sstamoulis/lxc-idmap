#!/bin/sh
set -e

DEBIAN_FRONTEND=noninteractive \
apt-get update && \
apt-get -y install --no-install-recommends lld
