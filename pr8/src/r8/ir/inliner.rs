use crate::r8::graph::app_view::AppView;
use crate::r8::ir::lambda_rewriter::LambdaRewriter;
use crate::r8::ir::lens_code_rewriter::LensCodeRewriter;
use crate::r8::ir::lambda_merger::LambdaMerger;

#[derive(Clone, Debug)]
pub struct Inliner { pub lens_code_rewriter: LensCodeRewriter, pub app_view: AppView, pub lambda_merger: LambdaMerger }

impl Inliner {
    pub fn new(app_view: AppView, lens_code_rewriter: LensCodeRewriter, lambda_merger: LambdaMerger) -> Inliner {
        Inliner {
            app_view,
            lens_code_rewriter,
            lambda_merger,
        }
    }
}
