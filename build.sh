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
OS_NAME=$(uname -s | tr '[:upper:]' '[:lower:]' | sed 's/darwin/Macos/' | sed 's/linux/Linux/' | sed 's/windows/Windows/')
if [ -z "$OS_NAME" ]; then
  echo "❌ Unable to determine OS name. Aborting."
  exit 1
fi

case "$OS_NAME" in
  Macos)
    CPP_LIB="libc++"
    ;;
  Linux)
    CPP_LIB="libstdc++"
    ;;
  Windows)
    CPP_LIB="msvc"
    ;;
  *)
    echo "❌ Unsupported OS: $OS_NAME. Supported: Macos, Linux, Windows."
    exit 1
    ;;
esac
echo "🛠️  Detected OS: $OS_NAME"

export CONAN_HOME="$(pwd)/.conan"
export PATH="$(pwd)/.venv/bin:$PATH"

CONF=$1
if [ -z "$CONF" ]; then
  CONF=--conf=clang-debug
fi

case "$CONF" in
  --conf=clang-debug)
    echo "TEST 1"
    COMPILER_NAME="clang"
    echo "?????"
    COMPILER_PATH="$(command -v clang)"
    COMPILER_CPP_PATH="$(command -v clang++)"
    echo "??"
    COMPILER_VERSION=20
    BUILD_TYPE="Debug"
    PROFILE_PATH="./.conan/profiles/clang-debug"
    echo "TEST 1-1"
    ;;
  --conf=clang-release)
    echo "TEST 2"
    COMPILER_NAME="clang"
    COMPILER_PATH="$(command -v clang)"
    COMPILER_CPP_PATH="$(command -v clang++)"
    COMPILER_VERSION=20
    BUILD_TYPE="Release"
    PROFILE_PATH="./.conan/profiles/clang-release"
    ;;
  --conf=gcc-debug)
    echo "TEST 3"
    COMPILER_NAME="gcc"
    COMPILER_PATH="$(command -v gcc)"
    COMPILER_CPP_PATH="$(command -v g++)"
    COMPILER_VERSION=15
    BUILD_TYPE="Debug"
    PROFILE_PATH="./.conan/profiles/gcc-debug"
    ;;
  --conf=gcc-release)
    echo "TEST 4"
    COMPILER_NAME="gcc"
    COMPILER_PATH="$(command -v gcc)"
    COMPILER_CPP_PATH="$(command -v g++)"
    COMPILER_VERSION=15
    BUILD_TYPE="Release"
    PROFILE_PATH="./.conan/profiles/gcc-release"
    ;;
  *)
    echo "❌ Invalid or unsupported --conf option: $CONF"
    echo "❌ Unknown or missing --conf option(Given: $CONF)"
    echo "   Use one of: --conf=clang-debug, --conf=clang-release, --conf=gcc-debug, --conf=gcc-release"
    exit 1
    ;;
esac
if [[ -z "$COMPILER_PATH" || -z "$COMPILER_CPP_PATH}" ]]; then
  echo "❌ Compiler($COMPILER_PATH or $COMPILER_CPP_PATH) not found in PATH."
  exit 1
fi

echo "🛠️  Using compiler: $COMPILER_NAME"

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
os=${OS_NAME}
arch=$(uname -m | sed 's/x86_64/x86_64/' | sed 's/aarch64/armv8/' | sed 's/armv7l/armv7/' | sed 's/arm64/armv8/')
compiler=${COMPILER_NAME}
build_type=${BUILD_TYPE}
compiler.version=${COMPILER_VERSION}
compiler.libcxx=${CPP_LIB}
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
  --output-folder=build/conan \
  --profile:host="$PROFILE_PATH" \
  --profile:build="$PROFILE_PATH" \
  --build=missing

echo "🔨 Generating build system..."
BUILD_DIR="build"
TOOLCHAIN_FILE="build/conan/conan_toolchain.cmake"

cmake -S . -B "$BUILD_DIR" -DCMAKE_BUILD_TYPE=${BUILD_TYPE} -DCMAKE_TOOLCHAIN_FILE="$TOOLCHAIN_FILE"
cmake --build build
