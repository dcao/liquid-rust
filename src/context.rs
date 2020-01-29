use rustc::mir;
use rustc::ty::TyCtxt;
pub use rustc_errors::ErrorReported;
use rustc_hir::BodyId;
use rustc_lint::{LateContext, LintContext};
use rustc_session::declare_lint;
use rustc_span::{MultiSpan, Span};

declare_lint! {
    pub LIQUID_RUST,

    Forbid,
    "liquid rust"
}

pub struct LiquidRustContext<'a, 'tcx> {
    cx: &'a LateContext<'a, 'tcx>,
}

impl<'a, 'tcx> LiquidRustContext<'a, 'tcx> {
    pub fn new(cx: &'a LateContext<'a, 'tcx>) -> Self {
        LiquidRustContext { cx }
    }

    pub fn tcx(&self) -> &TyCtxt<'tcx> {
        &self.cx.tcx
    }

    pub fn hir(&self) -> &rustc::hir::map::Map<'tcx> {
        self.cx.tcx.hir()
    }

    pub fn track_errors<F, T>(&self, f: F) -> Result<T, ErrorReported>
    where
        F: FnOnce() -> T,
    {
        self.cx.sess().track_errors(f)
    }

    pub fn optimized_mir(&self, body_id: BodyId) -> &'tcx mir::BodyAndCache<'tcx> {
        let def_id = self.hir().body_owner_def_id(body_id);
        self.cx.tcx.optimized_mir(def_id)
    }

    pub fn span_lint<S: Into<MultiSpan>>(&self, span: S, msg: &str) {
        self.cx.span_lint(LIQUID_RUST, span, msg);
    }

    pub fn span_lint_label(&self, span: Span, msg: &str) {
        let mut mspan = MultiSpan::from_span(span);
        mspan.push_span_label(span, msg.into());
        self.span_lint(mspan, msg);
    }

    pub fn abort_if_errors(&self) {
        self.cx.sess().abort_if_errors();
    }

    // pub fn span_lint_note(&self, span: Span, msg: &str, note_span: Span, note: &str) {
    //     self.cx
    //         .span_lint_note(LIQUID_RUST, span, msg, note_span, note);
    // }

    // pub fn struct_span_lint<S: Into<MultiSpan>>(&self, span: S, msg: &str) -> DiagnosticBuilder {
    //     self.cx.struct_span_lint(LIQUID_RUST, span, msg)
    // }
}