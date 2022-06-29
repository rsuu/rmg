use crate::utils::types::{MyError, MyResult, SelfResult};
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    fs::{self, File},
    io::{self, Cursor, Read, Write},
};

/// `".rmgdata".as_bytes() = [46, 114, 109, 103, 100, 97, 116, 97]`
pub const MAGICK_NUMBER: [u8; 8] = [46, 114, 109, 103, 100, 97, 116, 97];

macro_rules! meta_from_bytes {
    ($rdr:expr,$elem:expr) => {
        if let Ok(bool_) = read_bool::<Cursor<&Vec<u8>>>($rdr) {
            if bool_ == true {
                if let Ok(s) = read_string::<LittleEndian, Cursor<&Vec<u8>>>($rdr) {
                    $elem = Some(s);
                } else {
                }
            } else {
            }
        } else {
            panic!("");
        }
    };
}

macro_rules! meta_to_bytes {
    ($wtr:expr,$elem:expr) => {
        if $elem.is_some() {
            write_bool::<Vec<u8>>($wtr, true)?;
            write_string::<LittleEndian, Vec<u8>, &str>($wtr, $elem.expect("expect &str"))?;
        } else {
            write_bool::<Vec<u8>>($wtr, false)?;
        }
    };
}

/// Data format
/// ```text
/// type bool  = u8;
///      true  = 1;
///      false = 0;
/// type usize = {u32} || {u64}
///
/// let MAGICK_NUMBER = [46, 114, 109, 103, 100, 97, 116, 97]
///
/// {MAGICK_NUMBER}                                                        ,
/// {_p: bool}  ||  {_p: bool && _size: usize && female:    Option<String>},
/// {_p: bool}  ||  {_p: bool && _size: usize && male:      Option<String>},
/// {_p: bool}  ||  {_p: bool && _size: usize && mixed:     Option<String>},
/// {_p: bool}  ||  {_p: bool && _size: usize && language:  Option<String>},
/// {_p: bool}  ||  {_p: bool && _size: usize && other:     Option<String>},
/// {_p: bool}  ||  {_p: bool && _size: usize && group:     Option<String>},
/// {_p: bool}  ||  {_p: bool && _size: usize && artist:    Option<String>},
/// {_p: bool}  ||  {_p: bool && _size: usize && cosplayer: Option<String>},
/// {_p: bool}  ||  {_p: bool && _size: usize && parody:    Option<String>},
/// {_p: bool}  ||  {_p: bool && _size: usize && character: Option<String>},
/// {_p: bool}  ||  {_p: bool && _size: usize && reclass:   Option<String>},
/// {_p: bool}  ||  {_p: bool && _size: usize && temp:      Option<String>},
/// ```
/// Example
/// ```text
/// magick_number:
///   MAGICK_NUMBER
/// female:
///   START [0_bool]  DONE
/// male:
///   START [0_bool]  DONE
/// mixed:
///   START [1_bool,
///         100_usize,
///         100_len_String
///         ]  DONE
/// ...
/// ```
#[derive(Debug)]
pub struct MetaData {
    pub magick_number: [u8; 8],
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
            magick_number: MAGICK_NUMBER,
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

    pub fn from_bytes(&self, v: &[u8]) -> SelfResult<Self> {
        let v = v.to_vec();

        let mut meta: MetaData = MetaData::new();
        let mut rdr = Cursor::new(&v);
        let mut magick_buf: [u8; 8] = [0; 8];

        rdr.read_exact(&mut magick_buf)?;
        if magick_buf == self.magick_number {
        } else {
            return Err(MyError::ErrMagickNumber);
        }

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

        Ok(meta)
    }

    pub fn to_bytes(&self) -> SelfResult<Vec<u8>> {
        let mut wtr = Vec::new();

        wtr.write_all(self.magick_number.as_slice())?;

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

        Ok(wtr)
    }

    pub fn read_from_file<S>(&self, filename: S) -> SelfResult<Self>
    where
        S: AsRef<str>,
    {
        let fmetadata = fs::metadata(filename.as_ref())?;
        let mut f = File::open(filename.as_ref())?;
        let mut buffer = vec![0; fmetadata.len() as usize];

        f.read(&mut buffer)?;

        self.from_bytes(&buffer)
    }

    pub fn write_to_file<S>(&self, filename: S) -> MyResult
    where
        S: AsRef<str>,
    {
        let mut f = File::create(filename.as_ref())?;

        f.write_all(self.to_bytes()?.as_slice())?;

        Ok(())
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

pub fn read_string<B, R>(rdr: &mut R) -> SelfResult<String>
where
    B: ByteOrder,
    R: ReadBytesExt,
{
    let str_len = read_usize::<B, R>(rdr)?;
    let mut str_bytes = vec![0_u8; str_len as usize];

    rdr.read_exact(&mut str_bytes)?;

    return Ok(String::from_utf8(str_bytes)?);
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

pub fn read_bool<R>(b: &mut R) -> Result<bool, std::io::Error>
where
    R: ReadBytesExt,
{
    let bool_ = b.read_u8()?;

    if bool_ == 1 {
        Ok(true)
    } else if bool_ == 0 {
        Ok(false)
    } else {
        panic!()
    }
}

pub fn write_bool<W>(wtr: &mut W, data: bool) -> Result<(), std::io::Error>
where
    W: WriteBytesExt,
{
    if data == true {
        wtr.write_u8(1_u8)
    } else {
        wtr.write_u8(0_u8)
    }
}
