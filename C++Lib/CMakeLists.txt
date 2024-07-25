cmake_minimum_required(VERSION 3.10)

project(HsgmParser VERSION 1.0)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Include ExternalProject module
include(ExternalProject)

# Define paths for the external project
set(LIBPNG_SOURCE_DIR ${CMAKE_BINARY_DIR}/libpng-src)
set(LIBPNG_BINARY_DIR ${CMAKE_BINARY_DIR}/libpng-build)

# Fetch and build libpng
ExternalProject_Add(
    libpng
    PREFIX ${CMAKE_BINARY_DIR}/libpng
    GIT_REPOSITORY https://github.com/glennrp/libpng.git
    GIT_TAG v1.6.37 # Choose the version you want
    CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
               -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
               -DCMAKE_CXX_STANDARD=${CMAKE_CXX_STANDARD}
    BUILD_COMMAND ${CMAKE_COMMAND} --build <BINARY_DIR> --target install
    INSTALL_DIR ${LIBPNG_SOURCE_DIR}
)

# Define an alias target for the static library
add_library(HsgmParser STATIC
    HsgmParser.cpp
)

# Link against the libpng library
add_dependencies(HsgmParser libpng)
include_directories(${LIBPNG_SOURCE_DIR}/include)
target_link_libraries(HsgmParser ${LIBPNG_SOURCE_DIR}/lib/libpng16.a)

# Add executable for testing
add_executable(HsgmParserTest main.cpp)
add_dependencies(HsgmParserTest libpng)
target_link_libraries(HsgmParserTest HsgmParser ${LIBPNG_SOURCE_DIR}/lib/libpng16.a)
