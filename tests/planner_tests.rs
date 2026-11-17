use burn_disk::models::{BurnFile, DiscType};
use burn_disk::DiscSpanningPlanner;
use std::path::PathBuf;

fn bf(size: u64) -> BurnFile {
    BurnFile {
        path: PathBuf::from(format!("file_{size}.bin")),
        size,
        is_dir: false,
    }
}

#[test]
fn single_small_file_one_disc() {
    let files = vec![bf(1024)];
    let p = DiscSpanningPlanner::new(DiscType::Cd700).plan(files);
    assert_eq!(p.disc_count(), 1);
    assert_eq!(p.discs[0].files.len(), 1);
}

#[test]
fn spanning_splits_large_set() {
    // 10 files of 100MB on CD 700MB (~650MB usable) => at least 2 discs
    let files: Vec<_> = (0..10).map(|_| bf(100 * 1024 * 1024)).collect();
    let p = DiscSpanningPlanner::new(DiscType::Cd700).plan(files);
    assert!(p.disc_count() >= 2);
}

#[test]
fn oversized_file_skipped() {
    let files = vec![bf(10 * 1024 * 1024 * 1024)]; // 10GB on CD
    let p = DiscSpanningPlanner::new(DiscType::Cd700).plan(files);
    assert_eq!(p.disc_count(), 0);
    assert_eq!(p.skipped.len(), 1);
}

#[test]
fn total_bytes_preserved() {
    let files = vec![bf(1000), bf(2000), bf(3000)];
    let total: u64 = files.iter().map(|f| f.size).sum();
    let p = DiscSpanningPlanner::new(DiscType::Dvd4_7).plan(files);
    assert_eq!(p.total_bytes, total);
}

#[test]
fn utilization_under_one() {
    let files: Vec<_> = (0..5).map(|_| bf(100 * 1024 * 1024)).collect();
    let p = DiscSpanningPlanner::new(DiscType::Dvd4_7).plan(files);
    for d in &p.discs {
        assert!(d.utilization() <= 1.0 + 1e-9);
    }
}

#[test]
fn project_roundtrip() {
    use burn_disk::BurnProject;
    use std::io::Write;
    let dir = std::env::temp_dir();
    let path = dir.join("test_burn.proj");
    let mut proj = BurnProject::default();
    proj.name = "test".into();
    proj.save(&path).unwrap();
    let loaded = BurnProject::load(&path).unwrap();
    assert_eq!(loaded.name, "test");
    let _ = std::fs::remove_file(&path);
}
