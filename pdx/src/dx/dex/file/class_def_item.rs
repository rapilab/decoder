use crate::dx::dex::file::encoded_method::EncodedMethod;
use crate::dx::dex::rop::cst::cst_string::CstString;
use crate::dx::dex::rop::cst::cst_type::CstType;
use crate::dx::dex::rop::iface::type_list::TypeList;

pub struct ClassDefItem {
    this_class: CstType,
    access_flags: i32,
    superclass: CstType,
    interfaces: TypeList,
    sourcefile: CstString,
    methods: Vec<EncodedMethod>,
}

impl ClassDefItem {
    pub fn new() -> ClassDefItem {
        ClassDefItem {
            this_class: CstType {},
            access_flags: 0,
            superclass: CstType {},
            interfaces: TypeList {},
            sourcefile: CstString {},
            methods: vec![],
        }
    }
}
