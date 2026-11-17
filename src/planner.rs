use crate::models::{BurnFile, DiscType, SpannedDisc, SpanningPlan};

/// First-fit-decreasing bin packing planner.
pub struct DiscSpanningPlanner {
    pub disc_type: DiscType,
    /// Headroom reserved per disc for filesystem overhead (3% + 8 MB).
    pub overhead: bool,
}

impl DiscSpanningPlanner {
    pub fn new(disc_type: DiscType) -> Self {
        Self {
            disc_type,
            overhead: true,
        }
    }

    fn usable_capacity(&self) -> u64 {
        let cap = self.disc_type.capacity_bytes();
        if self.overhead {
            // ~3% + 8 MB headroom
            cap - (cap * 3 / 100) - 8 * 1024 * 1024
        } else {
            cap
        }
    }

    pub fn plan(&self, mut files: Vec<BurnFile>) -> SpanningPlan {
        let capacity = self.usable_capacity();
        let total_bytes: u64 = files.iter().map(|f| f.size).sum();

        // sort largest first
        files.sort_by_key(|a| std::cmp::Reverse(a.size));

        let mut discs: Vec<SpannedDisc> = Vec::new();
        let mut skipped: Vec<BurnFile> = Vec::new();

        for f in files {
            if f.size > capacity {
                skipped.push(f);
                continue;
            }
            // first-fit: find first disc with enough space
            let mut placed = false;
            for disc in discs.iter_mut() {
                if disc.used_bytes + f.size <= disc.capacity_bytes {
                    disc.used_bytes += f.size;
                    disc.files.push(f.clone());
                    placed = true;
                    break;
                }
            }
            if !placed {
                let mut new_disc = SpannedDisc {
                    index: discs.len(),
                    total: 0, // filled later
                    files: vec![f.clone()],
                    used_bytes: f.size,
                    capacity_bytes: capacity,
                };
                // fix capacity for the disc (same)
                let _ = &mut new_disc;
                discs.push(new_disc);
            }
        }

        let total = discs.len();
        for (i, d) in discs.iter_mut().enumerate() {
            d.index = i;
            d.total = total;
            d.capacity_bytes = capacity;
        }

        SpanningPlan {
            disc_type: self.disc_type,
            discs,
            skipped,
            total_bytes,
        }
    }
}
