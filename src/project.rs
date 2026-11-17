use crate::models::{BurnFile, DiscType};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnProject {
    pub name: String,
    pub disc_type: DiscType,
    pub virtual_burner: bool,
    pub iso_output_dir: Option<PathBuf>,
    pub volume_label: String,
    pub finalize: bool,
    pub eject: bool,
    pub files: Vec<BurnFile>,
}

impl Default for BurnProject {
    fn default() -> Self {
        Self {
            name: "Untitled".into(),
            disc_type: DiscType::Dvd4_7,
            virtual_burner: true,
            iso_output_dir: None,
            volume_label: "BURN_DISK".into(),
            finalize: true,
            eject: false,
            files: Vec::new(),
        }
    }
}

impl BurnProject {
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let tmp = path.with_extension("tmp");
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&tmp, json)?;
        std::fs::rename(&tmp, path)?;
        Ok(())
    }

    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let mut proj: BurnProject = serde_json::from_str(&data)?;
        // re-measure sizes from disk
        let mut kept = Vec::new();
        for f in proj.files.drain(..) {
            if f.path.exists() {
                if let Ok(bf) = BurnFile::new(f.path.clone()) {
                    kept.push(bf);
                }
            }
        }
        proj.files = kept;
        Ok(proj)
    }
}
