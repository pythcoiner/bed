#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "Qt6::QVkKhrDisplayIntegrationPlugin" for configuration "Release"
set_property(TARGET Qt6::QVkKhrDisplayIntegrationPlugin APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(Qt6::QVkKhrDisplayIntegrationPlugin PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/./plugins/platforms/libqvkkhrdisplay.a"
  )

list(APPEND _cmake_import_check_targets Qt6::QVkKhrDisplayIntegrationPlugin )
list(APPEND _cmake_import_check_files_for_Qt6::QVkKhrDisplayIntegrationPlugin "${_IMPORT_PREFIX}/./plugins/platforms/libqvkkhrdisplay.a" )

# Import target "Qt6::QVkKhrDisplayIntegrationPlugin_init" for configuration "Release"
set_property(TARGET Qt6::QVkKhrDisplayIntegrationPlugin_init APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(Qt6::QVkKhrDisplayIntegrationPlugin_init PROPERTIES
  IMPORTED_COMMON_LANGUAGE_RUNTIME_RELEASE ""
  IMPORTED_OBJECTS_RELEASE "${_IMPORT_PREFIX}/./plugins/platforms/objects-Release/QVkKhrDisplayIntegrationPlugin_init/QVkKhrDisplayIntegrationPlugin_init.cpp.o"
  )

list(APPEND _cmake_import_check_targets Qt6::QVkKhrDisplayIntegrationPlugin_init )
list(APPEND _cmake_import_check_files_for_Qt6::QVkKhrDisplayIntegrationPlugin_init "${_IMPORT_PREFIX}/./plugins/platforms/objects-Release/QVkKhrDisplayIntegrationPlugin_init/QVkKhrDisplayIntegrationPlugin_init.cpp.o" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
