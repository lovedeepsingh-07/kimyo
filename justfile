build_dir := "build"
binary_name := "kimyo"
src_dir := "src"
include_dir := "include"
cmake_build_type := "Debug"

default:
	just -l

build:
	mkdir -p {{build_dir}}
	cmake -S . -B {{build_dir}} -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DCMAKE_BUILD_TYPE={{cmake_build_type}}
	cmake --build ./{{build_dir}}

run: build
	./{{build_dir}}/{{binary_name}}

fmt:
    find {{src_dir}} {{include_dir}} -regex '.*\.\(c\|h\)' | xargs clang-format -i
