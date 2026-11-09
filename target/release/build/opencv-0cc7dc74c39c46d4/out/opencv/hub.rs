#[cfg(ocvrs_has_module_core)]
include!(concat!(env!("OUT_DIR"), "/opencv/core.rs"));
#[cfg(ocvrs_has_module_dnn)]
include!(concat!(env!("OUT_DIR"), "/opencv/dnn.rs"));
#[cfg(ocvrs_has_module_highgui)]
include!(concat!(env!("OUT_DIR"), "/opencv/highgui.rs"));
#[cfg(ocvrs_has_module_imgproc)]
include!(concat!(env!("OUT_DIR"), "/opencv/imgproc.rs"));
#[cfg(ocvrs_has_module_videoio)]
include!(concat!(env!("OUT_DIR"), "/opencv/videoio.rs"));
pub mod types {
	include!(concat!(env!("OUT_DIR"), "/opencv/types.rs"));
}
#[doc(hidden)]
pub mod sys {
	include!(concat!(env!("OUT_DIR"), "/opencv/sys.rs"));
}
pub mod hub_prelude {
	#[cfg(ocvrs_has_module_core)]
	pub use super::core::prelude::*;
	#[cfg(ocvrs_has_module_dnn)]
	pub use super::dnn::prelude::*;
	#[cfg(ocvrs_has_module_highgui)]
	pub use super::highgui::prelude::*;
	#[cfg(ocvrs_has_module_imgproc)]
	pub use super::imgproc::prelude::*;
	#[cfg(ocvrs_has_module_videoio)]
	pub use super::videoio::prelude::*;
}

mod ffi_exports {
	use crate::mod_prelude_sys::*;
	#[no_mangle] unsafe extern "C" fn ocvrs_create_string_0_93_7(s: *const c_char) -> *mut String { crate::templ::ocvrs_create_string(s) }
	#[no_mangle] unsafe extern "C" fn ocvrs_create_byte_string_0_93_7(v: *const u8, len: size_t) -> *mut Vec<u8> { crate::templ::ocvrs_create_byte_string(v, len) }
}
