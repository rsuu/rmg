use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

use std::{
    fs::OpenOptions,
    io::{prelude::*, Seek, SeekFrom, Write},
};

extern crate tar;

use std::io::prelude;
use std::io::Read;
const BLOCK_LEN: u64 = 2;

fn main() {
    let path = std::env::args().nth(1).unwrap();

    let packs = Packs::from_dir(Path::new(&path), 1024 * 1024 / 2);
    //let packs = Packs::from_tar(Path::new(&path), 1024 * 1024 / 2);

    println!("{:#?}", packs);
}

#[derive(Debug)]
pub struct BlockInfo {
    len: u64,
    pad: u64,
}

#[derive(Debug, Clone)]
pub struct Packs(pub Vec<Pack>);

#[derive(Debug, Clone)]
pub struct Pack {
    pub id: usize,
    pub len: u64,

    pub range: Vec<PathBuf>,
}

impl Packs {
    pub fn from_dir(path: &Path, size: u64) -> Self {
        let mut blocks: Vec<BlockInfo> = Vec::new();
        let mut pack = Pack {
            len: 0,
            id: 0,
            range: Vec::new(),
        }; // temp value
        let mut packs: Vec<Pack> = Vec::new();
        let mut tmp: u64 = 0;

        for f in walkdir::WalkDir::new(path).into_iter() {
            let path = f.as_ref().unwrap().path();

            if path.is_file()
                && (path.extension().unwrap() == "jpg"
                    || path.extension().unwrap() == "jpeg"
                    || path.extension().unwrap() == "png")
            {
                tmp += path.metadata().unwrap().len();
                pack.range.push(path.to_path_buf());

                if tmp >= size {
                    pack.len = tmp;
                    packs.push(pack.clone());
                    pack.id += 1;
                    pack.range.clear();

                    tmp = 0;
                } else {
                }
            } else {
            }
        }

        if packs.is_empty() && !pack.range.is_empty() {
            pack.len = tmp;
            packs.push(pack);
        } else {
        }

        Packs(packs)
    }

    pub fn from_tar(path: &Path, size: u64) -> Self {
        let file = File::open(path).unwrap();
        let mut tar = tar::Archive::new(file);

        let mut blocks: Vec<BlockInfo> = Vec::new();
        let mut pack = Pack {
            len: 0,
            id: 0,
            range: Vec::new(),
        }; // temp value
        let mut packs: Vec<Pack> = Vec::new();
        let mut tmp: u64 = 0;

        for f in tar.entries().unwrap() {
            let path = f.as_ref().unwrap().path().unwrap();

            if path.extension().is_some()
                && (path.extension().unwrap() == "jpg"
                    || path.extension().unwrap() == "jpeg"
                    || path.extension().unwrap() == "png")
            {
                tmp += f.as_ref().unwrap().size();
                pack.range.push(path.to_path_buf());

                if tmp >= size {
                    pack.len = tmp;
                    packs.push(pack.clone());
                    pack.id += 1;
                    pack.range.clear();

                    tmp = 0;
                } else {
                }
            } else {
            }
        }

        if packs.is_empty() && !pack.range.is_empty() {
            pack.len = tmp;
            packs.push(pack);
        } else {
        }

        Packs(packs)
    }
}
