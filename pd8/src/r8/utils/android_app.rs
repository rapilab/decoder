pub trait ClassFileResourceProvider {
    fn get_class_descriptors(&self);
    fn get_program_resource(&self);
}

pub struct AndroidApp {
    classpath_resource_providers: Vec<Box<dyn ClassFileResourceProvider>>,
    library_resource_providers: Vec<Box<dyn ClassFileResourceProvider>>,
}

impl AndroidApp {}