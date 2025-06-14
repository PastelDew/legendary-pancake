cmake_minimum_required(VERSION 3.21)
project(legendary-pancake-engine)

# Enable C++17
set(CMAKE_CXX_STANDARD 17)

# Conan toolchain
include(${CMAKE_BINARY_DIR}/conan_toolchain.cmake OPTIONAL)

# Subdirectories
add_subdirectory(engine)

