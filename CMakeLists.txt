cmake_minimum_required(VERSION 3.22)

set(BIN Bed)
project(${BIN} VERSION 0.0.1 LANGUAGES CXX)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release CACHE STRING "Build type" FORCE)
endif()

# build rust
add_custom_target(
    RunBeforeBuild ALL
    COMMAND python3 contrib/rust.py ${CMAKE_BUILD_TYPE}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    VERBATIM
)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(bed STATIC IMPORTED)
set_target_properties(bed PROPERTIES
    IMPORTED_LOCATION ${CMAKE_SOURCE_DIR}/lib/libbed.a
)

find_package(PkgConfig REQUIRED)
pkg_check_modules(libudev REQUIRED IMPORTED_TARGET libudev)

find_package(Qt6 REQUIRED COMPONENTS Widgets Gui Core)

# include qontrol from git repo
include(FetchContent)
if(NOT DEFINED ENV{OFFLINE} OR "$ENV{OFFLINE}" STREQUAL "false")
    FetchContent_Declare(qontrol
        GIT_REPOSITORY https://github.com/pythcoiner/qontrol.git
        GIT_TAG 4444fc3
    )
else()
    FetchContent_Declare(qontrol
        SOURCE_DIR ./_deps/qontrol-src/
    )
endif()
FetchContent_MakeAvailable(qontrol)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)

set(SOURCES
    src/main.cpp
    src/AppController.cpp
    src/MainWindow.cpp
    src/screens/common.cpp
    src/screens/Decrypt.cpp
    src/screens/Encrypt.cpp
    src/widgets/Key.cpp
)

qt_add_executable(${BIN}
    ${SOURCES}
)

# ensure rust lib is built BEFORE c++
add_dependencies(${BIN} RunBeforeBuild)

add_dependencies(${BIN} 
    qontrol
)

target_include_directories(${BIN}
  PRIVATE
    $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/src>
    $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/lib>
)

find_package(OpenSSL REQUIRED)
target_link_libraries(${BIN} 
    PRIVATE 
    Qt6::Gui 
    Qt6::Core 
    Qt6::Widgets 
    qontrol
    bed
    PkgConfig::libudev
)

set_target_properties(${BIN} PROPERTIES
    WIN32_EXECUTABLE ON
    MACOSX_BUNDLE ON
)

