use crate::r8::graph::app_view::AppView;
use crate::r8::graph::dex_application::{DexApplication, DexApplicationBuilder};
use crate::r8::graph::dex_encoded_method::DexEncodedMethod;
use crate::r8::graph::dex_program_class::DexProgramClass;
use crate::r8::graph::lazy_loaded_dex_application::LazyLoadedDexApplicationBuilder;
use crate::r8::graph::program_method::ProgramMethod;
use std::time::Instant;

pub struct IRConverter {
    app_view: AppView,
}

impl IRConverter {
    pub fn new(app_view: AppView) -> IRConverter {
        IRConverter { app_view }
    }

    pub fn convert_method(&self, method: ProgramMethod) {}

    pub fn convert_methods(&self, clazz: DexProgramClass) {
        let option = clazz.clone().get_class_initializer();
        match option {
            None => {}
            Some(method) => {
                let init_method = ProgramMethod::new(clazz.clone(), method);
                self.convert_method(init_method)
            }
        }
        for d_method in clazz.clone().direct_methods {
            let method1 = ProgramMethod::new(clazz.clone(), d_method);
            self.convert_method(method1)
        }
        for v_method in clazz.clone().virtual_methods {
            let method2 = ProgramMethod::new(clazz.clone(), v_method);
            self.convert_method(method2)
        }
    }

    pub fn convert(&self, app: DexApplication) -> DexApplication {
        println!("IR conversion: {}", Instant::now().elapsed().as_secs());
        for clazz in app.classes() {
            self.convert_methods(clazz);
        }

        let builder = DexApplication::builder();

        self.desugar_nest_based_access(builder.clone());
        self.synthesize_lambda_classes(builder.clone());
        self.desugar_interface_methods(builder.clone());
        self.synthesize_twr_close_resource_utility_class(builder.clone());
        self.synthesize_java8utility_class(builder.clone());
        self.synthesize_retarget_class(builder.clone());

        self.process_covariant_return_type_annotations(builder.clone());
        self.generate_desugared_library_apiwrappers(builder.clone());

        self.handle_synthesized_class_mapping(builder.clone());

        println!("IR end: {}", Instant::now().elapsed().as_secs());

        builder.build()
    }

    pub fn desugar_nest_based_access(&self, builder: DexApplicationBuilder) {}
    pub fn synthesize_lambda_classes(&self, builder: DexApplicationBuilder) {}
    pub fn desugar_interface_methods(&self, builder: DexApplicationBuilder) {}
    pub fn synthesize_twr_close_resource_utility_class(&self, builder: DexApplicationBuilder) {}
    pub fn synthesize_java8utility_class(&self, builder: DexApplicationBuilder) {}
    pub fn synthesize_retarget_class(&self, builder: DexApplicationBuilder) {}
    pub fn process_covariant_return_type_annotations(&self, builder: DexApplicationBuilder) {}
    pub fn generate_desugared_library_apiwrappers(&self, builder: DexApplicationBuilder) {}
    pub fn handle_synthesized_class_mapping(&self, builder: DexApplicationBuilder) {}
}
