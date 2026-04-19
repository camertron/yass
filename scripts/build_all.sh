# /bin/bash

# This script is for testing the build process locally. Actual releases
# are done by creating a new release on GitHub.

ruby_versions="~> 3.2,~> 3.3,~> 3.4,~> 4.0"

#### LINUX ####

bundle exec rb-sys-dock \
  --platform x86_64-linux \
  --ruby-versions "$ruby_versions" \
  --build

bundle exec rb-sys-dock \
  --platform aarch64-linux \
  --ruby-versions "$ruby_versions" \
  --build

#### MAC OS ####

bundle exec rb-sys-dock \
  --platform x86_64-darwin \
  --ruby-versions "$ruby_versions" \
  --build

bundle exec rb-sys-dock \
  --platform arm64-darwin \
  --ruby-versions "$ruby_versions" \
  --build

#### WINDOWS ####

docker build \
  --platform linux/amd64 \
  -f dockerfiles/windows.dockerfile \
  -t camertron/rbsys-x64-mingw-ucrt:latest \
  dockerfiles/

RCD_IMAGE=camertron/rbsys-x64-mingw-ucrt:latest \
  bundle exec rb-sys-dock \
  --platform x64-mingw-ucrt \
  --ruby-versions "$ruby_versions" \
  --build
