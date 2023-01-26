use crate::syn::NestedMeta::Lit;
use quote::__private::Span;
use serde::Serialize;
use syn;
use syn::Lit::Str;

#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MethodType {
    GET,
    POST,
    PUT,
}

impl MethodType {
    pub fn to_string(&self) -> String {
        match self {
            Self::GET => "get",
            Self::POST => "post",
            Self::PUT => "put",
        }
        .to_string()
    }
}

pub fn parse_route(input: &syn::NestedMeta) -> Result<String, syn::Error> {
    if let Lit(Str(s)) = input {
        return Ok(s.value());
    }

    Err(syn::Error::new(
        Span::call_site(),
        "Invalid route provided.",
    ))
}
