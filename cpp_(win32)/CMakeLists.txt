cmake_minimum_required(VERSION 3.29.0)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(fallingSand VERSION 1.0)

add_library(HSLtoRGB STATIC lib/HSLtoRGB/HSLtoRGB.cpp)
target_include_directories(HSLtoRGB PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/lib/HSLtoRGB/include)

add_executable(fallingSand WIN32
    src/fall.cpp
    src/main.cpp)

target_include_directories(fallingSand PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

target_link_libraries(fallingSand PUBLIC HSLtoRGB)
