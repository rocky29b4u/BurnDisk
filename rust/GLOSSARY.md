# Glossary

| Term (original) | Meaning | Rust name |
|-----------------|---------|-----------|
| DiscType | Optical media type with capacity | `models::DiscType` |
| BurnFile | File/folder selected for burning | `models::BurnFile` |
| SpannedDisc | One planned volume | `models::SpannedDisc` |
| SpanningPlan | Full multi-disc plan | `models::SpanningPlan` |
| DiscSpanningPlanner | FFD bin-packing planner | `planner::DiscSpanningPlanner` |
| BurnProject | Saved compilation (.dbproj) | `project::BurnProject` |
| Virtual Burner | Write ISO file instead of disc | CLI `plan` (ISO writing TBD) |