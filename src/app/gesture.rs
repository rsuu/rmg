use crate::*;

use ::zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};
use eyre::OptionExt;
use guessture::{find_matching_template_with_defaults, Path2D, Template};
use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Write},
};

// rmg --record-gesture test_gest
pub struct Gesture {
    pub temps: Vec<Template>,
    pub zip_path: String,
}

impl Gesture {
    pub fn new(zip_path: &str) -> eyre::Result<Self> {
        let mut temps = Vec::new();

        let file = {
            if let Ok(file) = File::open(zip_path) {
                file

            // $XDG_DATA_HOME/rmg/gestures.zip
            } else if let Some(mut default_dir) = dirs_next::data_dir() {
                default_dir.push("rmg/gestures.zip");

                File::open(default_dir)?
            } else {
                return Err(eyre::eyre!("Unknown to read `gestures.zip`"));
            }
        };

        let render = BufReader::new(file);
        let mut zip = ZipArchive::new(render)?;

        for i in 0..zip.len() {
            let mut gest_file = zip.by_index(i)?;

            if !gest_file.is_file() {
                continue;
            }

            let mut buf = Vec::new();
            gest_file.read_to_end(&mut buf)?;

            let name = gest_file.name().to_string();
            temps.push(load_gesture(name, buf.as_slice())?);
        }

        Ok(Self {
            temps,
            zip_path: zip_path.to_string(),
        })
    }

    pub fn matches(&self, path: &[Vec2], min_score: f32) -> eyre::Result<String> {
        let gest = path_to_gest(path);

        let res = find_matching_template_with_defaults(&self.temps, &gest);
        if let Ok((t, v)) = res {
            if v > min_score {
                tracing::info!(gest_name = &t.name, score = v);

                return Ok(t.name.clone());
            }
        }

        Err(eyre::eyre!("Found Nothing"))
    }

    pub fn push(&mut self, name: String, path: &[Vec2]) -> eyre::Result<()> {
        let gest = path_to_gest(path);
        let temp = Template::new(name, &gest).or_else(|e| Err(eyre::eyre!("{e:#?}")))?;

        self.temps.push(temp);

        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> eyre::Result<()> {
        todo!()
    }

    pub fn save(&self) -> eyre::Result<()> {
        let mut file = File::create(self.zip_path.as_str())?;
        let mut zip = ZipWriter::new(&mut file);
        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .compression_level(Some(9));

        for temp in self.temps.iter() {
            let name = temp.name.as_str();
            let points = &temp.path.points();

            let mut buf = Vec::with_capacity(points.len() * 2 * 4);

            for (x, y) in points.iter() {
                let x = x.to_be_bytes();
                let y = y.to_be_bytes();

                buf.extend_from_slice(&x);
                buf.extend_from_slice(&y);
            }

            zip.start_file(name, options.clone())?;
            zip.write(buf.as_slice())?;
        }

        zip.finish()?;

        Ok(())
    }
}

fn path_to_gest(data: &[Vec2]) -> Path2D {
    let mut res = Path2D::default();

    for Vec2 { x, y } in data.iter() {
        res.push(*x, *y);
    }

    res
}

fn load_gesture(name: String, data: &[u8]) -> eyre::Result<Template> {
    let mut gest = Path2D::default();

    for v in data.chunks(8).into_iter() {
        let [x1, x2, x3, x4, y1, y2, y3, y4] = *v else {
            unreachable!()
        };
        let x = f32::from_be_bytes([x1, x2, x3, x4]);
        let y = f32::from_be_bytes([y1, y2, y3, y4]);

        gest.push(x, y);
    }

    Ok(Template::new(name, &gest).or_else(|e| Err(eyre::eyre!("{e:#?}")))?)
}
