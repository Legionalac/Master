# Set the minimum required version of CMake
cmake_minimum_required(VERSION 3.10)

# Set the project name
project(exercise1)

# Add the source files
# Replace main.c with your actual C or C++ source files
set(SOURCES exercises1.cpp)

# Create an executable from the source files
add_executable(exercise1 ${SOURCES})

