
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated};

trait Forwarder {}

#[proc_macro_derive(Forwarder, attributes(forward))]
pub fn forwarder(input_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_stream as syn::ItemEnum);
    let ident = input.ident.clone();
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl().to_owned();

    let mut to_derive_idents = Vec::new();
    let mut to_derive_variants = Vec::new();
    let mut to_derive_args: Vec<Vec<syn::Path>> = Vec::new();

    for variant in input.variants.into_iter() {
        for attr in variant.attrs {
            if attr.path().is_ident("forward") {
                match attr.parse_args_with(
                    Punctuated::<syn::Path, syn::Token!(,)>::parse_terminated
                ) {
                    Ok(args) => {
                        to_derive_args.push(args.into_iter().collect());
                    },
                    Err(_) => {
                        to_derive_args.push(Vec::new());
                    }
                };

                to_derive_idents.push(match variant.fields {
                    syn::Fields::Unnamed(ref fields) => {
                        fields.unnamed.clone().into_iter().next().expect("expected a single field")
                    },
                    _ => panic!("expected unnamed tuple-like fields")
                });
                to_derive_variants.push(variant.ident.clone());
            }
        }
    }

    let gen = quote! {
        #(
            impl #impl_generics From<#to_derive_idents> for #ident #type_generics #where_clause {
                fn from(value: #to_derive_idents) -> #ident #type_generics {
                    Self::#to_derive_variants(value)
                }
            }

            #(
                impl #impl_generics From<#to_derive_args> for #ident #type_generics #where_clause {
                    fn from(value: #to_derive_args) -> #ident #type_generics {
                        Self::#to_derive_variants(value.into())
                    }
                }
            )*
        )*
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn deref(args: TokenStream, input_stream: TokenStream) -> TokenStream {
    let input: syn::ItemStruct = parse_macro_input!(input_stream);
    let field_name: syn::Ident = parse_macro_input!(args);

    let ident = input.ident.clone();
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let mut fields_iter = input.fields.iter();
    let field_type = loop {
        if let Some(field) = fields_iter.next() {
            if field.ident.clone().expect("expected struct to be {}") == field_name {
                break field.ty.clone();
            }
        } else {
            panic!("field not found");
        }
    };

    let gen = quote! {
        #input
        impl #impl_generics std::ops::Deref for #ident #type_generics #where_clause {
            type Target = #field_type;
            fn deref(&self) -> &Self::Target {
                &self.#field_name
            }
        }
    };
    gen.into()
}