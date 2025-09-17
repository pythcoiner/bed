import os
import shutil
import subprocess
import sys
from pathlib import Path


def run(cmd, **kwargs):
    print(f"Running: {' '.join(cmd) if isinstance(cmd, list) else cmd}")
    result = subprocess.run(cmd, check=True, **kwargs)
    return result

def build_rust(build_type, target="linux", offline=False):

    print(f"{offline=}")
    print(f"{build_type=}")
    print(f"{target=}")

    win_target = "x86_64-pc-windows-gnu"

    # create ./lib if not existing
    lib_dir = Path("lib")

    if lib_dir.exists():
        shutil.rmtree(lib_dir)
    lib_dir.mkdir(parents=True)

    include_dir = lib_dir / "include"
    include_dir.mkdir()

    # cd ./rust/bed
    rust_dir = Path("rust") / "bed"
    rust_dir.mkdir(exist_ok=True)
    os.chdir(rust_dir)

    # Run cargo build
    # cargo will:
    #   - first generate c++ bindings
    #   - then build library
    cargo_cmd = ["cargo", "build"]

    if target == "windows":
        cargo_cmd.append("--target")
        cargo_cmd.append(win_target)
    if build_type != "debug": 
        cargo_cmd.append("--release")

    if offline:
        cargo_cmd.append("--offline")
    run(cargo_cmd, env=os.environ.copy())

    # cd ../../
    os.chdir(Path("../../"))

    # copy bindings into ./lib/include/
        # cargo_cmd.append()
    header_path = Path("rust/bed/target")
    if target == "windows":
        header_path = header_path / win_target
    generated_header = header_path / "cxxbridge/bed/src/lib.rs.h"
    cxx_header = header_path / "cxxbridge/rust/cxx.h"
    shutil.copy(generated_header, include_dir / "bed.h")
    shutil.copy(cxx_header, include_dir / "cxx.h")

    # Copy libraries into ./lib/<target>/
    if target == "windows":
        lib_target = lib_dir / "windows"
    else:
        lib_target = lib_dir / "linux"
    lib_target.mkdir(exist_ok=True)

    rs_out_dir = Path("rust/bed/target")
    if target == "windows":
        rs_out_dir = rs_out_dir / win_target
    rs_out_dir = rs_out_dir / ("debug" if build_type == "debug" else "release")

    if target == "windows":
        filename = "libbed.a"
    else:
        filename = "libbed.a"

    src = rs_out_dir /  filename
    src = Path(src)
    dst = lib_target /   filename
    if src.exists():
        shutil.copy(src, dst)
    else:
        print(f"Warning: {filename} not found")

def build():
    if len(sys.argv) > 1:
        build_type = sys.argv[1].lower()
    else:
        build_type = "release"
    if len(sys.argv) > 2:
        target = sys.argv[2].lower()
    else:
        target = "linux"

    offline = os.getenv("OFFLINE", "false").lower() == "true"

    qt_path = os.getenv("Qt_PATH");
    if not qt_path:
        # TODO: try to be smart and look up in C:/Qt
        qt_path =  "C:/Qt/6.9.1/mingw_64"

    # create ./build if not existing
    build_dir = Path("build")
    if not build_dir.exists():
        build_dir.mkdir()

    # build rust lib
    build_rust(build_type, target, offline)

    # cd build
    os.chdir(build_dir)

    if build_type == "debug":
        build_type = "Debug"
    else:
        build_type = "Release"

    if target == "windows":
        cmake_cmd = ["cmake"]
    else:
        cmake_cmd = ["cmake"]


    if target == "windows":
        make_cmd = ["mingw32-make", "-j20"]
    else:
        make_cmd = ["make", "-j20"]

    cmake_cmd.append("..")
    cmake_cmd.append(f"-DCMAKE_BUILD_TYPE={build_type}")
    cmake_cmd.append(f"-DTARGET_OS={target}")

    if target == "windows":
        cmake_cmd.append(f"-DCMAKE_PREFIX_PATH={qt_path}")
        cmake_cmd.append("-G")
        cmake_cmd.append("MinGW Makefiles")

    # cmake ..
    run(cmake_cmd)

    # make -j20
    run(make_cmd)

    # move the binary to ./build/bin/
    exe_path = "Bed.exe"
    bin_dir =  Path("bin")
    if not bin_dir.exists():
        bin_dir.mkdir()

    if target == "windows":
        dst = bin_dir / "Bed.exe"
        shutil.move(exe_path, dst)

        #run windeployqt6 to fetch all needed dependencies
        qtdeploy_cmd = [f"{qt_path}/bin/windeployqt6", f"{bin_dir}/Bed.exe"]
        qtdeploy_cmd.append("--no-translations")
        run(qtdeploy_cmd)

    elif target == "linux":
        dst = bin_dir / "bed-x86_64-linux-gnu"
        shutil.move(exe_path, dst)


    # cp compile_commands.json ../compile_commands.json
    src = Path("compile_commands.json")
    dst = Path("..") / "compile_commands.json"
    if src.exists():
        shutil.copy(src, dst)
        print(f"compile_commands.json updated ")
    else:
        print(f"{src} does not exists at ({os.getcwd()})")

build()
