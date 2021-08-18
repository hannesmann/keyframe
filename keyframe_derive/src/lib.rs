extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

// https://github.com/dtolnay/syn/issues/516
#[proc_macro_derive(CanTween)]
pub fn derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let struct_name = &input.ident;
	let generics = &input.generics;
	let where_clause = &input.generics.where_clause;

	match &input.data {
		Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => {
			let field_name = fields.named.iter().map(|field| &field.ident);

			TokenStream::from(quote! {
				impl #generics keyframe::CanTween for #struct_name #generics #where_clause {
					fn ease(mut from: Self, mut to: Self, time: impl keyframe::num_traits::Float) -> Self {
						Self {
							#(
								#field_name: keyframe::CanTween::ease(from.#field_name, to.#field_name, time),
							)*
						}
					}
				}
			})
		}
		Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
			let field_idx = fields.unnamed.iter().enumerate().map(|(i, _)| syn::Index::from(i));

			TokenStream::from(quote! {
				impl #generics keyframe::CanTween for #struct_name #generics #where_clause {
					fn ease(mut from: Self, mut to: Self, time: impl keyframe::num_traits::Float) -> Self {
						Self(
							#(
								keyframe::CanTween::ease(from.#field_idx, to.#field_idx, time),
							)*
						)
					}
				}
			})
		},
		_ => panic!("Expected struct with fields!")
	}
}