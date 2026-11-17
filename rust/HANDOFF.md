# Handoff

Status: v1.1.0 core + Windows IMAPI2 drive enumeration + optional egui GUI. 6 tests pass.
Both default and `--features gui` builds succeed; clippy clean.

- Toolchain: MSVC (x86_64-pc-windows-msvc) via `.cargo/config.toml`.
- Next: implement physical burn path and ISO writing (libmkiso, currently blocked by disk space).
- Known gaps: GUI is minimal; physical burning not yet implemented; ISO writing behind `iso` feature (not buildable in current env).