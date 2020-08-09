use std::fs::{File, OpenOptions};
use std::io::{Read};

pub struct Reader {
    file: File,
}

impl Reader {
    pub fn new(name: &String) -> Self {
        let file_result = OpenOptions::new()
            .read(true)
            .open(name);

        if file_result.is_ok() {
            Self {
                file: file_result.unwrap(),
            }
        } else {
            panic!("Unable to open file: {}", name);
        }
    }

    pub fn read_file(&mut self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let content = self.file.read_to_end(&mut bytes);
        return match content {
            Ok(_) => bytes,
            Err(e) => {
                panic!("Error reading data: {}", e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::Reader;
    use std::path::PathBuf;

    // Magic, minor, and major source values are copied from:
    // https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html
    #[test]
    fn read_class() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        path.push("test");
        path.push("simple");
        path.push("Simple.class");

        let filename = path.as_os_str().to_str().unwrap().to_string();
        let mut reader = Reader::new(&filename);
        let bytes = reader.read_file();
        println!("Bytes: {:?}", bytes);

        let mut magic_array: [u8; 4] = Default::default(); //bytes.split_at(4).0;
        magic_array.copy_from_slice(&bytes[0..4]);
        let magic_int = u32::from_be_bytes(magic_array);
        println!("Magic: {}", magic_int);
        assert_eq!(0xCAFEBABE, magic_int);

        let mut minor_array: [u8; 2] = Default::default();
        minor_array.copy_from_slice(&bytes[4..6]);
        let minor_int = i16::from_be_bytes(minor_array);
        println!("Minor: {}", minor_int);
        assert_eq!(0, minor_int);

        let mut major_array: [u8; 2] = Default::default();
        major_array.copy_from_slice(&bytes[6..8]);
        let major_int = i16::from_be_bytes(major_array);
        println!("Major: {}", major_int);
        assert_eq!(58, major_int);
    }
}