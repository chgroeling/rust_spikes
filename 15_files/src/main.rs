use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    {
        let cwd = env::current_dir();
        if let Ok(i) = &cwd {
            println!("current working directory {:?}", i);
        }
    }
    {
        let cexe = env::current_exe();
        if let Ok(i) = &cexe {
            println!("current exe directory {:?}", i);
        }
    }

    {
        let buffer = fs::read_to_string("test_file.txt").unwrap();
        println!("{buffer}");
    }
    {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open("test_file.txt")
            .unwrap();
        let mut buffer = String::new();

        let _ = file.read_to_string(&mut buffer);

        println!("{buffer}");
    }
    {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("test_file2.txt")
            .unwrap();

        let _ = file.write("HELLO WORLD".as_bytes());
    }
    {
        fs::remove_file("test_file2.txt").expect("could not remove file");
    }
    {
        let mut entries = fs::read_dir(".")
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        // The order in which `read_dir` returns entries is not guaranteed. If reproducible
        // ordering is required the entries should be explicitly sorted.

        entries.sort();

        // The entries have now been sorted by their path.
        for i in entries {
            println!("{:?}", i)
        }
    }
}
