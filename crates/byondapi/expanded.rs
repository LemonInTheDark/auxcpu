#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use auxcpu_core as auxcpu;
use meowtonin::{ByondValue, byond_fn};
use std::ffi::c_char;
pub fn find_signatures() -> bool {
    auxcpu::find_signatures().expect("failed to find cpu value signatures");
    true
}
#[doc(hidden)]
mod __byond_export_find_signatures {
    use super::*;
    fn __byond_find_signatures_inner() -> ::std::result::Result<
        ::meowtonin::ByondValue,
        ::std::boxed::Box<dyn ::std::error::Error>,
    > {
        let mut __func = move || -> bool {
            {
                auxcpu::find_signatures().expect("failed to find cpu value signatures");
                true
            }
        };
        let ret = __func();
        ::meowtonin::ByondValue::new_value(ret).map_err(::std::boxed::Box::from)
    }
    #[no_mangle]
    #[inline(never)]
    pub unsafe extern "C" fn find_signatures(
        __argc: ::meowtonin::sys::u4c,
        __argv: *mut ::meowtonin::ByondValue,
    ) -> ::meowtonin::ByondValue {
        ::meowtonin::panic::setup_panic_hook();
        match ::std::panic::catch_unwind(move || { __byond_find_signatures_inner() }) {
            Ok(Ok(value)) => value,
            Ok(Err(err)) => {
                let error = err.to_string();
                let source = "find_signatures".to_string();
                let _ = ::meowtonin::call_global::<
                    _,
                    _,
                    _,
                    (),
                >("meowtonin_stack_trace", [error, source]);
                ::meowtonin::ByondValue::null()
            }
            Err(_err) => {
                ::meowtonin::panic::stack_trace_if_panic();
                ::meowtonin::ByondValue::null()
            }
        }
    }
}
