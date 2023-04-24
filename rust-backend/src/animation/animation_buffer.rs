use std::{cell::RefCell, fs::File, io::Write};

pub struct AnimationBuffer {
    file: RefCell<File>,
}
impl AnimationBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_file(file: String) -> Self {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file)
            .unwrap();
        let _ = file.write_all("rust_000\n".as_bytes());
        Self {
            file: RefCell::new(file),
        }
    }
    pub fn write(&self, mut data: String) {
        data.push('\n');
        self.file.borrow_mut().write_all(data.as_bytes()).unwrap();
    }
}
impl Default for AnimationBuffer {
    fn default() -> Self {
        Self::with_file(String::from("animation.log"))
    }
}
