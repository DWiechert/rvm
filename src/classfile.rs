use crate::cp_info::CPInfo;

pub struct ClassFile {
    magic_number: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
}

impl ClassFile {
    pub fn new(
        magic_number: u32,
        minor_version: u16,
        major_version: u16,
        constant_pool_count: u16,
        cp_infos: Vec<Box<dyn CPInfo>>,
        // access_flags: u16,
        // this_class: u16,
        // super_class: u16,
        // interfaces_count: u16,
        //
    ) -> Self {
        Self {
            magic_number,
            minor_version,
            major_version,
            constant_pool_count,
        }
    }

    pub fn magic_number(&self) -> &u32 {
        &self.magic_number
    }

    pub fn minor_version(&self) -> &u16 {
        &self.minor_version
    }

    pub fn major_version(&self) -> &u16 {
        &self.major_version
    }

    pub fn constant_pool_count(&self) -> &u16 {
        &self.constant_pool_count
    }
}