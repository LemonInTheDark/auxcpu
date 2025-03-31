use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Lit, parse_macro_input};

fn from_signature(signature: String) -> Vec<Option<u8>> {
	signature
		.trim()
		.split(' ')
		.map(|byte| {
			let byte = byte.trim();
			assert_eq!(
				byte.len(),
				2,
				"{byte} is not a valid byte: they must be either two hex characters, or ??"
			);
			if byte == "??" {
				None
			} else {
				match u8::from_str_radix(byte, 16) {
					Ok(hex) => Some(hex),
					Err(error) => panic!("Failed to convert {byte} from hex to u8: {error}"),
				}
			}
		})
		.collect::<Vec<Option<u8>>>()
}

#[proc_macro]
pub fn convert_signature(input: TokenStream) -> TokenStream {
	let string = match parse_macro_input!(input as Lit) {
		Lit::Str(lit) => lit.value(),
		_ => panic!("not string input"),
	};

	let streams = from_signature(string)
		.into_iter()
		.map(|x| match x {
			Some(byte) => {
				quote! {
					Some(#byte as u8)
				}
			}
			None => {
				quote! { None }
			}
		})
		.collect::<Vec<TokenStream2>>();

	let result = quote! {
		&[ #( #streams, )* ]
	};
	result.into()
}
