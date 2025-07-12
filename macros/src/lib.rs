extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Expr, ExprLit, ItemFn, Lit, Meta, MetaNameValue,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
};

struct MetaNameValueList {
    pairs: Punctuated<MetaNameValue, Comma>,
}

impl Parse for MetaNameValueList {
    fn parse(input: ParseStream) -> Result<Self> {
        let pairs = Punctuated::parse_terminated(input)?;
        Ok(MetaNameValueList { pairs })
    }
}

#[proc_macro_attribute]
pub fn problem(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let register_fn_ident = format_ident!("register_{}", fn_name);

    let attr_str = attr.to_string();
    let wrapped = format!("dummy({})", attr_str);

    let meta = match syn::parse_str::<Meta>(&wrapped) {
        Ok(m) => m,
        Err(err) => return err.to_compile_error().into(),
    };

    let meta_list = match meta {
        Meta::List(meta_list) => meta_list,
        _ => {
            return syn::Error::new_spanned(meta, "Expected meta list")
                .to_compile_error()
                .into();
        }
    };

    let pairs = match syn::parse2::<MetaNameValueList>(meta_list.tokens.clone()) {
        Ok(list) => list.pairs,
        Err(err) => return err.to_compile_error().into(),
    };

    let mut id: Option<String> = None;
    let mut difficulty: Option<u8> = None;

    for pair in pairs {
        if pair.path.is_ident("id") {
            match expr_to_lit_str(&pair.value) {
                Some(s) => id = Some(s),
                None => {
                    return syn::Error::new_spanned(
                        &pair.value,
                        "Expected string literal for `id`",
                    )
                    .to_compile_error()
                    .into();
                }
            }
        } else if pair.path.is_ident("difficulty") {
            match expr_to_lit_int(&pair.value) {
                Some(n) => difficulty = Some(n),
                None => {
                    return syn::Error::new_spanned(
                        &pair.value,
                        "Expected integer literal for `difficulty`",
                    )
                    .to_compile_error()
                    .into();
                }
            }
        } else {
            return syn::Error::new_spanned(pair.path, "Unknown argument")
                .to_compile_error()
                .into();
        }
    }

    let id = match id {
        Some(v) => v,
        None => {
            return syn::Error::new_spanned(&meta_list.path, "Missing 'id'")
                .to_compile_error()
                .into();
        }
    };

    let difficulty = match difficulty {
        Some(v) => v,
        None => {
            return syn::Error::new_spanned(&meta_list.path, "Missing 'difficulty'")
                .to_compile_error()
                .into();
        }
    };

    // 👇 Get the file stem (file name without extension)
    let file_stem_expr = quote! {{
        std::path::Path::new(file!())
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    }};

    // 👇 Build final ID expression at runtime
    let id_expr = quote! {{
        let file_prefix = #file_stem_expr;
        format!("{}_{}", file_prefix, #id)
    }};

    let expanded = quote! {
        #input_fn

        #[cfg(not(target_arch = "wasm32"))]
        #[ctor::ctor]
        fn #register_fn_ident() {
            use crate::backend::{PROBLEM_REGISTRY, ProblemType};
            let final_id = #id_expr;
            PROBLEM_REGISTRY.lock().unwrap().insert(
                final_id.clone(),
                ProblemType {
                    name: final_id,
                    generator: #fn_name,
                    difficulty: #difficulty,
                },
            );
        }

        #[cfg(target_arch = "wasm32")]
        pub fn #register_fn_ident() {
            use crate::backend::{PROBLEM_REGISTRY, ProblemType};
            let final_id = #id_expr;
            PROBLEM_REGISTRY.lock().unwrap().insert(
                final_id.clone(),
                ProblemType {
                    name: final_id,
                    generator: #fn_name,
                    difficulty: #difficulty,
                },
            );
        }
    };

    TokenStream::from(expanded)
}

fn expr_to_lit_str(expr: &Expr) -> Option<String> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(s), ..
    }) = expr
    {
        Some(s.value())
    } else {
        None
    }
}

fn expr_to_lit_int(expr: &Expr) -> Option<u8> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Int(i), ..
    }) = expr
    {
        i.base10_parse::<u8>().ok()
    } else {
        None
    }
}
