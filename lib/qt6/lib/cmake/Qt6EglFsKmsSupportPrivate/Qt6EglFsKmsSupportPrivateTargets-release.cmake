#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "Qt6::EglFsKmsSupportPrivate" for configuration "Release"
set_property(TARGET Qt6::EglFsKmsSupportPrivate APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(Qt6::EglFsKmsSupportPrivate PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LINK_INTERFACE_MULTIPLICITY_RELEASE "3"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libQt6EglFsKmsSupport.a"
  )

list(APPEND _cmake_import_check_targets Qt6::EglFsKmsSupportPrivate )
list(APPEND _cmake_import_check_files_for_Qt6::EglFsKmsSupportPrivate "${_IMPORT_PREFIX}/lib/libQt6EglFsKmsSupport.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
