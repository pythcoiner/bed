#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "Qt6::QNetworkManagerNetworkInformationPlugin" for configuration "Release"
set_property(TARGET Qt6::QNetworkManagerNetworkInformationPlugin APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(Qt6::QNetworkManagerNetworkInformationPlugin PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/./plugins/networkinformation/libqnetworkmanager.a"
  )

list(APPEND _cmake_import_check_targets Qt6::QNetworkManagerNetworkInformationPlugin )
list(APPEND _cmake_import_check_files_for_Qt6::QNetworkManagerNetworkInformationPlugin "${_IMPORT_PREFIX}/./plugins/networkinformation/libqnetworkmanager.a" )

# Import target "Qt6::QNetworkManagerNetworkInformationPlugin_init" for configuration "Release"
set_property(TARGET Qt6::QNetworkManagerNetworkInformationPlugin_init APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(Qt6::QNetworkManagerNetworkInformationPlugin_init PROPERTIES
  IMPORTED_COMMON_LANGUAGE_RUNTIME_RELEASE ""
  IMPORTED_OBJECTS_RELEASE "${_IMPORT_PREFIX}/./plugins/networkinformation/objects-Release/QNetworkManagerNetworkInformationPlugin_init/QNetworkManagerNetworkInformationPlugin_init.cpp.o"
  )

list(APPEND _cmake_import_check_targets Qt6::QNetworkManagerNetworkInformationPlugin_init )
list(APPEND _cmake_import_check_files_for_Qt6::QNetworkManagerNetworkInformationPlugin_init "${_IMPORT_PREFIX}/./plugins/networkinformation/objects-Release/QNetworkManagerNetworkInformationPlugin_init/QNetworkManagerNetworkInformationPlugin_init.cpp.o" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
