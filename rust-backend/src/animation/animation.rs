use std::{cell::RefCell, fmt::Debug, fs::File, io::Write, rc::Rc};

pub trait NewInstance<T: std::fmt::Debug> {
    fn new(buffer: &AnimationBuffer) -> Self;
}

pub(super) trait AnimationBufferWrite {
    fn write<T: std::fmt::Debug>(&self, operation: &str, data: &T);
    fn write_raw<T: Debug>(&self, opertations: Vec<&str>, data: Vec<&T>) {}
}

pub(super) trait AnimatedFromIterator<A>: Sized {
    fn from_iter<T>(buffer: &AnimationBuffer, iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}

pub struct AnimationBuffer {
    file: Rc<RefCell<File>>,
    data_structure: Option<String>,
    data_structure_id: Option<u64>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum DataStructure {
    Stack,
    Queue,
}

impl DataStructure {
    pub fn to_str(&self) -> &str {
        match self {
            DataStructure::Stack => "stack",
            DataStructure::Queue => "queue",
        }
    }
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
            file: Rc::new(RefCell::new(file)),
            data_structure: None,
            data_structure_id: None,
        }
    }

    pub(super) fn with_ds(&self, ds: DataStructure) -> Self {
        Self {
            file: self.file.clone(),
            data_structure: Some(ds.to_str().into()),
            data_structure_id: Some(fastrand::u64(..)),
        }
    }
}

impl Default for AnimationBuffer {
    fn default() -> Self {
        Self::with_file(String::from("animation.log"))
    }
}

impl AnimationBufferWrite for AnimationBuffer {
    fn write<T: std::fmt::Debug>(&self, operation: &str, data: &T) {
        if let Some(ds) = self.data_structure.as_ref() {
            let content_to_write = format!(
                "{0}_{1}:{2}({3:?})\n",
                ds,
                self.data_structure_id
                    .expect("Data structure id should be set before writing to file"),
                operation,
                data
            );
            self.file
                .borrow_mut()
                .write_all(content_to_write.as_bytes())
                .unwrap();
        } else {
            panic!("Data structure should be set before writing to file");
        }
    }
    fn write_raw<T: Debug>(&self, opertations: Vec<&str>, data: Vec<&T>) {
        if let Some(ds) = self.data_structure.as_ref() {
            let mut file = self.file.borrow_mut();
            for (operation, data) in opertations.iter().zip(data.iter()) {
                file.write(
                    format!(
                        "{}_{:?}:{}({:?})\n",
                        ds, self.data_structure_id, operation, data
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
        } else {
            panic!("Data structure should be set before writing to file")
        }
    }
}
