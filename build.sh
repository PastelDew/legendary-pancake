#!/usr/bin/env bash
set -e

echo "🛠️  Checking environment..."
command -v python3 >/dev/null 2>&1 || { echo >&2 "❌ python3 is not installed. Aborting."; exit 1; }
python3 -m ensurepip --default-pip || true
python3 -m pip --version >/dev/null 2>&1 || { echo >&2 "❌ pip is not available. Aborting."; exit 1; }
python3 -m venv .venv || { echo >&2 "❌ Failed to create virtual environment. Aborting."; exit 1; }
source .venv/bin/activate
pip install --upgrade pip
pip install conan

FAKEBIN_DIR=".toolchain-fakebin"
PROFILE_DIR=".conan/profiles"
PROFILE_PATH="$PROFILE_DIR/default_profile.ini"

export CONAN_HOME="$(pwd)/.conan"
export PATH="$(pwd)/.venv/bin:$PATH"

CONF=$1
if [ -z "$CONF" ]; then
  CONF=--conf=clang-debug
fi

case "$CONF" in
  --conf=clang-debug)
    COMPILER_NAME="clang"
    COMPILER_PATH="$(command -v clang)"
    COMPILER_CPP_PATH="$(command -v clang++)"
    COMPILER_VERSION=20
    BUILD_TYPE="Debug"
    PROFILE_PATH="./.conan/profiles/clang-debug"
    ;;
  --conf=clang-release)
    COMPILER_NAME="clang"
    COMPILER_PATH="$(command -v clang)"
    COMPILER_CPP_PATH="$(command -v clang++)"
    COMPILER_VERSION=20
    BUILD_TYPE="Release"
    PROFILE_PATH="./.conan/profiles/clang-release"
    ;;
  --conf=gcc-debug)
    COMPILER_NAME="gcc"
    COMPILER_PATH="$(command -v gcc)"
    COMPILER_CPP_PATH="$(command -v g++)"
    COMPILER_VERSION=15
    BUILD_TYPE="Debug"
    PROFILE_PATH="./.conan/profiles/gcc-debug"
    ;;
  --conf=gcc-release)
    COMPILER_NAME="gcc"
    COMPILER_PATH="$(command -v gcc)"
    COMPILER_CPP_PATH="$(command -v g++)"
    COMPILER_VERSION=15
    BUILD_TYPE="Release"
    PROFILE_PATH="./.conan/profiles/gcc-release"
    ;;
  *)
    echo "❌ Unknown or missing --conf option(Given: $CONF)"
    echo "   Use one of: --conf=clang-debug, --conf=clang-release, --conf=gcc-debug, --conf=gcc-release"
    exit 1
    ;;
esac
if [[ -z "$COMPILER_PATH" || -z "$COMPILER_CPP_PATH}" ]]; then
  echo "❌ Compiler($COMPILER_PATH or $COMPILER_CPP_PATH) not found in PATH." >&2
  exit 1
fi

mkdir -p "$FAKEBIN_DIR"
ln -sf "$COMPILER_PATH" "$FAKEBIN_DIR/cc"
ln -sf "$COMPILER_CPP_PATH" "$FAKEBIN_DIR/c++"
export PATH="$(pwd)/$FAKEBIN_DIR:$PATH"
echo "📝 PATH override applied: $PATH"


mkdir -p "$PROFILE_DIR"
if [ ! -f "$PROFILE_PATH" ]; then
  echo "📝 Creating default profile at $PROFILE_PATH..."
  cat > "$PROFILE_PATH" <<EOF
[settings]
os=$(uname -s | tr '[:upper:]' '[:lower:]' | sed 's/darwin/Macos/' | sed 's/linux/Linux/' | sed 's/windows/Windows/')
arch=x86_64
compiler=${COMPILER_NAME}
build_type=${BUILD_TYPE}
compiler.version=14
compiler.libcxx=libstdc++11
compiler.cppstd=20

[conf]
tools.system.package_manager:mode=install
tools.build:compiler_executables={"cxx": "${COMPILER_CPP_PATH}", "cc": "${COMPILER_PATH}"}
EOF
fi

echo "📦 Installing Conan dependencies with:"
echo "   🔧 Compiler   = $COMPILER_PATH"
echo "   🏗  BuildType = $BUILD_TYPE"
echo "   📁 Profile    = $PROFILE_PATH"
conan install . \
  --output-folder=build \
  --profile:host="$PROFILE_PATH" \
  --profile:build="$PROFILE_PATH" \
  --build=missing

echo "🔨 Generating build system..."
cmake -S . -B build -DCMAKE_BUILD_TYPE=${BUILD_TYPE} -DCMAKE_TOOLCHAIN_FILE=conan_toolchain.cmake
cmake --build build
