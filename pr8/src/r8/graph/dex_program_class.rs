use crate::r8::graph::dex_encoded_method::DexEncodedMethod;
use crate::r8::graph::method_collection::MethodCollection;

#[derive(Debug, Clone)]
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
    pub direct_methods: Vec<DexEncodedMethod>,
    pub virtual_methods: Vec<DexEncodedMethod>,

    method_collection: Option<MethodCollection>,
}

impl DexProgramClass {
    pub fn new(
        direct_methods: Vec<DexEncodedMethod>,
        virtual_methods: Vec<DexEncodedMethod>,
    ) -> DexProgramClass {
        let mut class = DexProgramClass {
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
            direct_methods: direct_methods.clone(),
            virtual_methods: virtual_methods.clone(),
            method_collection: None,
        };

        let collection =
            MethodCollection::new(Box::from(class.clone()), direct_methods, virtual_methods);
        class.method_collection = Option::from(collection);

        class
    }

    pub fn get_class_initializer(&self) -> Option<DexEncodedMethod> {
        let collection = self.method_collection.as_ref().unwrap();
        collection.get_class_initializer()
    }
}
