use burn_disk::models::DiscType;
use burn_disk::{BurnProject, DiscSpanningPlanner};
use std::path::PathBuf;

fn parse_disc(s: &str) -> DiscType {
    match s.to_lowercase().as_str() {
        "cd" | "cd700" => DiscType::Cd700,
        "dvd" | "dvd4.7" | "dvd47" => DiscType::Dvd4_7,
        "dvd8.5" | "dvd85" | "dvd-dl" => DiscType::Dvd8_5,
        "bd25" | "bdr25" | "bd-r25" => DiscType::BdR25,
        "bd50" | "bdr50" | "bd-r50" => DiscType::BdR50,
        "bdxl" | "bdxl100" => DiscType::BdXl100,
        _ => DiscType::Dvd4_7,
    }
}

fn print_help() {
    println!("burn-disk - disc burning & ISO creation (Rust)");
    println!("Usage:");
    println!("  burn-disk types");
    println!("  burn-disk scan <path>");
    println!("  burn-disk plan <disc> <file|folder>...");
    println!("  burn-disk new-project <path> [disc]");
    println!("  burn-disk info <path>");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }
    let res = run(&args[1..]);
    if let Err(e) = res {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run(args: &[String]) -> anyhow::Result<()> {
    match args[0].as_str() {
        "types" => {
            for d in DiscType::all() {
                println!("{:<18} {:>14} bytes", d.label(), d.capacity_bytes());
            }
        }
        "scan" => {
            let path = PathBuf::from(&args[1]);
            let files = burn_disk::scanner::scan_folder(&path)?;
            let total: u64 = files.iter().map(|f| f.size).sum();
            println!("files: {}  total: {} bytes", files.len(), total);
        }
        "plan" => {
            let disc = parse_disc(&args[1]);
            let mut files = Vec::new();
            for p in &args[2..] {
                let pb = PathBuf::from(p);
                if pb.is_dir() {
                    files.extend(burn_disk::scanner::scan_folder(&pb)?);
                } else {
                    files.push(burn_disk::scanner::burn_file_for(&pb)?);
                }
            }
            let planner = DiscSpanningPlanner::new(disc);
            let plan = planner.plan(files);
            println!("Disc type: {}", plan.disc_type);
            println!("Total bytes: {}", plan.total_bytes);
            println!("Discs: {}", plan.disc_count());
            for d in &plan.discs {
                println!(
                    "  Disc {}/{}: {} files, {} / {} bytes ({:.1}%)",
                    d.index + 1,
                    d.total,
                    d.files.len(),
                    d.used_bytes,
                    d.capacity_bytes,
                    d.utilization() * 100.0
                );
            }
            if !plan.skipped.is_empty() {
                println!("Skipped (too large): {}", plan.skipped.len());
                for s in &plan.skipped {
                    println!("  - {} ({} bytes)", s.path.display(), s.size);
                }
            }
        }
        "new-project" => {
            let path = PathBuf::from(&args[1]);
            let disc = args.get(2).map(|s| s.as_str()).unwrap_or("dvd4.7");
            let proj = BurnProject {
                disc_type: parse_disc(disc),
                ..BurnProject::default()
            };
            proj.save(&path)?;
            println!("created project: {}", path.display());
        }
        "info" => {
            let path = PathBuf::from(&args[1]);
            let proj = BurnProject::load(&path)?;
            println!("name: {}", proj.name);
            println!("disc: {}", proj.disc_type);
            println!("files: {}", proj.files.len());
            let total: u64 = proj.files.iter().map(|f| f.size).sum();
            println!("total bytes: {}", total);
        }
        _ => {
            print_help();
        }
    }
    Ok(())
}
