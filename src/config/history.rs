use std::path::{Path, PathBuf};

// rmg 2
//   # book_list[len - 2]
// rmg
//   # book_list[len - 1]

struct History {
    inner: Vec<MangaHistory>,
}

struct MangaHistory {
    path: PathBuf,
    freq: u32,
    page_number: u32,
    page_bookmark: Option<u32>,
}

impl History {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn add(&mut self, val: MangaHistory) {
        for (idx, history) in self.inner.iter().enumerate() {
            if &history.path == &val.path {
                self.inner[idx].freq_add();
                return;
            }
        }

        self.inner.push(val);
    }
}

impl MangaHistory {
    fn new(path: &Path, freq: u32, page_number: u32) -> Self {
        Self {
            path: path.to_path_buf(),
            freq,
            page_bookmark: None,
            page_number,
        }
    }

    fn set_page_bookmark(&mut self, idx: usize) {
        self.page_bookmark = Some(idx as u32);
    }

    fn set_page_number(&mut self, idx: usize) {
        self.page_number = idx as u32;
    }

    fn freq_add(&mut self) {
        self.freq += 1;
    }
}
