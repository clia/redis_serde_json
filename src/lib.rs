#![recursion_limit = "128"]

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(RedisJsonValue)]
pub fn redis_json_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_type = input.ident;

    let expanded = quote! {
        impl redis::ToRedisArgs for &#struct_type {
            fn write_redis_args<W>(&self, out: &mut W)
            where
                W: ?Sized + RedisWrite
            {
                        out.push(serde_json::to_vec(self).expect("successful json encoding"));
            }
        }

        impl redis::FromRedisValue for #struct_type {
            fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
                match *v {
                    redis::Value::Data(ref bytes) => Ok(serde_json::from_slice(bytes)
                                                        .map_err(|err| {
                                                            (
                                                                redis::ErrorKind::TypeError,
                                                                "json deserialize failed",
                                                                err.to_string(),
                                                            )
                                                        })?),
                    _ => Err((
                        redis::ErrorKind::TypeError,
                        "invalid response type for json",
                    )
                             .into()),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
