use std::fs::{File, OpenOptions};
use std::io::{Read};
use crate::classfile::ClassFile;
use crate::cp_info::{CPInfo, Class, Methodref};

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
        };
    }

    pub fn read_class_file(&mut self) -> ClassFile {
        let bytes = self.read_file();

        let mut magic_array: [u8; 4] = Default::default();
        magic_array.copy_from_slice(&bytes[0..4]);
        let magic_int = u32::from_be_bytes(magic_array);

        let mut minor_array: [u8; 2] = Default::default();
        minor_array.copy_from_slice(&bytes[4..6]);
        let minor_int = u16::from_be_bytes(minor_array);

        let mut major_array: [u8; 2] = Default::default();
        major_array.copy_from_slice(&bytes[6..8]);
        let major_int = u16::from_be_bytes(major_array);

        let mut cpc_array: [u8; 2] = Default::default();
        cpc_array.copy_from_slice(&bytes[8..10]);
        let cpc_int = u16::from_be_bytes(cpc_array);

        let cp_infos: Vec<Box<dyn CPInfo>> = self.read_cp_infos(bytes[10..].to_vec());

        ClassFile::new(magic_int, minor_int, major_int, cpc_int, cp_infos)
    }

    pub fn read_cp_infos(&self, bytes: Vec<u8>) -> Vec<Box<dyn CPInfo>> {
        let mut cp_infos: Vec<Box<dyn CPInfo>> = Vec::new();

        let mut index = 0;
        while index < bytes.len() {
            let mut tag = bytes.get(index).unwrap();
            match tag {
                7 => {
                    println!("Got 7");
                    let mut name_index_arr: [u8; 2] = Default::default();
                    name_index_arr.copy_from_slice(&bytes[1..3]);
                    let name_index = u16::from_be_bytes(name_index_arr);
                    cp_infos.push(Box::new(Class::new(name_index)))
                }
                10 => {
                    println!("Got 10");
                    let mut class_index_arr: [u8; 2] = Default::default();
                    let i1_start = index + 1;
                    let i1_end = index + 3;
                    class_index_arr.copy_from_slice(&bytes[i1_start..i1_end]);
                    let class_index = u16::from_be_bytes(class_index_arr);

                    let mut nati_arr: [u8; 2] = Default::default();
                    let i2_start = index + 3;
                    let i2_end = index + 5;
                    nati_arr.copy_from_slice(&bytes[i2_start..i2_end]);
                    let nati = u16::from_be_bytes(nati_arr);

                    cp_infos.push(Box::new(Methodref::new(class_index, nati)));
                    index = index + 4;
                }
                _ => panic!("Unknown tag: {}", tag)
            }
        }

        cp_infos
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
        let mut class_file = reader.read_class_file();

        let magic_int = class_file.magic_number();
        println!("Magic: {}", magic_int);
        assert_eq!(&0xCAFEBABE, magic_int);

        let minor_int = class_file.minor_version();
        println!("Minor: {}", minor_int);
        assert_eq!(&0, minor_int);

        let major_int = class_file.major_version();
        println!("Major: {}", major_int);
        assert_eq!(&58, major_int);

        let cpc_int = class_file.constant_pool_count();
        println!("Constant Pool Count: {}", cpc_int);
        assert_eq!(&29, cpc_int);
    }
}