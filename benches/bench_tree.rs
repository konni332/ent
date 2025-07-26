use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use tempfile::tempdir;
use std::fs::{create_dir, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use ent::{TreeEntry, Cli, Format};


fn bench_build_tree(c: &mut Criterion) {
    let tmp_dir = tempdir().unwrap();
    let root = tmp_dir.path();

    for i in 0..10 {
        File::create(root.join(format!("file{i}.txt"))).unwrap();
    }
    create_dir(root.join("sub")).unwrap();
    for i in 0..10 {
        File::create(root.join("sub").join(format!("subfile{i}.log"))).unwrap();
    }

    let cli = Cli {
        hidden: false,
        all: false,
        ignored: false,
        depth: None,
        dirs_only: false,
        files_only: false,
        ..Default::default()
    };

    c.bench_function("build_tree", |b| {
        b.iter(|| {
            TreeEntry::build(&cli, root.to_str().unwrap()).unwrap();
        });
    });
}

fn create_complex_fs(root: &PathBuf, depth: usize, width: usize) {
    fn create_rec(path: &PathBuf, current_depth: usize, max_depth: usize, width: usize) {
        if current_depth >= max_depth {
            return;
        }
        for i in 0..width {
            let dir = path.join(format!("dir_{current_depth}_{i}"));
            create_dir_all(&dir).unwrap();
            for j in 0..width {
                let file = dir.join(format!("file_{current_depth}_{i}_{j}.txt"));
                let mut f = File::create(file).unwrap();
                writeln!(f, "Dummy content").unwrap();
            }
            create_rec(&dir, current_depth + 1, max_depth, width);
        }
    }

    create_rec(root, 0, depth, width);
}

fn bench_large_tree(c: &mut Criterion) {
    let tmp_dir = tempdir().unwrap();
    let root = tmp_dir.path().to_path_buf();

    create_complex_fs(&root, 4, 5);

    let cli = Cli {
        hidden: false,
        all: false,
        ignored: false,
        depth: None,
        dirs_only: false,
        files_only: false,
        ..Default::default()
    };

    c.bench_function("build_large_tree", |b| {
        b.iter(|| {
            let _ = TreeEntry::build(black_box(&cli), black_box(root.to_str().unwrap())).unwrap();
        });
    });
}


fn bench_export_tree(c: &mut Criterion) {
    let mut tree = TreeEntry::new_dir("test".to_string());
    for i in 0..100 {
        tree.children.as_mut().unwrap().push(TreeEntry::new_file(format!("file{i}.rs")));
    }

    let tmp_file = tempfile::NamedTempFile::new().unwrap();

    c.bench_function("export_tree_json", |b| {
        b.iter(|| {
            tree.export(tmp_file.path(), Format::Json).unwrap();
        });
    });
}


criterion_group!(benches, bench_build_tree, bench_export_tree, bench_large_tree);
criterion_main!(benches);
