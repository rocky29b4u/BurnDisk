use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Optical disc media types with their usable (user-data) capacities in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscType {
    Cd700,   // 700 MB
    Dvd4_7,  // 4.7 GB
    Dvd8_5,  // 8.5 GB dual layer
    BdR25,   // 25 GB
    BdR50,   // 50 GB
    BdXl100, // 100 GB
}

impl DiscType {
    pub fn capacity_bytes(self) -> u64 {
        match self {
            DiscType::Cd700 => 700 * 1024 * 1024,
            DiscType::Dvd4_7 => 4_700_000_000,
            DiscType::Dvd8_5 => 8_500_000_000,
            DiscType::BdR25 => 25_000_000_000,
            DiscType::BdR50 => 50_000_000_000,
            DiscType::BdXl100 => 100_000_000_000,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            DiscType::Cd700 => "CD 700 MB",
            DiscType::Dvd4_7 => "DVD 4.7 GB",
            DiscType::Dvd8_5 => "DVD 8.5 GB (DL)",
            DiscType::BdR25 => "BD-R 25 GB",
            DiscType::BdR50 => "BD-R 50 GB",
            DiscType::BdXl100 => "BDXL 100 GB",
        }
    }

    pub fn all() -> &'static [DiscType] {
        &[
            DiscType::Cd700,
            DiscType::Dvd4_7,
            DiscType::Dvd8_5,
            DiscType::BdR25,
            DiscType::BdR50,
            DiscType::BdXl100,
        ]
    }
}

impl std::fmt::Display for DiscType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// A file or folder entry selected for burning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnFile {
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
}

impl BurnFile {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let meta = std::fs::metadata(&path)?;
        let size = if meta.is_dir() {
            dir_size(&path)
        } else {
            meta.len()
        };
        Ok(BurnFile {
            path,
            size,
            is_dir: meta.is_dir(),
        })
    }
}

fn dir_size(p: &std::path::Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(p) {
        for e in entries.flatten() {
            let path = e.path();
            if let Ok(ft) = e.file_type() {
                if ft.is_dir() {
                    total += dir_size(&path);
                } else if ft.is_file() {
                    if let Ok(m) = std::fs::metadata(&path) {
                        total += m.len();
                    }
                }
            }
        }
    }
    total
}

/// A single planned disc/volume.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpannedDisc {
    pub index: usize,
    pub total: usize,
    pub files: Vec<BurnFile>,
    pub used_bytes: u64,
    pub capacity_bytes: u64,
}

impl SpannedDisc {
    pub fn free_bytes(&self) -> u64 {
        self.capacity_bytes.saturating_sub(self.used_bytes)
    }
    pub fn utilization(&self) -> f64 {
        if self.capacity_bytes == 0 {
            0.0
        } else {
            self.used_bytes as f64 / self.capacity_bytes as f64
        }
    }
}

/// Result of planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanningPlan {
    pub disc_type: DiscType,
    pub discs: Vec<SpannedDisc>,
    pub skipped: Vec<BurnFile>, // files larger than a single disc
    pub total_bytes: u64,
}

impl SpanningPlan {
    pub fn disc_count(&self) -> usize {
        self.discs.len()
    }
}
