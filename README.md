# kimyo

> weird

```cmake
cmake_minimum_required(VERSION 3.15)
set (CMAKE_CXX_STANDARD 17)

project(project_name)

add_subdirectory(/path/to/kimyo ${CMAKE_BINARY_DIR}/kimyo)
include_directories(${CMAKE_BINARY_DIR}/kimyo)

file(GLOB_RECURSE SRC_FILES src/*.cc)
add_executable(project_name ${SRC_FILES})

target_include_directories(basement PRIVATE
	include
)

target_link_libraries(basement kimyo)
```

```cc
#include "kimyo.h"
#include <iostream>
#include <string>

int main() {
  kimyo::Server server{.address = static_cast<std::string>("localhost"),
                       .port = 8080};
  std::cout << static_cast<std::string>(server.start()) << '\n';
  return 0;
}
```
