cmake_minimum_required(VERSION 3.22)

set(BIN Bed)
project(${BIN} VERSION 0.0.1 LANGUAGES CXX)

# build type
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release CACHE STRING "Build type" FORCE)
endif()

if(NOT TARGET_OS)
    message(FATAL_ERROR "TARGET_OS must be defined")
endif()
message(STATUS "TARGET_OS=${TARGET_OS}")

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# rust lib path
if(TARGET_OS STREQUAL "linux")
    find_library(BED
        NAMES bed
        PATHS "lib/linux/"
        NO_DEFAULT_PATH
        REQUIRED
    )
elseif(TARGET_OS STREQUAL "windows")
    find_library(BED
        NAMES bed
        PATHS "lib/windows/"
        NO_DEFAULT_PATH
        REQUIRED
    )
endif()

if(TARGET_OS STREQUAL "linux")
    find_package(PkgConfig REQUIRED)
    pkg_check_modules(libudev REQUIRED IMPORTED_TARGET libudev)
    set(UDEV_TARGET PkgConfig::libudev)
endif()

find_package(Qt6 REQUIRED COMPONENTS Widgets Gui Core)

# include qontrol from git repo
include(FetchContent)
if(NOT DEFINED ENV{OFFLINE} OR "$ENV{OFFLINE}" STREQUAL "false")
    FetchContent_Declare(qontrol
        GIT_REPOSITORY https://github.com/pythcoiner/qontrol.git
        GIT_TAG 1f2637b
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

add_dependencies(${BIN} qontrol)

target_include_directories(${BIN}
  PRIVATE
    $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/src>
    $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/lib>
)

if(TARGET_OS STREQUAL "windows")
    target_link_libraries(${BIN}
        PRIVATE
        Qt6::Gui
        Qt6::Core
        Qt6::Widgets
        qontrol
        ${BED}
        ws2_32
        bcrypt
        ntdll
        setupapi
    )
else()
    target_link_libraries(${BIN}
        PRIVATE
        Qt6::Gui
        Qt6::Core
        Qt6::Widgets
        qontrol
        ${BED}
        ${UDEV_TARGET}
    )
endif()

set_target_properties(${BIN} PROPERTIES
    WIN32_EXECUTABLE ON
    MACOSX_BUNDLE ON
)

