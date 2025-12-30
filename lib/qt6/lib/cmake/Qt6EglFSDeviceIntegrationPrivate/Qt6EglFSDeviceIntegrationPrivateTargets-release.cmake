#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "Qt6::EglFSDeviceIntegrationPrivate" for configuration "Release"
set_property(TARGET Qt6::EglFSDeviceIntegrationPrivate APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(Qt6::EglFSDeviceIntegrationPrivate PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LINK_INTERFACE_MULTIPLICITY_RELEASE "3"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libQt6EglFSDeviceIntegration.a"
  )

list(APPEND _cmake_import_check_targets Qt6::EglFSDeviceIntegrationPrivate )
list(APPEND _cmake_import_check_files_for_Qt6::EglFSDeviceIntegrationPrivate "${_IMPORT_PREFIX}/lib/libQt6EglFSDeviceIntegration.a" )

# Import target "Qt6::EglFSDeviceIntegrationPrivate_resources_1" for configuration "Release"
set_property(TARGET Qt6::EglFSDeviceIntegrationPrivate_resources_1 APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(Qt6::EglFSDeviceIntegrationPrivate_resources_1 PROPERTIES
  IMPORTED_COMMON_LANGUAGE_RUNTIME_RELEASE ""
  IMPORTED_OBJECTS_RELEASE "${_IMPORT_PREFIX}/lib/objects-Release/EglFSDeviceIntegrationPrivate_resources_1/.rcc/qrc_cursor.cpp.o"
  )

list(APPEND _cmake_import_check_targets Qt6::EglFSDeviceIntegrationPrivate_resources_1 )
list(APPEND _cmake_import_check_files_for_Qt6::EglFSDeviceIntegrationPrivate_resources_1 "${_IMPORT_PREFIX}/lib/objects-Release/EglFSDeviceIntegrationPrivate_resources_1/.rcc/qrc_cursor.cpp.o" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
