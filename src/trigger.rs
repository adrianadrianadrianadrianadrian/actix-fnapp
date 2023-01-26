mod binding;
mod http;

use binding::{create_http_trigger_bindings, parse_auth_level, AuthLevel};
use convert_case::{Case, Casing};
use http::{parse_route, MethodType};
use proc_macro::TokenStream;
use quote::__private::Span;
use quote::quote;
use serde_json;
use std::path::PathBuf;
use std::{env, fs};
use std::{fs::File, io::Write};
use syn::{self, parse_macro_input, ItemFn};

const FNAPP_BINDING_DIR: &'static str = "fnapp-bindings";
const CARGO_MANIFEST_DIR: &'static str = "CARGO_MANIFEST_DIR";

#[proc_macro_attribute]
pub fn post_trigger(args: TokenStream, input: TokenStream) -> TokenStream {
    http_trigger(MethodType::POST, args, input)
}

#[proc_macro_attribute]
pub fn put_trigger(args: TokenStream, input: TokenStream) -> TokenStream {
    http_trigger(MethodType::PUT, args, input)
}

#[proc_macro_attribute]
pub fn get_trigger(args: TokenStream, input: TokenStream) -> TokenStream {
    http_trigger(MethodType::GET, args, input)
}

fn http_trigger(method: MethodType, args: TokenStream, input: TokenStream) -> TokenStream {
    let mut bindings_path = PathBuf::from(env::var(CARGO_MANIFEST_DIR).unwrap());
    bindings_path.push(FNAPP_BINDING_DIR);

    let out = input.clone();

    let args = parse_macro_input!(args as syn::AttributeArgs);
    if args.is_empty() {
        return error(
            out,
            syn::Error::new(
                Span::call_site(),
                format!("The #[{}_trigger(..)] macro requires a route & optionally an authentication level.", method.to_string()),
            ),
        );
    }

    let route = match parse_route(&args[0]) {
        Ok(r) => r,
        Err(e) => return error(out, e),
    };
    let auth_level = if let Some(auth_arg) = args.get(1) {
        match parse_auth_level(auth_arg) {
            Ok(a) => a,
            Err(e) => return error(out, e),
        }
    } else {
        AuthLevel::Function
    };

    let func = parse_macro_input!(input as ItemFn);
    let name = func.sig.ident.to_string();

    let trigger_bindings =
        create_http_trigger_bindings(method.clone(), route.to_string(), auth_level);

    let create_bindings = fs::create_dir_all(&bindings_path)
        .and_then(|_| {
            bindings_path.push(name.to_case(Case::Pascal));
            fs::create_dir_all(&bindings_path)
        })
        .and_then(|_| {
            bindings_path.push("function.json");
            File::create(bindings_path)
        })
        .and_then(|mut file| {
            serde_json::to_string_pretty(&trigger_bindings)
                .map_err(|e| e.into())
                .and_then(|content| file.write_all(content.as_bytes()))
        });

    if let Err(e) = create_bindings {
        return error(out, syn::Error::new(Span::call_site(), format!("{:?}", e)));
    }

    let prefixed_route = format!("/{}{}", "api", route);

    let mut output: TokenStream = match method {
        MethodType::POST => quote! { #[post(#prefixed_route)] },
        MethodType::GET => quote! { #[get(#prefixed_route)] },
        MethodType::PUT => quote! { #[put(#prefixed_route)] },
    }
    .into();

    output.extend(out);
    output
}

fn error(mut item: TokenStream, err: syn::Error) -> TokenStream {
    let compile_err = TokenStream::from(err.to_compile_error());
    item.extend(compile_err);
    item
}
