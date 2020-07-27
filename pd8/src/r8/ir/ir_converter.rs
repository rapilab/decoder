use crate::r8::graph::app_view::AppView;
use crate::r8::graph::dex_application::DexApplication;
use std::time::Instant;
use crate::r8::graph::dex_program_class::DexProgramClass;
use crate::r8::graph::dex_encoded_method::DexEncodedMethod;
use crate::r8::graph::program_method::ProgramMethod;

pub struct IRConverter {
    app_view: AppView
}

impl IRConverter {
    pub fn new(app_view: AppView) -> IRConverter {
        IRConverter {
            app_view
        }
    }

    pub fn convert_method(&self, method: ProgramMethod) {

    }

    pub fn convert_methods(&self, clazz: DexProgramClass) {
        let option = clazz.clone().get_class_initializer();
        match option {
            None => {},
            Some(method) => {
                let init_method = ProgramMethod::new(clazz.clone(), method);
                self.convert_method(init_method)
            },
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

        println!("IR end: {}", Instant::now().elapsed().as_secs());
        app
    }
}

