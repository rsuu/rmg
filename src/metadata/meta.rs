use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    fs::{self, File},
    io::{self, Cursor, Read, Write},
};

macro_rules! meta_from_bytes {
    ($rdr:expr,$elem:expr) => {
        if let Ok(s) = read_string::<LittleEndian, Cursor<&Vec<u8>>>($rdr) {
            $elem = Some(s);
        } else {
        }
    };
}

macro_rules! meta_to_bytes {
    ($wtr:expr,$elem:expr) => {
        if $elem.is_some() {
            write_string::<LittleEndian, Vec<u8>, &str>($wtr, $elem.unwrap()).unwrap();
        } else {
            write_usize::<LittleEndian, Vec<u8>>($wtr, 0_usize).unwrap();
        }
    };
}

#[derive(Debug)]
pub struct MetaData {
    pub female: Option<String>,
    pub male: Option<String>,
    pub mixed: Option<String>,
    pub language: Option<String>,
    pub other: Option<String>,
    pub group: Option<String>,
    pub artist: Option<String>,
    pub cosplayer: Option<String>,
    pub parody: Option<String>,
    pub character: Option<String>,
    pub reclass: Option<String>,
    pub temp: Option<String>,
}

impl MetaData {
    pub fn new() -> Self {
        Self {
            female: None,
            male: None,

            mixed: None,
            language: None,
            other: None,
            group: None,
            artist: None,
            cosplayer: None,
            parody: None,
            character: None,
            reclass: None,
            temp: None,
        }
    }

    pub fn from_bytes(v: &[u8]) -> Self {
        let v = v.to_vec();

        let mut meta: MetaData = MetaData::new();
        let mut rdr = Cursor::new(&v);

        meta_from_bytes! {&mut rdr,meta.female};
        meta_from_bytes! {&mut rdr,meta.male};
        meta_from_bytes! {&mut rdr,meta.mixed};
        meta_from_bytes! {&mut rdr,meta.language};
        meta_from_bytes! {&mut rdr,meta.other};
        meta_from_bytes! {&mut rdr,meta.group};
        meta_from_bytes! {&mut rdr,meta.artist};
        meta_from_bytes! {&mut rdr,meta.cosplayer};
        meta_from_bytes! {&mut rdr,meta.parody};
        meta_from_bytes! {&mut rdr,meta.character};
        meta_from_bytes! {&mut rdr,meta.reclass};
        meta_from_bytes! {&mut rdr,meta.temp};

        meta
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut wtr = Vec::new();

        // .as_deref() : Convert Option<String> to Option<&str>
        meta_to_bytes! {&mut wtr,self.female.as_deref()};
        meta_to_bytes! {&mut wtr,self.male.as_deref()};
        meta_to_bytes! {&mut wtr,self.mixed.as_deref()};
        meta_to_bytes! {&mut wtr,self.language.as_deref()};
        meta_to_bytes! {&mut wtr,self.other.as_deref()};
        meta_to_bytes! {&mut wtr,self.group.as_deref()};
        meta_to_bytes! {&mut wtr,self.artist.as_deref()};
        meta_to_bytes! {&mut wtr,self.cosplayer.as_deref()};
        meta_to_bytes! {&mut wtr,self.parody.as_deref()};
        meta_to_bytes! {&mut wtr,self.character.as_deref()};
        meta_to_bytes! {&mut wtr,self.reclass.as_deref()};
        meta_to_bytes! {&mut wtr,self.temp.as_deref()};

        wtr
    }

    pub fn read_from_file<S>(&self, filename: S) -> Self
    where
        S: AsRef<str>,
    {
        let fmetadata = fs::metadata(filename.as_ref()).expect("unable to read metadata");
        let mut f = File::open(filename.as_ref()).expect("no file found");
        let mut buffer = vec![0; fmetadata.len() as usize];

        f.read(&mut buffer).expect("buffer overflow");

        MetaData::from_bytes(&buffer)
    }

    pub fn write_to_file<S>(&self, filename: S)
    where
        S: AsRef<str>,
    {
        let mut f = File::create(filename.as_ref()).unwrap();

        f.write_all(self.to_bytes().as_slice()).unwrap();
    }
}

pub fn write_string<B, W, S>(wtr: &mut W, data: S) -> io::Result<()>
where
    B: ByteOrder,
    W: WriteBytesExt,
    S: AsRef<[u8]>,
{
    let data = data.as_ref();

    wtr.write_u64::<B>(data.len() as u64)?;
    wtr.write_all(data)?;

    Ok(())
}

pub fn read_string<B, R>(rdr: &mut R) -> Result<String, ()>
where
    B: ByteOrder,
    R: ReadBytesExt,
{
    let str_len = read_usize::<B, R>(rdr).unwrap();

    if str_len == 0 {
        return Err(());
    } else {
        let mut str_bytes = vec![0_u8; str_len as usize];

        rdr.read_exact(&mut str_bytes).unwrap();

        return Ok(String::from_utf8(str_bytes).expect(""));
    }
}

pub fn read_usize<B, R>(b: &mut R) -> Result<usize, std::io::Error>
where
    B: ByteOrder,
    R: ReadBytesExt,
{
    let res = if cfg!(target_pointer_width = "64") {
        b.read_u64::<B>()? as usize
    } else if cfg!(target_pointer_width = "32") {
        b.read_u32::<B>()? as usize
    } else {
        panic!()
    };

    Ok(res)
}

pub fn write_usize<B, W>(wtr: &mut W, data: usize) -> Result<(), std::io::Error>
where
    B: ByteOrder,
    W: WriteBytesExt,
{
    if cfg!(target_pointer_width = "64") {
        wtr.write_u64::<B>(data as u64)?
    } else if cfg!(target_pointer_width = "32") {
        wtr.write_u32::<B>(data as u32)?
    } else {
        panic!()
    }

    Ok(())
}
