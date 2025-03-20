use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
struct FileInfo {
    path: PathBuf,
    size: u64,
    flie_type: String,
}

struct DirStats {
    total_size: u64,
    file_count: usize,
    file_types: HashMap<String, usize>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let root_path = if args.len() > 1 {
        Path::new(&args[1]).to_path_buf()
    } else {
        env::current_dir()?
    };

    let mut stats = DirStats {
        total_size: 0,
        file_count: 0,
        file_types: HashMap::new(),
    };

    let largest_file = scan_directory(&root_path, &mut stats, 10)?;

    println!("\nSummary:");
    println!("Total size: {}", stats.total_size);
    println!("Total files: {}", stats.file_count);

    println!("\nFile types:");
    for (file_types, count) in stats.file_types.iter() {
        println!(" {}: {} files", file_types, count);
    }

    println!("\nLargest files:");
    for file in largest_file {
        println!(" {}: {} bytes", file.path.display(), file.size)
    }

    Ok(())
}

fn scan_directory(dir: &Path, stats: &mut DirStats, top_n: usize) -> io::Result<Vec<FileInfo>> {
    let mut largest_file: Vec<FileInfo> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let mut sub_largest = scan_directory(&path, stats, top_n)?;
                largest_file.append(&mut sub_largest)
            } else {
                let metadata = fs::metadata(&path)?;
                let size = metadata.len();

                stats.total_size += size;
                stats.file_count += 1;

                let extension = path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                *stats.file_types.entry(extension.clone()).or_insert(0) += 1;

                largest_file.push(FileInfo {
                    path: path.clone(),
                    size,
                    flie_type: extension,
                });
            }
        }
    }

    largest_file.sort_by(|a, b| b.size.cmp(&a.size));
    largest_file.truncate(top_n);

    Ok(largest_file)
}
