"""
Invoke Task Script — STM32F411CE Firmware Tools

Dependencies:
    pip install invoke

Common Commands:
    inv build              # Build firmware (debug)
    inv build --release    # Build firmware (release)
    inv run                # Flash and run with RTT logs
    inv flash              # Flash only (no RTT monitoring)
    inv attach             # Attach to the running firmware and display RTT logs
    inv size               # Display binary sizes
    inv clean              # Clean build artifacts
"""

from invoke import task, Exit
import sys
import os
import subprocess

# ── Board Configuration ───────────────────────────────────────────────────────
CHIP    = "STM32F411CE"
TARGET  = "thumbv7em-none-eabihf"
PKG     = "examples-stm32"       
# ──────────────────────────────────────────────────────────────────────────────


def _elf(release: bool) -> str:
    profile = "release" if release else "debug"
    return f"target/{TARGET}/{profile}/{PKG}"


def _release_flag(release: bool) -> str:
    return "--release" if release else ""


def _run(cmd: str):
    res = subprocess.run(cmd, shell=True)
    if res.returncode != 0:
        raise Exit(res.returncode)


# ── build ─────────────────────────────────────────────────────────────────────
@task(help={
    "release": "Use release profile (lto + opt-level=s)",
})
def build(c, release=False):
    flags = _release_flag(release)
    profile  = "release" if release else "debug"
    print(f"[build] profile={profile}")
    _run(f"cargo build -p {PKG} {flags}")


# ── flash ─────────────────────────────────────────────────────────────────────
@task(help={
    "release": "Use release build artifact",
    "reset":   "Reset chip after programming (enabled by default)",
})
def flash(c, release=False, reset=True):
    build(c, release=release)
    elf = _elf(release)
    print(f"[flash] chip={CHIP}  elf={elf}")
    _run(f"probe-rs download --chip {CHIP} {elf}")
    if reset:
        _run(f"probe-rs reset --chip {CHIP}")


# ── run ───────────────────────────────────────────────────────────────────────
@task(help={
    "release": "Use release build artifact",
})
def run(c, release=False):
    flags = _release_flag(release)
    _run(f"cargo run -p {PKG} {flags}")


# ── attach ────────────────────────────────────────────────────────────────────
@task(help={
    "release": "Attach to the release build firmware",
})
def attach(c, release=False):
    elf = _elf(release)
    print(f"[attach] chip={CHIP}  elf={elf}")
    _run(f"probe-rs attach --chip {CHIP} --speed 4000 {elf}")


# ── size ──────────────────────────────────────────────────────────────────────
@task(help={
    "release": "View size of the release build artifact",
})
def size(c, release=False):
 
    build(c, release=release)
    elf = _elf(release)
    
    result = c.run("cargo size --version", warn=True, hide=True)
    if result.ok:
        flags = _release_flag(release)
        _run(f"cargo size -p {PKG} {flags} -- -A")
    else:
        print(f"[size] ELF file: {elf}")
        _run(f'powershell -Command "(Get-Item \\"{elf}\\").length | '
             f'ForEach-Object {{ \\"ELF size: $_ bytes\\" }}"')

# ── clean ─────────────────────────────────────────────────────────────────────
@task
def clean(c):
    print("clean target")
    _run("cargo clean")
