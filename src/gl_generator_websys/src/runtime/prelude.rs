use std::cell::RefCell;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

thread_local!(static CONTEXT: RefCell<Option<WebGlRenderingContext>> = RefCell::new(None));

pub fn set_context(ctx: WebGlRenderingContext) {
    CONTEXT.with(|tls_ctx| {
        *tls_ctx.borrow_mut() = Some(ctx);
    });
}
macro_rules! withctx {
    ($global:ident, $local:ident, $code:block) => {
        $global.with(|ctx| {
            let scope = ctx.borrow();
            let $local = scope.as_ref().expect("WebGlRenderingContext not set for current thread");
            $code
        })
    };
}

struct Error {
    #[allow(unused)]
    details: JsValue
}
thread_local!(static ERROR: RefCell<Option<Error>> = RefCell::new(None));
trait HandleJsError {
    type Output;
    fn handle_js_error(self);
}
impl<T> HandleJsError for Result<T, JsValue> {
    type Output = T;
    fn handle_js_error(self) {
        if let Err(details) = self {
            ERROR.with(|err| *err.borrow_mut() = Some(Error{details}));
        }
    }
}
