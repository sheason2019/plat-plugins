#[allow(warnings)]
mod bindings;
mod methods;
mod store;
mod utils;

use bindings::Guest;
use methods::find_one;

struct Component;

impl Guest for Component {
    fn emit(ty: String, payload: String) -> String {
        let func = match ty.as_str() {
            "find-identity-by-public-key" => find_one,
            _ => panic!("invalid invoke method type: {}", &ty),
        };
        func(payload)
    }
}

bindings::export!(Component with_types_in bindings);
