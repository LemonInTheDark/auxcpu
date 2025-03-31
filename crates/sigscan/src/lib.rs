cfg_if::cfg_if! {
	if #[cfg(windows)] {
		mod windows;
		pub use windows::Scanner;
	} else {
		mod linux;
		pub use linux::Scanner;
	}
}
