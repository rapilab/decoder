use crate::r8::graph::app_view::AppView;
use crate::r8::graph::dex_application::{DexApplication, DexApplicationBuilder};
use crate::r8::graph::dex_encoded_method::DexEncodedMethod;
use crate::r8::graph::dex_program_class::DexProgramClass;
use crate::r8::graph::lazy_loaded_dex_application::LazyLoadedDexApplicationBuilder;
use crate::r8::graph::program_method::ProgramMethod;
use std::time::Instant;
use std::ptr::null;
use crate::r8::ir::conversion::one_time_method_processor::OneTimeMethodProcessor;
use crate::r8::ir::code::ir_code::IRCode;
use crate::r8::ir::lens_code_rewriter::LensCodeRewriter;
use crate::r8::ir::lambda_rewriter::LambdaRewriter;
use crate::r8::ir::inliner::Inliner;
use crate::r8::ir::lambda_merger::LambdaMerger;
use crate::r8::ir::service_loader_rewriter::ServiceLoaderRewriter;

pub struct IRConverter {
    app_view: AppView,
    pub lens_code_rewriter: LensCodeRewriter,
    pub lambda_rewriter: LambdaRewriter,
    pub inliner: Inliner,
    pub lambda_merger: LambdaMerger,
    pub service_loader_rewriter: ServiceLoaderRewriter
}

impl IRConverter {
    pub fn new(app_view: AppView) -> IRConverter {
        let lens_code_rewriter = LensCodeRewriter::new(app_view.clone());
        let lambda_rewriter = LambdaRewriter::new(app_view.clone());
        let lambda_merger = LambdaMerger::new(app_view.clone());
        let inliner = Inliner::new(app_view.clone(),  lens_code_rewriter.clone(), lambda_merger.clone());
        let service_loader_rewriter = ServiceLoaderRewriter::new(app_view.clone());

        IRConverter {
            app_view, lens_code_rewriter, lambda_rewriter, lambda_merger, inliner, service_loader_rewriter
        }
    }

    pub fn rewrite_code(&self, method: ProgramMethod, processor: OneTimeMethodProcessor) {
        let code = method.build_ir(self.app_view.clone());
        self.optimize(code, processor)
    }

    pub fn optimize(&self, code: IRCode, processor: OneTimeMethodProcessor) {
        let context = code.context();
        let method = context.clone().method;
        let holder = context.clone().holder;

        println!("Initial (SSA) flow graph {:?}", Instant::now().elapsed().as_secs());

        println!("Lens rewrite");
        self.lens_code_rewriter.rewrite(code.clone(), context);

        println!("Desugar lambdas");
        self.lambda_rewriter.desugar_lambdas(code.clone());

        println!("Merge lambdas");
        self.lambda_merger.rewrite_code(code.context(), code.clone(), self.inliner.clone());

        println!("Rewrite service loaders");
        self.service_loader_rewriter.rewrite(code.clone());

    }
    pub fn convert_method(&self, method: ProgramMethod) {
        // let definition = method.get_definition();
        // if definition.get_code() {
        let processor = OneTimeMethodProcessor::create(method.clone(), self.app_view.clone());
        self.rewrite_code(method, processor);
        // }
    }

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
