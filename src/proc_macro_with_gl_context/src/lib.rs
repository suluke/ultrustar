use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Block, ItemFn, Result, Token,
};

/// Parser for the "DSL" in the macro arguments
struct MacroArgs {
    global_id: Ident,
    binding: Ident,
}
impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let global_id = input.parse::<Ident>()?;
        let _as = input.parse::<Token![as]>()?;
        let binding = input.parse::<Ident>()?;
        Ok(Self { global_id, binding })
    }
}

fn patch(body: Box<Block>, thread_local: Ident, binding: Ident) -> TokenStream {
    quote! {{
        #thread_local.with(|#binding| {
            let scope = #binding.borrow();
            let #binding = scope
            .as_ref()
            .expect("WebGlRenderingContext not set for current thread");
            #body
        })
    }}
}

#[proc_macro_attribute]
pub fn with_gl_context(
    attr: proc_macro::TokenStream,
    tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // parse arguments to the macro
    let MacroArgs { global_id, binding } = parse_macro_input!(attr as MacroArgs);
    // parse the function the macro is applied to
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(tokens as ItemFn);
    // fold other function attributes to token stream
    let attrs = attrs.iter().fold(TokenStream::new(), |mut acc, attr| {
        attr.to_tokens(&mut acc);
        acc
    });
    // patch the code
    let block = patch(block, global_id, binding);
    // rebuild the transformed code
    quote! {
        #attrs
        #vis #sig
        #block
    }
    .into()
}
