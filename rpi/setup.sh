#!/usr/bin/env sh

set -eu
export DEBIAN_FRONTEND=noninteractive

# Update the system
sudo apt update
sudo apt upgrade -q -y
sudo apt dist-upgrade -q -y
