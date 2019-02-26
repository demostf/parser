extern crate proc_macro;

use proc_macro2::{TokenStream, Span};
use quote::{quote, quote_spanned};
use quote::{TokenStreamExt, ToTokens};
use syn::{Data, DeriveInput, Field, Fields, GenericParam, Generics, Ident, Index, Lit, Meta, parse_macro_input, parse_quote, Path, Token, Type, TypePath};
use syn::spanned::Spanned;

#[proc_macro_derive(Parse, attributes(size))]
pub fn derive_helper_attr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let parse = parse(&input.data, &name);

    let expanded = quote! {
        impl #impl_generics ::tf_demo_parser::Parse<'_> for #name #ty_generics #where_clause {
            fn parse(stream: &mut ::tf_demo_parser::Stream, _state: &::tf_demo_parser::ParserState) -> ::tf_demo_parser::Result<Self> {
                Ok(#parse)
            }
            fn skip(stream: &mut ::tf_demo_parser::Stream) -> ::tf_demo_parser::Result<()> {
                // TODO
                Ok(())
            }
        }
    };

    //panic!("{}", TokenStream::to_string(&expanded));

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(::heapsize::HeapSize));
        }
    }
    generics
}

fn parse(data: &Data, struct_name: &Ident) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        // Get attributes `#[..]` on each field
                        let size = get_field_size(f);
                        let parser = parse_type(&f.ty, size);
                        quote_spanned! { f.span()=>
                            #name: #parser?,
                        }
                    });
                    quote! {
                        #struct_name {
                            #(#recurse)*
                        }
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

fn get_field_size(field: &Field) -> Option<usize> {
    for attr in field.attrs.iter() {
        // Parse the attribute
        let meta = attr.parse_meta().unwrap();
        match meta {
            // Matches attribute with single word like `#[nng_member]` (as opposed to `#[derive(NngGetOps)]` or `#[nng_member = "socket"]`)
            Meta::NameValue(ref nameValue) if nameValue.ident == "size" =>
                match &nameValue.lit {
                    Lit::Int(size) => {
                        return Some(size.value() as usize);
                    }
                    _ => panic!("Invalid size for field")
                }
            _ => (),
        }
    }
    None
}

fn parse_type(ty: &Type, size: Option<usize>) -> TokenStream {
    match ty {
        Type::Path(path) => {
            let types: Vec<(TypePath, Option<usize>, &str)> = vec![
                (parse_quote!(String), None, "read_string"),
                (parse_quote!(bool), None, "read"),
                (parse_quote!(u8), Some(8), "read"),
                (parse_quote!(u16), Some(16), "read"),
                (parse_quote!(u32), Some(32), "read"),
                (parse_quote!(i8), Some(8), "read"),
                (parse_quote!(i16), Some(16), "read"),
                (parse_quote!(i32), Some(32), "read"),
                (parse_quote!(f32), None, "read_float"),
                (parse_quote!(f64), None, "read_float"),
            ];
            for (type_option, default_size, method) in types {
                let method_ident = Ident::new(method,Span::call_site());
                if type_option == *path {
                    match default_size {
                        Some(default) => {
                            let size = size.unwrap_or(default);
                            return quote! {
                                stream.#method_ident(#size)
                            }
                        }
                        None => {
                            if method == "read_string" {
                                let size = QuoteOption(size);
                                return quote! {
                                    stream.#method_ident(#size)
                                }
                            } else {
                                return quote! {
                                    stream.#method_ident()
                                }
                            }
                        }
                    }
                }
            }
            unimplemented!("a");
        }
        _ => unimplemented!("b"),
    }
}

struct QuoteOption<T>(Option<T>);

impl<T: ToTokens> ToTokens for QuoteOption<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self.0 {
            Some(ref t) => quote! { ::std::option::Option::Some(#t) },
            None => quote! { ::std::option::Option::None },
        });
    }
}