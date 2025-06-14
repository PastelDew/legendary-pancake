#!/usr/bin/env bash
set -euo pipefail

# ----------------------
# Configurable Defaults
# ----------------------
BUILD_TYPE="Debug"
PROFILE_NAME="clang-debug"
FAKEBIN_DIR=".toolchain-fakebin"
CONAN_PROFILE_DIR=".conan/profiles"
OS_NAME="$(uname -s)"

# ----------------------
# Parse arguments
# ----------------------
if [[ $# -ge 1 ]]; then
  PROFILE_NAME="$1"
fi
if [[ "$PROFILE_NAME" == *"release"* ]]; then
  BUILD_TYPE="Release"
fi

# ----------------------
# Determine OS and package manager
# ----------------------
install_pkg() {
  if [[ "$OS_NAME" == "Linux" ]]; then
    if command -v apt &>/dev/null; then
      sudo apt update && sudo apt install -y "$@"
    elif command -v pacman &>/dev/null; then
      sudo pacman -Sy --noconfirm "$@"
    fi
  elif [[ "$OS_NAME" == "Darwin" ]]; then
    if ! command -v brew &>/dev/null; then
      echo "[ERROR] Homebrew not found. Please install it manually from https://brew.sh" >&2
      exit 1
    fi
    brew install "$@"
  elif [[ "$OS_NAME" == MINGW* || "$OS_NAME" == MSYS* || "$OS_NAME" == CYGWIN* ]]; then
    echo "[ERROR] Please install dependencies manually on Windows." >&2
    exit 1
  fi
}

# ----------------------
# Check and install python3 & pip if missing
# ----------------------
if ! command -v python3 &>/dev/null; then
  echo "[INFO] python3 not found. Installing..."
  install_pkg python3
fi
if ! command -v pip3 &>/dev/null; then
  echo "[INFO] pip3 not found. Installing..."
  install_pkg python3-pip
fi

# ----------------------
# Check and install conan if missing (in venv)
# ----------------------
if [[ ! -d ".venv" ]]; then
  echo "[INFO] Creating Python virtual environment..."
  python3 -m venv .venv
fi
source .venv/bin/activate
if ! command -v conan &>/dev/null; then
  echo "[INFO] Installing Conan in venv..."
  pip install conan
fi

# ----------------------
# Detect clang path
# ----------------------
CLANGXX_PATH="$(command -v clang++)"
CLANG_PATH="$(command -v clang)"
if [[ -z "$CLANGXX_PATH" || -z "$CLANG_PATH" ]]; then
  echo "[ERROR] clang or clang++ not found in PATH." >&2
  exit 1
fi

# ----------------------
# Detect current OS for Conan profile
# ----------------------
case "$OS_NAME" in
  Linux*) CONAN_OS="Linux";;
  Darwin*) CONAN_OS="Macos";;
  MINGW*|MSYS*|CYGWIN*) CONAN_OS="Windows";;
  *) echo "[ERROR] Unknown OS: $OS_NAME" >&2; exit 1;;
esac

# ----------------------
# Create default profile if missing
# ----------------------
mkdir -p "$CONAN_PROFILE_DIR"
PROFILE_PATH="$CONAN_PROFILE_DIR/$PROFILE_NAME"
if [[ ! -f "$PROFILE_PATH" ]]; then
  echo "[INFO] Creating default profile: $PROFILE_NAME"
  CLANG_VERSION=$(clang --version | grep -oE 'clang version ([0-9]+)' | awk '{print $3}' | head -1)
  cat > "$PROFILE_PATH" <<EOF
[settings]
os=$CONAN_OS
arch=x86_64
compiler=clang
compiler.version=$CLANG_VERSION
build_type=${BUILD_TYPE}
compiler.libcxx=libstdc++11
compiler.cppstd=20

[conf]
tools.system.package_manager:mode=install
tools.build:compiler_executables={"cxx":"$CLANGXX_PATH","cc":"$CLANG_PATH"}
EOF
fi

# ----------------------
# Setup fakebin (force Conan to use clang++)
# ----------------------
mkdir -p "$FAKEBIN_DIR"
ln -sf "$CLANGXX_PATH" "$FAKEBIN_DIR/c++"
ln -sf "$CLANG_PATH" "$FAKEBIN_DIR/cc"
export PATH="$(pwd)/$FAKEBIN_DIR:$PATH"
echo "[INFO] PATH override applied: $PATH"

# ----------------------
# Conan install
# ----------------------
BUILD_DIR="build/$PROFILE_NAME"
echo "[INFO] Conan 의존성 설치..."
conan install . \
  --output-folder="$BUILD_DIR" \
  --profile="$PROFILE_PATH" \
  --build=missing

# ----------------------
# CMake configure & build
# ----------------------
echo "[INFO] CMake 설정 및 빌드..."
cmake -S . -B "$BUILD_DIR" -DCMAKE_BUILD_TYPE="$BUILD_TYPE"
cmake --build "$BUILD_DIR"

