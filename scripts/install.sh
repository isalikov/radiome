#!/bin/sh

set -eu

die() {
	printf '%s\n' "$*" >&2
	exit 1
}

have() {
	command -v "$1" >/dev/null 2>&1
}

repo=${RADIOME_REPO:-isalikov/radiome}
version=${RADIOME_VERSION:-latest}
prefix=${RADIOME_PREFIX:-$HOME/.local}
bin_dir=$prefix/bin
release_base=${RADIOME_RELEASE_BASE_URL:-https://github.com/$repo/releases/$version/download}

os=$(uname -s)
arch=$(uname -m)

case "$os" in
	Darwin)
		platform=macos
		;;
	Linux)
		platform=linux
		;;
	*)
		die "Unsupported operating system: $os"
		;;
esac

case "$arch" in
	x86_64|amd64)
		arch=x86_64
		;;
	arm64|aarch64)
		arch=aarch64
		;;
	*)
		die "Unsupported architecture: $arch"
		;;
esac

tmp_dir=$(mktemp -d 2>/dev/null || mktemp -d -t radiome-install)
trap 'rm -rf "$tmp_dir"' EXIT INT TERM

asset="radiome-${platform}-${arch}.tar.gz"
asset_url="$release_base/$asset"

printf '%s\n' "Installing radiome for ${platform}/${arch}"

if curl -fsSL "$asset_url" -o "$tmp_dir/$asset"; then
	tar -xzf "$tmp_dir/$asset" -C "$tmp_dir"
	install -d "$bin_dir"
	install -m 755 "$tmp_dir/radiome" "$bin_dir/radiome"
	printf '%s\n' "Installed to $bin_dir/radiome"
else
	if have cargo; then
		printf '%s\n' "Release asset not found, building from source with cargo"
		cargo install --locked --git "https://github.com/$repo.git" --force --root "$prefix" radiome
		printf '%s\n' "Installed to $bin_dir/radiome"
	else
		die "No release asset found at $asset_url and cargo is not installed"
	fi
fi

case ":$PATH:" in
	*":$bin_dir:"*)
		;;
	*)
		printf '%s\n' "Add $bin_dir to PATH if radiome is not found after installation"
		;;
esac
