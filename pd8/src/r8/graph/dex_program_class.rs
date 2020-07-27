use crate::r8::graph::method_collection::MethodCollection;

pub struct DexProgramClass {
    this_type: String,
    origin_kind: String,
    origin: String,
    access_flags: String,
    super_type: String,
    interfaces: String,
    source_file: String,
    nest_host: String,
    nest_members: String,
    enclosing_member: String,
    inner_classes: String,
    class_annotations: String,
    static_fields: String,
    instance_fields: String,
    direct_methods: String,
    virtual_methods: String,

    method_collection: MethodCollection
}

impl DexProgramClass {
    pub fn new() -> DexProgramClass {
        DexProgramClass {
            this_type: "".to_string(),
            origin_kind: "".to_string(),
            origin: "".to_string(),
            access_flags: "".to_string(),
            super_type: "".to_string(),
            interfaces: "".to_string(),
            source_file: "".to_string(),
            nest_host: "".to_string(),
            nest_members: "".to_string(),
            enclosing_member: "".to_string(),
            inner_classes: "".to_string(),
            class_annotations: "".to_string(),
            static_fields: "".to_string(),
            instance_fields: "".to_string(),
            direct_methods: "".to_string(),
            virtual_methods: "".to_string(),
            method_collection: MethodCollection::new()
        }
    }

    pub fn get_class_initializer(&self) {
        self.method_collection.get_class_initializer()
    }
}
