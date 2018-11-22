cmake_minimum_required(VERSION 3.5)
project(axis)
enable_testing()
include(ExternalProject)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -static -s")

set(AXIS_VERSION_MAJOR 1)
set(AXIS_VERSION_MINOR 0)
set(AXIS_VERSION_PATCH 0)

execute_process(
  COMMAND git rev-parse --short HEAD
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
  OUTPUT_VARIABLE "AXIS_BUILD_VERSION"
  ERROR_QUIET
  OUTPUT_STRIP_TRAILING_WHITESPACE
)
execute_process(
  COMMAND date -R
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
  OUTPUT_VARIABLE "AXIS_BUILD_TIME"
  ERROR_QUIET
  OUTPUT_STRIP_TRAILING_WHITESPACE
)
execute_process(
  COMMAND git config user.name
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
  OUTPUT_VARIABLE "AXIS_AUTHOR_NAME"
  ERROR_QUIET
  OUTPUT_STRIP_TRAILING_WHITESPACE
)
execute_process(
  COMMAND git config user.email
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
  OUTPUT_VARIABLE "AXIS_AUTHOR_EMAIL"
  ERROR_QUIET
  OUTPUT_STRIP_TRAILING_WHITESPACE
)

configure_file(
  "${PROJECT_SOURCE_DIR}/src/env.h.in"
  "${PROJECT_BINARY_DIR}/env.h"
)
include_directories(${CMAKE_CURRENT_BINARY_DIR})

set(EXTRA_LIBS
  stdc++fs pthread
)

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

aux_source_directory(src DIR_SRCS)
add_executable(axis ${DIR_SRCS})
target_link_libraries (axis ${EXTRA_LIBS} ${CONAN_LIBS} ${EXTRA_LIBS})
