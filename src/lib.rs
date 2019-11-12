#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
pub mod macros; /* Handy macros */
pub mod vpp;
mod vpp_int; /* VPP bindings */

// use vpp::*;
use std::convert::TryInto;
use std::mem::size_of;
use std::ptr;

static mut test_vlib_main: Option<*mut vpp_int::vlib_main_t> = None;

pub fn set_vlib_main(vm: *mut vpp::vlib_main_t) {
    unsafe {
        test_vlib_main = Some(vm);
    }
}

#[macro_export]
macro_rules! register_plugin {
    ($init_func: ident, $desc: expr) => {
        #[no_mangle]
        pub extern "C" fn rust_plugin_init(
            vm: *mut rust_vpp::vpp::vlib_main_t,
        ) -> *const rust_vpp::vpp::clib_error_t {
            rust_vpp::set_vlib_main(vm);
            $init_func();
            return std::ptr::null();
        }

        #[link_section = ".vlib_plugin_registration"]
        #[no_mangle]
        pub static mut vlib_plugin_registration: rust_vpp::vpp::vlib_plugin_registration_t =
            rust_vpp::vpp::vlib_plugin_registration_t {
                default_disabled: 0,
                version: [0; 32], // FIXME: fill in the version
                version_required: [0; 32],
                early_init: cstr!("rust_plugin_init"),
                description: cstr!($desc),
            };
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
