use crate::models::BurnFile;
use std::fs;
use std::path::Path;

pub fn scan_folder(root: &Path) -> std::io::Result<Vec<BurnFile>> {
    let mut out = Vec::new();
    if !root.is_dir() {
        return Ok(out);
    }
    scan_dir_recursive(root, &mut out)?;
    Ok(out)
}

fn scan_dir_recursive(dir: &Path, out: &mut Vec<BurnFile>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let ft = entry.file_type()?;
        if ft.is_dir() {
            scan_dir_recursive(&path, out)?;
        } else if ft.is_file() {
            let meta = fs::metadata(&path)?;
            out.push(BurnFile {
                path,
                size: meta.len(),
                is_dir: false,
            });
        }
    }
    Ok(())
}

pub fn burn_file_for(path: &Path) -> std::io::Result<BurnFile> {
    BurnFile::new(path.to_path_buf())
}
