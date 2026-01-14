use std::collections::HashMap;
use std::fs::{self, Metadata};
use std::path::PathBuf;
use sha2::digest::crypto_common::IvSizeUser;
use walkdir::WalkDir;


fn main() {
    let target_dir = "./";

    let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    println!("Scanning directory: {}...", target_dir);

    for entry in WalkDir::new(target_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Ok(metadata) = fs::metadata(path){
                let size = metadata.len();
                size_map.entry(size).or_insert(Vec::new()).push(path.to_path_buf());
            }
        }
    }

    println!("\n--- 重複の可能性があるファイル（サイズ一致） ---");
    for (size, paths) in size_map {
        if paths.len() > 1 {
            println!("サイズ: {} bytes", size);
            for path in paths {
                println!("  - {:?}", path);
            }
        }
    }
}
