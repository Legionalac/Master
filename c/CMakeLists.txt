cmake_minimum_required(VERSION 3.16)
project(Game LANGUAGES CXX)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Add this line, to enable compile command export
set(CMAKE_EXPORT_COMPILE_COMMANDS ON CACHE INTERNAL "")
set (CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

add_subdirectory(exercise2)
# Other stuff...
