project(lp_engine)
message(STATUS "===============================================")
message(STATUS "🚀 Configuring ${PROJECT_NAME} module...")
message(STATUS "===============================================")

if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_SOURCE_DIR)
  message(FATAL_ERROR "This CMakeLists.txt cannot be built directly. Please use the top-level CMakeLists.txt.")
endif()

configure_file(
  "${CMAKE_CURRENT_SOURCE_DIR}/core/src/version.cpp.in"
  "${CMAKE_CURRENT_SOURCE_DIR}/core/src/version.cpp"
  @ONLY
)

file(GLOB_RECURSE ENGINE_SOURCES
  "core/src/*.cpp"
  "core/src/*.hpp"
)

add_library(${PROJECT_NAME} STATIC ${ENGINE_SOURCES})
add_library(lp::${PROJECT_NAME} ALIAS ${PROJECT_NAME})

target_include_directories(${PROJECT_NAME}
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/core/include>
    $<INSTALL_INTERFACE:core/include>
)

# Install the library
install(TARGETS ${PROJECT_NAME}
  EXPORT ${PROJECT_NAME}Targets
  ARCHIVE DESTINATION lib
  LIBRARY DESTINATION lib
  RUNTIME DESTINATION bin
)
# Install the headers
install(DIRECTORY include/
  DESTINATION include
  FILES_MATCHING PATTERN "*.hpp"
)

configure_file(
  "${CMAKE_CURRENT_SOURCE_DIR}/core/tests/test_version.cpp.in"
  "${CMAKE_CURRENT_SOURCE_DIR}/core/tests/test_version.cpp"
  @ONLY
)
file(GLOB_RECURSE TEST_SOURCES
  "core/tests/*.cpp"
  "core/tests/*.hpp"
)
add_executable(${PROJECT_NAME}_test ${TEST_SOURCES})
target_link_libraries(${PROJECT_NAME}_test
  PRIVATE
    ${PROJECT_NAME}
    gtest::gtest
)

message(STATUS "===============================================")
message(STATUS "✅ Configuring ${PROJECT_NAME} module... Done.")
message(STATUS "===============================================")