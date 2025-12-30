#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "Qt6::QXdgDesktopPortalThemePlugin" for configuration "Release"
set_property(TARGET Qt6::QXdgDesktopPortalThemePlugin APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(Qt6::QXdgDesktopPortalThemePlugin PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/./plugins/platformthemes/libqxdgdesktopportal.a"
  )

list(APPEND _cmake_import_check_targets Qt6::QXdgDesktopPortalThemePlugin )
list(APPEND _cmake_import_check_files_for_Qt6::QXdgDesktopPortalThemePlugin "${_IMPORT_PREFIX}/./plugins/platformthemes/libqxdgdesktopportal.a" )

# Import target "Qt6::QXdgDesktopPortalThemePlugin_init" for configuration "Release"
set_property(TARGET Qt6::QXdgDesktopPortalThemePlugin_init APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(Qt6::QXdgDesktopPortalThemePlugin_init PROPERTIES
  IMPORTED_COMMON_LANGUAGE_RUNTIME_RELEASE ""
  IMPORTED_OBJECTS_RELEASE "${_IMPORT_PREFIX}/./plugins/platformthemes/objects-Release/QXdgDesktopPortalThemePlugin_init/QXdgDesktopPortalThemePlugin_init.cpp.o"
  )

list(APPEND _cmake_import_check_targets Qt6::QXdgDesktopPortalThemePlugin_init )
list(APPEND _cmake_import_check_files_for_Qt6::QXdgDesktopPortalThemePlugin_init "${_IMPORT_PREFIX}/./plugins/platformthemes/objects-Release/QXdgDesktopPortalThemePlugin_init/QXdgDesktopPortalThemePlugin_init.cpp.o" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
