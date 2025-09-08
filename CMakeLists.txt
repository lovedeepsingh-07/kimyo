set (PROJECT_NAME kimyo)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED ON)

cmake_minimum_required(VERSION 3.15)
project(${PROJECT_NAME} LANGUAGES C)

set(EXTERNAL_DIR ${CMAKE_CURRENT_LIST_DIR}/../deps)

add_subdirectory(${EXTERNAL_DIR}/expected ${CMAKE_BINARY_DIR}/expected)
include(${CMAKE_CURRENT_LIST_DIR}/lua.cmake)

file(GLOB_RECURSE SRC_FILES src/*.c)

add_executable(${PROJECT_NAME} ${SRC_FILES})

target_include_directories(${PROJECT_NAME} PRIVATE
    include
)

target_link_libraries(${PROJECT_NAME} PRIVATE
	m
	lua
	expected
)
