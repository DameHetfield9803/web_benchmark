cmake_minimum_required(VERSION 3.15)
project(crow_app)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Locate Crow provided by Homebrew
find_package(Crow REQUIRED)

add_executable(my_server main.cpp)
target_link_libraries(my_server PRIVATE Crow::Crow)
