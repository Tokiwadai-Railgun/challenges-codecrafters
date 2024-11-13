use std::env;
use std::fs;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use std::io::prelude::*;
use sha1::{Digest, Sha1};
use hex;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
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
        // test.txt = 2656faa96107f2c8028df08716cfe408e7ee3fed
        // First find the folder which contains the file
        let folder_name = &args[3][0..2];
        let file_name = &args[3][2..];
        match fs::read(format!(".git/objects/{}/{}", &folder_name, &file_name)) {
            Err(_) => {
                println!("Error fetching file")
            },
            Ok(content) => {
                // Decompress file
                let unzipped_content = unzip_content(&content);
                print!("{unzipped_content}")
            }
        }
    }
    else if args[1] == "hash-object"{
        // testing on : 8a61407747a6b8cbdb840c42a660c220eb201e00

        // First step : Read the file
        let content = fs::read(format!("./{}", args[3])).expect("Error reading file data");

        // Then create the file
    
        // Write Zipped content in it
        let zipped_content = zip_content(&content).expect("Zip contained non UTF-8 characters");
    } else {
        println!("unknown command: {}", args[1])
    }
}

fn hash_content(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    hex::encode(hasher.finalize())
}

fn unzip_content(content: &[u8]) -> String {
    let mut d = ZlibDecoder::new(content);
    let mut s = String::new();
    d.read_to_string(&mut s).expect("Error dezipping data");
    s
}

fn zip_content(content: &[u8]) -> Result<String, std::string::FromUtf8Error>{
    // Also add the glob <size>\0 at the start of the file
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    let _ = e.write_all(b"blob ");
    let _ = e.write_all(content.len().to_string().as_bytes());
    let _ = e.write_all(b"\0");
    let _ = e.write_all(content);
    String::from_utf8(e.finish().expect("Failed compressing file"))
}
