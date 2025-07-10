file(GLOB_RECURSE KIMYO_RS_FILES
    "${CMAKE_CURRENT_SOURCE_DIR}/src/*.rs"
)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --verbose)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --verbose)
    set(TARGET_DIR "release")
endif ()

if (DEFINED ENV{CARGO_BUILD_TARGET})
    set(TARGET_DIR "$ENV{CARGO_BUILD_TARGET}/${TARGET_DIR}")
endif()

set(KIMYO_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libkimyo.a")

set(KIMYO_CXX "${CMAKE_CURRENT_BINARY_DIR}/kimyo.cpp")
add_library(kimyo STATIC ${KIMYO_CXX})
add_custom_command(
	OUTPUT ${KIMYO_CXX}
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} RUSTFLAGS="${RUST_FLAGS}" ${CARGO_CMD}
	COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/kimyo/src/lib.rs.cc ${KIMYO_CXX}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/kimyo/src/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/kimyo.h
	DEPENDS ${KIMYO_RS_FILES}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(kimyo pthread dl ${KIMYO_LIB})

add_test(NAME kimyo_test
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
