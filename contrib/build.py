import os
import shutil
import subprocess
from pathlib import Path

def run(cmd, **kwargs):
    print(f"Running: {' '.join(cmd) if isinstance(cmd, list) else cmd}")
    subprocess.run(cmd, check=True, **kwargs)

# create ./build if not existing
build_dir = Path("build")
if not build_dir.exists():
    build_dir.mkdir()

# cd build
os.chdir(build_dir)

# cmake ..
run(["cmake", ".."])

# make -j20
run(["make", "-j20"])

# cp compile_commands.json ../compile_commands.json
src = build_dir / "compile_commands.json"
dst = Path("..") / "compile_commands.json"
if src.exists():
    shutil.copy(src, dst)
