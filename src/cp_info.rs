pub trait CPInfo {
    fn tag(&self) -> u8;
}

pub struct Class {
    name_index: u16,
}

impl CPInfo for Class {
    fn tag(&self) -> u8 {
        7
    }
}

impl Class {
    pub fn new(name_index: u16) -> Self {
        Self { name_index }
    }
}

pub struct Fieldref {
    class_index: u16,
    name_and_type_index: u16,
}

impl CPInfo for Fieldref {
    fn tag(&self) -> u8 {
        9
    }
}

pub struct Methodref {
    class_index: u16,
    name_and_type_index: u16,
}

impl CPInfo for Methodref {
    fn tag(&self) -> u8 {
        10
    }
}

impl Methodref {
    pub fn new(class_index: u16,
               name_and_type_index: u16,) -> Self {
        Self {class_index, name_and_type_index }
    }
}

pub struct InterfaceMethodref {
    class_index: u16,
    name_and_type_index: u16,
}

impl CPInfo for InterfaceMethodref {
    fn tag(&self) -> u8 {
        11
    }
}

pub struct String {
    string_index: u16,
}

impl CPInfo for String {
    fn tag(&self) -> u8 {
        8
    }
}

pub struct Integer {
    bytes: u32,
}

impl CPInfo for Integer {
    fn tag(&self) -> u8 {
        3
    }
}

pub struct Float {
    bytes: u32,
}

impl CPInfo for Float {
    fn tag(&self) -> u8 {
        4
    }
}

pub struct Long {
    high_bytes: u32,
    low_bytes: u32,
}

impl CPInfo for Long {
    fn tag(&self) -> u8 {
        5
    }
}

pub struct Double {
    high_bytes: u32,
    low_bytes: u32,
}

impl CPInfo for Double {
    fn tag(&self) -> u8 {
        6
    }
}

pub struct NameAndType {
    name_index: u16,
    descriptor_index: u16,
}

impl CPInfo for NameAndType {
    fn tag(&self) -> u8 {
        12
    }
}

pub struct Utf8 {
    length: u16,
    bytes: Vec<u8>,
}

impl CPInfo for Utf8 {
    fn tag(&self) -> u8 {
        1
    }
}

pub struct MethodHandle {
    reference_kind: u8,
    reference_index: u8,
}

impl CPInfo for MethodHandle {
    fn tag(&self) -> u8 {
        15
    }
}

pub struct MethodType {
    descriptor_index: u16,
}

impl CPInfo for MethodType {
    fn tag(&self) -> u8 {
        16
    }
}

pub struct Dynamic {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
}

impl CPInfo for Dynamic {
    fn tag(&self) -> u8 {
        17
    }
}

pub struct InvokeDynamic {
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
}

impl CPInfo for InvokeDynamic {
    fn tag(&self) -> u8 {
        18
    }
}

pub struct Module {
    name_index: u16,
}

impl CPInfo for Module {
    fn tag(&self) -> u8 {
        19
    }
}

pub struct Package {
    name_index: u16,
}

impl CPInfo for Package {
    fn tag(&self) -> u8 {
        20
    }
}