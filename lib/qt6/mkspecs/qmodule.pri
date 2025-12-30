QT_CPU_FEATURES.x86_64 = 
QT.global_private.enabled_features = gc_binaries reduce_exports x86intrin sse2 sse3 ssse3 sse4_1 sse4_2 avx f16c avx2 avx512f avx512er avx512cd avx512pf avx512dq avx512bw avx512vl avx512ifma avx512vbmi avx512vbmi2 aesni vaes rdrnd rdseed shani posix_fallocate alloca_h alloca system-zlib dbus dbus-linked gui network printsupport sql testlib widgets xml libudev openssl dlopen largefile precompile_header enable_new_dtags sse2 sse3 ssse3 sse4_1 sse4_2 avx f16c avx2 avx512f avx512er avx512cd avx512pf avx512dq avx512bw avx512vl avx512ifma avx512vbmi avx512vbmi2 aesni vaes rdrnd rdseed shani
QT.global_private.disabled_features = use_bfd_linker use_gold_linker use_lld_linker use_mold_linker android-style-assets developer-build private_tests debug no_direct_extern_access mips_dsp mips_dspr2 neon arm_crc32 arm_crypto alloca_malloc_h stack-protector-strong stdlib-libcpp relocatable intelcet
CONFIG += largefile precompile_header enable_new_dtags sse2 sse3 ssse3 sse4_1 sse4_2 avx f16c avx2 avx512f avx512er avx512cd avx512pf avx512dq avx512bw avx512vl avx512ifma avx512vbmi avx512vbmi2 aesni vaes rdrnd rdseed shani
PKG_CONFIG_EXECUTABLE = /nix/store/kkxx963z7a6ihb3xhvxlqaziyb18zc84-pkg-config-wrapper-0.29.2/bin/pkg-config
QT_COORD_TYPE = double
QT_BUILD_PARTS = libs tools

QMAKE_LIBS_ZLIB = -lz
QMAKE_INCDIR_DBUS = /nix/store/yvhgv4qr73s53wnia8mh49788ckvaq3k-dbus-1.14.4-dev/include/dbus-1.0 /nix/store/p7sf03b0i5pw7kz3lvbgh9d2yp9gd1qg-dbus-1.14.4-lib/lib/dbus-1.0/include
QMAKE_LIBS_DBUS = -ldbus-1
QMAKE_LIBS_LIBUDEV = -ludev
