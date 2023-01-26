use crate::http::MethodType;
use crate::syn::NestedMeta::Lit;
use quote::__private::Span;
use serde::Serialize;
use syn;
use syn::Lit::Str;

#[derive(Serialize)]
pub enum AuthLevel {
    Anonymous,
    Function,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BindingType {
    HttpTrigger,
    Http,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_level: Option<AuthLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    methods: Option<Vec<MethodType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route: Option<String>,

    #[serde(rename(serialize = "type"))]
    binding_type: BindingType,
    name: String,
    direction: String,
}

pub fn create_http_trigger_bindings(
    method: MethodType,
    route: String,
    auth_level: AuthLevel,
) -> Vec<Binding> {
    vec![
        Binding {
            auth_level: Some(auth_level),
            binding_type: BindingType::HttpTrigger,
            direction: "int".to_string(),
            name: "req".to_string(),
            methods: Some(vec![method]),
            route: Some(route),
        },
        Binding {
            binding_type: BindingType::Http,
            direction: "out".to_string(),
            name: "res".to_string(),
            auth_level: None,
            methods: None,
            route: None,
        },
    ]
}

pub fn parse_auth_level(input: &syn::NestedMeta) -> Result<AuthLevel, syn::Error> {
    let err = Err(syn::Error::new(
        Span::call_site(),
        "Invalid Authentication Level provided.",
    ));

    if let Lit(Str(s)) = input {
        return match s.value().as_ref() {
            "Function" => Ok(AuthLevel::Function),
            "Anonymous" => Ok(AuthLevel::Anonymous),
            _ => err,
        };
    }

    err
}
