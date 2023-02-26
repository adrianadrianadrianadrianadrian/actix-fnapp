# actix-fnapp

A set of macros that'll create the relevant Azure function app binding definitions for your actix endpoints. I abandoned the idea, and just manually updated the bindings.
But here is the usage,

```rust 
use actix_fnapp::get_trigger;
use actix_web::{get, HttpResponse, Responder};

#[get_trigger("/say-hello/{name}")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().finish()
}
```

will output a folder called 'fnapp-bindings' with the relevant bindings inside. `fnapp-bindings/Hello`,

```
[
  {
    "authLevel": "Function",
    "methods": [
      "get"
    ],
    "route": "/say-hello/{name}",
    "type": "httpTrigger",
    "name": "req",
    "direction": "int"
  },
  {
    "type": "http",
    "name": "res",
    "direction": "out"
  }
]
```

