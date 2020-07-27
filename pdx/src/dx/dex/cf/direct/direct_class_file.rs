use std::path::PathBuf;

use crate::dx::dex::rop::cst::cst_type::CstType;
use crate::dx::dex::rop::cst::std_constant_pool::StdConstantPool;
use crate::dx::dex::rop::iface::field_list::FieldList;
use crate::dx::dex::rop::iface::method_list::MethodList;
use crate::dx::dex::rop::iface::std_attribute_list::StdAttributeList;
use crate::dx::dex::rop::iface::type_list::TypeList;

#[derive(Debug, Clone)]
pub struct DirectClassFile {
    name: PathBuf,
    bytes: Vec<u8>,

    constant_pool: StdConstantPool,
    minor_version: i32,
    major_version: i32,
    access_flags: i32,
    magic: i32,
    this_class: CstType,
    super_class: CstType,
    interfaces: TypeList,
    fields: FieldList,
    methods: MethodList,
    attributes: StdAttributeList,
}

impl DirectClassFile {
    pub fn new(name: PathBuf, bytes: Vec<u8>) -> DirectClassFile {
        DirectClassFile {
            name,
            bytes,
            constant_pool: StdConstantPool::new(),
            minor_version: 0,
            major_version: 0,
            access_flags: 0,
            magic: 0,
            this_class: CstType::new(),
            super_class: CstType::new(),
            interfaces: TypeList {},
            fields: FieldList {},
            methods: MethodList {},
            attributes: StdAttributeList {},
        }
    }

    pub fn set_attribute_factory(&self) {}

    pub fn is_good_magic(&self, magic: i32) -> bool {
        return true;
    }

    pub fn get_magic0(&self) -> i32 {
        return 0;
    }

    pub fn parse0(&self) {
        if self.bytes.len() < 10 {
            // throw error
        }
        if self.is_good_magic(self.get_magic0()) {}
        //todo: is good version
    }

    pub fn parse_to_interfaces_if_necessary(&self) {
        self.parse0();
    }

    pub fn get_magic(&self) -> u8 {
        self.parse_to_interfaces_if_necessary();
        self.bytes[0]
    }

    pub fn get_source_file(&self) {}

    pub fn get_constant_pool(&self) -> StdConstantPool {
        self.constant_pool.clone()
    }
}
