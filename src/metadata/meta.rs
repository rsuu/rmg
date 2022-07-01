use crate::{
    metadata::tags,
    utils::types::{MyResult, SelfResult},
};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use speedy::{Readable, Writable};

/// `".rmgdata".as_bytes() = [46, 114, 109, 103, 100, 97, 116, 97]`
pub const MAGICK_NUMBER: [u8; 8] = [46, 114, 109, 103, 100, 97, 116, 97];

pub trait TUpdateMeta {
    fn other(&mut self, data: &Option<impl AsRef<str>>);
    fn artist(&mut self, data: &Option<tags::TagArtist>);
    fn character(&mut self, data: &Option<tags::TagCharacter>);
    fn cosplayer(&mut self, data: &Option<tags::TagCosplayer>);
    fn female(&mut self, data: &Option<tags::TagFemale>);
    fn group(&mut self, data: &Option<tags::TagGroup>);
    fn language(&mut self, data: &Option<tags::TagLanguage>);
    fn male(&mut self, data: &Option<tags::TagMale>);
    fn mixed(&mut self, data: &Option<tags::TagMixed>);
    fn parody(&mut self, data: &Option<tags::TagParody>);
    fn reclass(&mut self, data: &Option<tags::TagReclass>);
}

// RUSTFLAGS=-Zprint-type-sizes cargo run --release
#[repr(C)]
#[repr(align(8))]
#[derive(PartialEq, Eq, Debug, Readable, Writable)]
pub struct MetaData {
    pub magick_number: [u8; 8],
    pub artist: Option<tags::TagArtist>,
    pub character: Option<tags::TagCharacter>,
    pub cosplayer: Option<tags::TagCosplayer>,
    pub female: Option<tags::TagFemale>,
    pub group: Option<tags::TagGroup>,
    pub language: Option<tags::TagLanguage>,
    pub male: Option<tags::TagMale>,
    pub mixed: Option<tags::TagMixed>,
    pub parody: Option<tags::TagParody>,
    pub reclass: Option<tags::TagReclass>,
    pub other: Option<String>,
}

impl MetaData {
    pub fn new() -> Self {
        Self {
            magick_number: MAGICK_NUMBER,
            artist: None,
            character: None,
            cosplayer: None,
            female: None,
            group: None,
            language: None,
            male: None,
            mixed: None,
            other: None,
            parody: None,
            reclass: None,
        }
    }

    pub fn to_bytes(&self) -> SelfResult<Vec<u8>> {
        Ok(self.write_to_vec()?)
    }

    pub fn from_bytes<B>(&self, bytes: &B) -> SelfResult<Self>
    where
        B: AsRef<[u8]>,
    {
        Ok(Self::read_from_buffer(bytes.as_ref())?)
    }

    pub fn write_to_file<P>(&self, filename: &P) -> MyResult
    where
        P: AsRef<Path> + ?Sized,
    {
        if filename.as_ref().is_file() {
        } else {
            let mut f = File::create(filename.as_ref())?;

            f.write_all(self.to_bytes()?.as_slice())?;
        }

        Ok(())
    }

    pub fn read_from_file<P>(&self, filename: &P) -> SelfResult<Self>
    where
        P: AsRef<Path> + ?Sized,
    {
        let fmetadata = fs::metadata(filename.as_ref())?;
        let mut f = File::open(filename.as_ref())?;
        let mut buffer = vec![0; fmetadata.len() as usize];

        f.read(&mut buffer)?;

        self.from_bytes(&buffer)
    }
}

impl Default for MetaData {
    fn default() -> Self {
        Self::new()
    }
}

impl TUpdateMeta for MetaData {
    fn other(&mut self, data: &Option<impl AsRef<str>>) {
        self.other = data.as_ref().map(|s| s.as_ref().to_string());
    }

    fn artist(&mut self, data: &Option<tags::TagArtist>) {
        self.artist = (*data).clone();
    }
    fn character(&mut self, data: &Option<tags::TagCharacter>) {
        self.character = (*data).clone();
    }
    fn cosplayer(&mut self, data: &Option<tags::TagCosplayer>) {
        self.cosplayer = (*data).clone();
    }
    fn female(&mut self, data: &Option<tags::TagFemale>) {
        self.female = (*data).clone();
    }
    fn group(&mut self, data: &Option<tags::TagGroup>) {
        self.group = (*data).clone();
    }
    fn language(&mut self, data: &Option<tags::TagLanguage>) {
        self.language = *data;
    }
    fn male(&mut self, data: &Option<tags::TagMale>) {
        self.male = (*data).clone();
    }
    fn reclass(&mut self, data: &Option<tags::TagReclass>) {
        self.reclass = *data;
    }
    fn parody(&mut self, data: &Option<tags::TagParody>) {
        self.parody = (*data).clone();
    }
    fn mixed(&mut self, data: &Option<tags::TagMixed>) {
        self.mixed = (*data).clone();
    }
}
