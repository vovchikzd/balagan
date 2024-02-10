cmake_minimum_required(VERSION 3.2)
project(balagan
        VERSION 0.1.0
        LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
add_compile_options(-Wall -Wextra -Wpedantic)

find_package(Qt6 REQUIRED COMPONENTS Widgets)
# qt_standard_project_setup()

qt_add_executable(balagan
    main.cpp
)

target_link_libraries(balagan PRIVATE Qt6::Widgets)
# add_executable(balagan main.cpp)
