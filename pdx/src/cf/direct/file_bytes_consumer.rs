pub struct FileBytesConsumer {

}

impl FileBytesConsumer {
    pub fn new() {

    }

    pub fn check_class_name(&self, name: String) {

    }

    pub fn process_class(&self, name: String) {
        self.check_class_name(name);
    }

    pub fn process_file_bytes(&self, name: String) -> bool {
        let is_class = name.ends_with(".class");
        let is_classes_dex = name.eq("classes.dex");

        if !isClass && !isClassesDex {
            return false;
        }

        self.process_class(name);
        return true;
    }
}