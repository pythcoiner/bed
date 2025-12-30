#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "Qt6::QLinuxFbIntegrationPlugin" for configuration "Release"
set_property(TARGET Qt6::QLinuxFbIntegrationPlugin APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(Qt6::QLinuxFbIntegrationPlugin PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/./plugins/platforms/libqlinuxfb.a"
  )

list(APPEND _cmake_import_check_targets Qt6::QLinuxFbIntegrationPlugin )
list(APPEND _cmake_import_check_files_for_Qt6::QLinuxFbIntegrationPlugin "${_IMPORT_PREFIX}/./plugins/platforms/libqlinuxfb.a" )

# Import target "Qt6::QLinuxFbIntegrationPlugin_init" for configuration "Release"
set_property(TARGET Qt6::QLinuxFbIntegrationPlugin_init APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(Qt6::QLinuxFbIntegrationPlugin_init PROPERTIES
  IMPORTED_COMMON_LANGUAGE_RUNTIME_RELEASE ""
  IMPORTED_OBJECTS_RELEASE "${_IMPORT_PREFIX}/./plugins/platforms/objects-Release/QLinuxFbIntegrationPlugin_init/QLinuxFbIntegrationPlugin_init.cpp.o"
  )

list(APPEND _cmake_import_check_targets Qt6::QLinuxFbIntegrationPlugin_init )
list(APPEND _cmake_import_check_files_for_Qt6::QLinuxFbIntegrationPlugin_init "${_IMPORT_PREFIX}/./plugins/platforms/objects-Release/QLinuxFbIntegrationPlugin_init/QLinuxFbIntegrationPlugin_init.cpp.o" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
