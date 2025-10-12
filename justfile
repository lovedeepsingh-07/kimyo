BUILD_DIR := "build"
BINARY_NAME := "kimyo"
SRC_DIR := "src"
INCLUDE_DIR := "include"
CMAKE_BUILD_TYPE := "Debug"

default:
	just -l

build:
	mkdir -p {{BUILD_DIR}}
	cmake -S . -B {{BUILD_DIR}} -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DCMAKE_BUILD_TYPE={{CMAKE_BUILD_TYPE}}
	cmake --build ./{{BUILD_DIR}}

run: build
	./{{BUILD_DIR}}/{{BINARY_NAME}}

fmt:
    find {{SRC_DIR}} {{INCLUDE_DIR}} -regex '.*\.\(c\|h\)' | xargs clang-format -i
