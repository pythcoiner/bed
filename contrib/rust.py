import os
import sys
import shutil
import subprocess
from pathlib import Path

offline = os.getenv("OFFLINE", "false").lower() == "true"
build_type = sys.argv[1].lower()
print(f"OFFLINE={offline}")
print(f"PROFILE={build_type}")

def run(cmd, **kwargs):
    print(f"Running: {' '.join(cmd) if isinstance(cmd, list) else cmd}")
    result = subprocess.run(cmd, check=True, **kwargs)
    return result

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
print(os.getcwd())

# Run cargo build
# cargo will:
#   - first generate c++ bindings
#   - then build library
cargo_cmd = ["cargo", "build"]
if build_type != "debug": 
    cargo_cmd.append("--release")

if offline:
    cargo_cmd.append("--offline")
run(cargo_cmd)

# cd ../../
os.chdir(Path("../../"))

# copy bindings into ./lib/include/
generated_header = Path("rust/bed/target/cxxbridge/bed/src/lib.rs.h")
cxx_header = Path("rust/bed/target/cxxbridge/rust/cxx.h")
shutil.copy(generated_header, include_dir / "bed.h")
shutil.copy(cxx_header, include_dir / "cxx.h")

# Copy libraries into ./lib/
rs_out_dir = Path("rust/bed/target") / ("debug" if build_type == "debug" else "release")

for filename in [
    "libbed.a",
    "libbed.rlib",
    "libbed.so",
    "libbed.d"
]:
    src = rs_out_dir /  filename
    src = Path(src)
    dst = lib_dir /   filename
    if src.exists():
        shutil.copy(src, dst)
    else:
        print(f"Warning: {filename} not found")

print("Done.")
