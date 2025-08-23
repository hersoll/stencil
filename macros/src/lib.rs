extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn problem(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let register_fn_ident = format_ident!("register_{}", fn_name);

    // Get the file stem (file name without extension)
    let file_stem_expr = quote! {{
        std::path::Path::new(file!())
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    }};

    // Build final ID expression at runtime using function name
    let fn_name_str = fn_name.to_string();
    let id_expr = quote! {{
        let file_prefix = #file_stem_expr;
        format!("{}_{}", file_prefix, #fn_name_str)
    }};

    let expanded = quote! {
        #input_fn

        #[cfg(not(target_arch = "wasm32"))]
        #[ctor::ctor]
        fn #register_fn_ident() {
            use crate::backend::PROBLEM_MAP;
            let final_id = #id_expr;
            PROBLEM_MAP.write().unwrap().insert(
                final_id,
                #fn_name,
            );
        }

        #[cfg(target_arch = "wasm32")]
        pub fn #register_fn_ident() {
            use crate::backend::PROBLEM_MAP;
            let final_id = #id_expr;
            PROBLEM_MAP.write().unwrap().insert(
                final_id,
                #fn_name,
            );
        }
    };

    TokenStream::from(expanded)
}
