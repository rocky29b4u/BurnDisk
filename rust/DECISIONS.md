# Decisions

## ADR-001: Target is a CLI, not a GUI, for v1
Status: accepted
Context: Porting WPF to Rust GUI frameworks is a large, separate effort; the testable core
logic (planning, scanning, project persistence) is UI-independent.
Decision: Ship a CLI binary `burn-disk` first; GUI is a follow-up.
Consequences: No windowed interaction; progress is textual.

## ADR-002: Manual argument parsing instead of clap
Status: accepted
Context: clap pulls in windows-sys which needs dlltool/MinGW unavailable in this env.
Decision: Parse `std::env::args()` manually.
Consequences: Fewer features, no help generation beyond a static message.

## ADR-003: std::fs instead of walkdir
Status: accepted
Context: walkdir depends on windows-sys (dlltool).
Decision: Recursive std::fs traversal.
Consequences: Slightly more code, no symlink handling.

## ADR-004: Keep libmkiso optional
Status: accepted
Context: ISO creation is desirable but libmkiso needs network and a newer toolchain.
Decision: Feature-gate ISO creation behind `iso` feature (default off).
Consequences: Default build stays lightweight; `--features iso` enables ISO writing.

## ADR-005: Headroom of 3% + 8MB per disc
Status: accepted
Context: Matches DiscBurner's planner for filesystem overhead.
Decision: Apply 3% + 8MB reduction to usable capacity.
Consequences: Plans slightly conservative; matches reference.