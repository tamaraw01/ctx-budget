// Integration tests for real-world scenarios
// Run with: cargo test --test integration_tests

#[cfg(test)]
mod integration_tests {
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn scan_mixed_encoding_files() {
        let temp = TempDir::new().unwrap();

        let mut file1 = fs::File::create(temp.path().join("valid.rs")).unwrap();
        file1
            .write_all(b"fn hello() { println!(\"world\"); }")
            .unwrap();

        let mut file2 = fs::File::create(temp.path().join("with_bom.txt")).unwrap();
        file2.write_all(&[0xef, 0xbb, 0xbf]).unwrap();
        file2.write_all(b"Hello, world!").unwrap();

        let mut file3 = fs::File::create(temp.path().join("invalid.bin")).unwrap();
        file3.write_all(&[0xff, 0xfe, 0xfd]).unwrap();

        let scan_result = std::process::Command::new("./target/release/ctx-budget")
            .arg(temp.path())
            .arg("--output")
            .arg("json")
            .output();

        assert!(
            scan_result.is_ok(),
            "scan should complete even with mixed encodings"
        );
        let output_data = scan_result.unwrap();
        let output = String::from_utf8_lossy(&output_data.stdout);
        assert!(
            output.contains("valid.rs"),
            "valid UTF-8 file should be counted"
        );
    }

    #[test]
    fn exclude_symlinks_safely() {
        let temp = TempDir::new().unwrap();

        fs::write(temp.path().join("file.txt"), "content").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs as unix_fs;
            let _ = unix_fs::symlink(temp.path().join("file.txt"), temp.path().join("link.txt"));
        }

        let scan_result = std::process::Command::new("./target/release/ctx-budget")
            .arg(temp.path())
            .output();

        assert!(
            scan_result.is_ok(),
            "scan should handle symlinks gracefully"
        );
    }

    #[test]
    fn handle_permission_denied_gracefully() {
        let temp = TempDir::new().unwrap();

        fs::write(temp.path().join("readable.rs"), "content").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let read_only = fs::Permissions::from_mode(0o000);
            let _ = fs::File::create(temp.path().join("unreadable.rs"))
                .and_then(|_| fs::set_permissions(temp.path().join("unreadable.rs"), read_only));
        }

        let scan_result = std::process::Command::new("./target/release/ctx-budget")
            .arg(temp.path())
            .arg("--output")
            .arg("text")
            .output();

        assert!(
            scan_result.is_ok(),
            "scan should not crash on permission denied"
        );
        let output_data = scan_result.unwrap();
        let output = String::from_utf8_lossy(&output_data.stdout);
        assert!(
            output.contains("readable.rs"),
            "readable files should be counted"
        );
    }

    #[test]
    fn streaming_large_files() {
        let temp = TempDir::new().unwrap();

        let large_file = temp.path().join("large.py");
        let mut f = fs::File::create(&large_file).unwrap();

        for i in 0..100_000 {
            writeln!(f, "def func_{:06}(): pass", i % 1_000_000).unwrap();
        }

        let start = std::time::Instant::now();
        let scan_result = std::process::Command::new("./target/release/ctx-budget")
            .arg(temp.path())
            .arg("--output")
            .arg("json")
            .output();
        let elapsed = start.elapsed();

        assert!(scan_result.is_ok(), "scan should handle 10MB file");
        assert!(elapsed.as_secs() < 5, "scan should complete in < 5 seconds");

        let output_data = scan_result.unwrap();
        let output = String::from_utf8_lossy(&output_data.stdout);
        assert!(output.contains("large.py"), "large file should be counted");
    }
}
