#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use flate2::read::{ZlibDecoder};
#[allow(unused_imports)]
use std::io::prelude::*;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory");
    } else if args[1] == "cat-file"{
        // We will try to recover the object : 9daeafb9864cf43055ae93beb0afd6c7d144bfa4
        // And : 250591f6d4523e8d78215e3ade01d5bd946ed33b

        // First find the folder which contains the file
        let folder_name = &args[3][0..2];
        let file_name = &args[3][2..];
        match fs::read(format!(".git/objects/{}/{}", &folder_name, &file_name)) {
            Err(_) => {
                println!("Error fetching file")
            },
            Ok(content) => {
                // Decompress to a usable string
                let mut z = ZlibDecoder::new(&content[..]);
                let mut s = String::new();
                z.read_to_string(&mut s).expect("Error decoding data");
                // Now filtering content to start after the first null char
                let content = &s[s.find('\0').unwrap()..];
                print!("{content}")
            }
        }
    } else {
        println!("unknown command: {}", args[1])
    }
}
