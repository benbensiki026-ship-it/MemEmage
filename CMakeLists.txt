cmake_minimum_required(VERSION 3.15)
project(MemeProcessor VERSION 1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Source files
set(SOURCES
    meme_processor.cpp
)

set(HEADERS
    meme_processor.hpp
)

# Create shared library for FFI with Rust
add_library(meme_processor SHARED ${SOURCES} ${HEADERS})

# Set output directory
set_target_properties(meme_processor PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

# Include directories
target_include_directories(meme_processor PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
)

# Installation
install(TARGETS meme_processor
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
)

install(FILES ${HEADERS} DESTINATION include/mememage)

# Optional: Build example executable
add_executable(meme_example example.cpp)
target_link_libraries(meme_example meme_processor)
