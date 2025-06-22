cmake_minimum_required(VERSION 3.21)

# Enable C++17
set(CMAKE_CXX_STANDARD 17)

# Add the cmake modules directory to the module path
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/modules/")

# Conan toolchain
if(NOT CMAKE_TOOLCHAIN_FILE)
    if(EXISTS "${CMAKE_BINARY_DIR}/conan_toolchain.cmake")
        include(${CMAKE_BINARY_DIR}/conan_toolchain.cmake)
    endif()
endif()

# Set the build type if not specified
if(NOT CMAKE_BUILD_TYPE)
  set(default_build_type "Debug")
  message(STATUS "Set the build type to `${default_build_type}` as none was specified.")
  set(CMAKE_BUILD_TYPE ${default_build_type} CACHE STRING "Choose the build type." FORCE)
  set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS
    "Debug" "Release" "MinSizeRel" "RelWithDebInfo")
endif()

# Always export compile commands for tooling support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON CACHE BOOL "Export compile commands for tooling" FORCE)

# Include the versioning information
include(GetGitRevisionDescription)
git_describe(GIT_VERSION --tags)
git_commit_id(PROJECT_LPE_GIT_COMMIT_HASH)
string(REGEX REPLACE "^v([0-9]+)\\..*" "\\1" PROJECT_LPE_VERSION_MAJOR "${GIT_VERSION}")
string(REGEX REPLACE "^v[0-9]+\\.([0-9]+).*" "\\1" PROJECT_LPE_VERSION_MINOR "${GIT_VERSION}")
string(REGEX REPLACE "^v[0-9]+\\.[0-9]+\\.([0-9]+).*" "\\1" PROJECT_LPE_VERSION_PATCH "${GIT_VERSION}")
string(REGEX REPLACE "^v[0-9]+\\.[0-9]+\\.[0-9]+-((alpha|beta)+.[0-9]+).*" "\\1" PROJECT_LPE_VERSION_PRERELEASE "${GIT_VERSION}")
string(REGEX MATCH "[0-9]+\\.[0-9]+\\.[0-9]+-(alpha|beta)\\.[0-9]+" PROJECT_LPE_VERSION "${GIT_VERSION}")
set(PROJECT_LPE_FULL_VERSION
  "${PROJECT_LPE_VERSION_MAJOR}.${PROJECT_LPE_VERSION_MINOR}.${PROJECT_LPE_VERSION_PATCH}-${PROJECT_LPE_GIT_COMMIT_HASH}"
   CACHE INTERNAL "Full version information with pre-release and commit hash" FORCE
)

if(NOT PROJECT_LPE_VERSION) # if not pre-release
  string(REGEX MATCH "[0-9]+\\.[0-9]+\\.[0-9]+" PROJECT_LPE_VERSION "${GIT_VERSION}")
endif()

if(NOT PROJECT_LPE_VERSION)
  set(PROJECT_LPE_VERSION_MAJOR "0")
  set(PROJECT_LPE_VERSION_MINOR "1")
  set(PROJECT_LPE_VERSION_PATCH "0")
  set(PROJECT_LPE_VERSION_PRERELEASE "")
  set(PROJECT_LPE_VERSION "0.1.0")
endif()

if("${PROJECT_LPE_VERSION_PRERELEASE}" STREQUAL ${GIT_VERSION})
  set(PROJECT_LPE_VERSION_PRERELEASE "")
endif()

set(PROJECT_LPE_FULL_VERSION
  "${PROJECT_LPE_VERSION_MAJOR}.${PROJECT_LPE_VERSION_MINOR}.${PROJECT_LPE_VERSION_PATCH}-${PROJECT_LPE_GIT_COMMIT_HASH}"
  CACHE INTERNAL "Full version information with pre-release and commit hash" FORCE
)

string(TIMESTAMP BUILD_DATE "%Y-%b-%d" UTC)
string(TIMESTAMP BUILD_DATE_FULL "%Y-%b-%d %H:%M:%S" UTC)

# Allow user to pass LLVM_HINTS via -DLLVM_HINTS=... externally
if(NOT DEFINED LLVM_HINTS OR LLVM_HINTS STREQUAL "")
  set(LLVM_HINTS "")
  if(APPLE)
    list(APPEND LLVM_HINTS "/opt/homebrew/opt/llvm/bin")
  elseif(UNIX)
    list(APPEND LLVM_HINTS "/usr/bin" "/usr/local/bin")
  elseif(WIN32)
    list(APPEND LLVM_HINTS "C:/Program Files/LLVM/bin")
  endif()
endif()
message(STATUS "LLVM_HINTS: ${LLVM_HINTS}")

option(CLANG_TIDY_FIX "Perform fixes for Clang-Tidy" OFF)
find_program(
  CLANG_TIDY_EXE
  NAMES "clang-tidy"
  HINTS ${LLVM_HINTS}
  DOC "Path to clang-tidy executable"
)
find_program( # clang이 사용되는 것은 아니나, clang-tidy의 올바른 동작을 위해 필요함
  CLANG_EXE
  NAMES "clang"
  HINTS ${LLVM_HINTS}
  DOC "Path to clang executable"
)

# 정적 분석 준비
if(CLANG_TIDY_FIX)
  if(NOT CLANG_TIDY_EXE OR NOT CLANG_EXE)
    message(FATAL_ERROR "Clang-Tidy analysis was requested, but the required executables could not be found.")
  endif()
else()
  if(NOT CLANG_TIDY_EXE)
    message(WARNING "Clang-Tidy could not be found, so analysis will be disabled.")
  endif()
endif()

project(legendary-pancake VERSION ${PROJECT_LPE_VERSION} LANGUAGES CXX)
message(STATUS "============================================================")
message(STATUS "Project Name    : ${PROJECT_NAME}")
message(STATUS "Project Version : ${PROJECT_LPE_VERSION}")
message(STATUS "Project Version (Major.Minor.Patch) : ${PROJECT_LPE_VERSION_MAJOR}.${PROJECT_LPE_VERSION_MINOR}.${PROJECT_LPE_VERSION_PATCH}")
message(STATUS "Project Version (Pre-release) : ${PROJECT_LPE_VERSION_PRERELEASE}")
message(STATUS "Build Date      : ${BUILD_DATE}")
message(STATUS "Build Date Full : ${BUILD_DATE_FULL}")
message(STATUS "Git Commit Hash : ${PROJECT_LPE_GIT_COMMIT_HASH}")
message(STATUS "============================================================")

set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
file(MAKE_DIRECTORY ${CMAKE_LIBRARY_OUTPUT_DIRECTORY})
file(MAKE_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})

# Print the version information
message(STATUS "Project[${PROJECT_NAME}] Build Type: ${CMAKE_BUILD_TYPE}, Version: ${PROJECT_LPE_VERSION}")

# Find dependencies
find_package(GTest REQUIRED)

# Subdirectories
enable_testing()
add_subdirectory(engine)
