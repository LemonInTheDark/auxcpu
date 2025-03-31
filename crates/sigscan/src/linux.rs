use libc::{Elf32_Phdr, PT_LOAD, dl_iterate_phdr, dl_phdr_info};
use std::{
	convert::Infallible,
	ffi::{CStr, CString, c_void},
	os::raw::{c_char, c_int},
};

#[repr(C)]
struct CallbackData {
	module_name_ptr: *const c_char,
	memory_start: usize,
	memory_len: usize,
	memory_area: Option<&'static [u8]>,
}

pub struct Scanner {
	module_name: String,
}

// Replace the dl_phdr_callback function with this implementation:
extern "C" fn dl_phdr_callback(info: *mut dl_phdr_info, _size: usize, data: *mut c_void) -> c_int {
	let info = unsafe { *info };
	let module_name = unsafe { CStr::from_ptr(info.dlpi_name) }.to_str().unwrap();
	let cb_data = unsafe { &mut *(data as *mut CallbackData) };
	let target_module_name = unsafe { CStr::from_ptr(cb_data.module_name_ptr as *mut c_char) }
		.to_str()
		.unwrap();
	if !module_name.ends_with(target_module_name) {
		return 0;
	}

	let headers: &'static [Elf32_Phdr] =
		unsafe { std::slice::from_raw_parts(info.dlpi_phdr, info.dlpi_phnum as usize) };

	// Check all PT_LOAD segments instead of just the first one
	let mut code_segment = None;
	for header in headers.iter().filter(|p| p.p_type == PT_LOAD) {
		let start = (info.dlpi_addr + header.p_vaddr) as usize;
		let len = header.p_memsz as usize;

		// If this segment has executable permissions, it likely contains code
		if header.p_flags & libc::PF_X != 0 {
			code_segment = Some((start, len));
			break;
		}

		// If we haven't found an executable segment yet, store this one
		if code_segment.is_none() {
			code_segment = Some((start, len));
		}
	}

	if let Some((start, len)) = code_segment {
		cb_data.memory_start = start;
		cb_data.memory_len = len;
		cb_data.memory_area = Some(unsafe { std::slice::from_raw_parts(start as *const u8, len) });
		return 1; // Signal that we've found the module
	}

	0 // Module found but no suitable segments
}

impl Scanner {
	pub fn for_module(name: &str) -> Result<Self, Infallible> {
		Ok(Self {
			module_name: name.to_string(),
		})
	}

	pub fn find(&self, signature: &[Option<u8>]) -> Option<*mut u8> {
		let module_name = CString::new(self.module_name.clone()).unwrap();
		let module_name_ptr = module_name.as_ptr();
		let mut data = CallbackData {
			module_name_ptr,
			memory_start: 0,
			memory_len: 0,
			memory_area: None,
		};
		unsafe {
			dl_iterate_phdr(
				Some(dl_phdr_callback),
				&mut data as *mut CallbackData as *mut c_void,
			)
		};

		// Check if we actually found the module
		if data.memory_start == 0 || data.memory_len == 0 || data.memory_area.is_none() {
			return None;
		}

		let mut data_current = data.memory_start as *mut u8;
		let data_end = (data.memory_start + data.memory_len) as *mut u8;

		// Rest of the function remains the same
		if data_current.is_null() || data_end == data_current {
			// There's no more bytes to scan or the module wasn't found.
			return None;
		}

		let mut signature_offset = 0;
		let mut result: Option<*mut u8> = None;

		unsafe {
			while data_current <= data_end {
				if signature[signature_offset] == None
					|| signature[signature_offset] == Some(*data_current)
				{
					if signature.len() <= signature_offset + 1 {
						if result.is_some() {
							// Found two matches.
							return None;
						}
						result = Some(data_current.offset(-(signature_offset as isize)));
						data_current = data_current.offset(-(signature_offset as isize));
						signature_offset = 0;
					} else {
						signature_offset += 1;
					}
				} else {
					data_current = data_current.offset(-(signature_offset as isize));
					signature_offset = 0;
				}

				data_current = data_current.offset(1);
			}
		}

		result
	}

	pub fn finish(self) -> Result<(), Infallible> {
		Ok(())
	}
}
