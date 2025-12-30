QT.core.VERSION = 6.6.3
QT.core.name = QtCore
QT.core.module = Qt6Core
QT.core.libs = $$QT_MODULE_LIB_BASE
QT.core.ldflags = 
QT.core.includes = $$QT_MODULE_INCLUDE_BASE $$QT_MODULE_INCLUDE_BASE/QtCore
QT.core.frameworks = 
QT.core.bins = $$QT_MODULE_BIN_BASE
QT.core.depends =  
QT.core.uses = libatomic
QT.core.module_config = v2 staticlib
QT.core.CONFIG = moc resources
QT.core.DEFINES = QT_CORE_LIB
QT.core.enabled_features = clock-monotonic cxx11_future cxx17_filesystem eventfd glib inotify std-atomic64 mimetype regularexpression sharedmemory shortcut systemsemaphore xmlstream xmlstreamreader xmlstreamwriter textdate datestring process processenvironment temporaryfile library settings filesystemwatcher filesystemiterator itemmodel proxymodel sortfilterproxymodel identityproxymodel transposeproxymodel concatenatetablesproxymodel stringlistmodel translation easingcurve animation gestures jalalicalendar islamiccivilcalendar timezone commandlineparser cborstreamreader cborstreamwriter permissions threadsafe-cloexec static pkg-config reduce_relocations signaling_nan thread future concurrent dbus openssl-linked opensslv30 static static reduce_exports reduce_relocations openssl
QT.core.disabled_features = cpp-winrt shared cross_compile debug_and_release separate_debug_info appstore-compliant simulator_and_device rpath force_asserts framework c++20 c++2a c++2b wasm-simd128 wasm-exceptions zstd opensslv11
QT_CONFIG += clock-monotonic cxx11_future cxx17_filesystem eventfd glib inotify std-atomic64 mimetype regularexpression sharedmemory shortcut systemsemaphore xmlstream xmlstreamreader xmlstreamwriter textdate datestring process processenvironment temporaryfile library settings filesystemwatcher filesystemiterator itemmodel proxymodel sortfilterproxymodel identityproxymodel transposeproxymodel concatenatetablesproxymodel stringlistmodel translation easingcurve animation gestures jalalicalendar islamiccivilcalendar timezone commandlineparser cborstreamreader cborstreamwriter permissions threadsafe-cloexec static pkg-config reduce_relocations signaling_nan thread future concurrent dbus openssl-linked opensslv30 static static reduce_exports reduce_relocations openssl
QT_MODULES += core

