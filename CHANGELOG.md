# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.0] - 2026-11-17

### Added
- MSVC toolchain support; IMAPI2 optical-drive enumeration via the `windows` crate (`burn-disk drives`).
- Optional GUI front-end behind the `gui` feature (egui/eframe) with drive enumeration button.

## [1.0.0] - 2026-11-17

### Added
- Initial Rust port of the DiscBurner disc-burning core: disc types with capacities,
  file/folder scanning, multi-disc spanning planner (first-fit-decreasing), and project
  persistence (JSON).
- CLI: `types`, `scan`, `plan`, `new-project`, `info`.
- 6 unit/integration tests.