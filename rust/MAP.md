# BurnDisk port map

Goal: a standalone Rust application "BurnDisk" that provides disc burning and ISO creation,
replacing the C# DiscBurner app and inspired by BurnAware. The original C# app and
BurnAware remain as reference.

- Original: `C:\Users\Sukhwinder\Projects\DiscBurner` (C#, WPF, .NET 8) + BurnAware Professional.
- Target: pure Rust binary `burn-disk` (CLI now; GUI optional later).
- Language of original (C#) is not directly supported by rust-anything; this port is a
  manual re-implementation of the testable core logic.

## Decisions index

See `DECISIONS.md` (ADR-001..).

## Units (state)

| Unit | State | Notes |
|------|-------|-------|
| core-models (DiscType, BurnFile) | done | gate: tests pass |
| scanner (folder walk) | done | std::fs |
| planner (FFD bin packing) | done | tests pass |
| project (JSON persistence) | done | tests pass |
| cli (argument parsing) | done | manual parsing |
| iso (virtual burner) | open | libmkiso optional, not yet wired |
| gui (WPF replacement) | open | out of scope for v1 |
| physical burn (IMAPI/SPTI) | open | requires Windows COM/SCSI; out of scope for v1 |

## Open questions

- Should the GUI be ported (egui/iced) or stay CLI?
- Physical burning via IMAPI2 COM from Rust (windows crate) - large effort.

## Out of scope (v1)

- WPF UI, themes, taskbar progress.
- Physical disc burning / SPTI transport.
- Appending to existing discs, erase.