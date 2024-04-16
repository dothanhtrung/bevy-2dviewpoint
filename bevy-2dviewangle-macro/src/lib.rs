// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, Expr, ExprLit, Fields, Lit, Meta, Token};
use syn::punctuated::Punctuated;

const TEXTUREVIEW_ATTRIBUTE: &str = "textureview";
#[proc_macro_derive(ActorsTexturesCollection, attributes(textureview))]
pub fn actors_textures_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_actors_textures(ast).unwrap_or_default().into()
}

fn impl_actors_textures(
    ast: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, Vec<syn::Error>> {
    let struct_name = &ast.ident;

    if let Data::Struct(data_struct) = &ast.data {
        if let Fields::Named(fields) = &data_struct.fields {
            let field_info = fields.named.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let mut actor_value = quote! {None};
                let mut action_value = quote! {None};
                let mut angle_value = quote! {None};
                let mut image_value = quote! {None};
                let mut atlas_layout_value = quote! {None};
                let mut type_value = String::new();

                for attr in field
                    .attrs
                    .iter()
                    .filter(|attribute| attribute.path().is_ident(TEXTUREVIEW_ATTRIBUTE))
                {
                    let view_meta_list =
                        attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated);

                    for attribute in view_meta_list.unwrap() {
                        match attribute {
                            Meta::NameValue(named_value) if named_value.path.is_ident("actor") => {
                                if let Expr::Lit(ExprLit {
                                    lit: Lit::Int(key), ..
                                }) = &named_value.value
                                {
                                    let value = key.base10_parse::<u64>().unwrap();
                                    actor_value = quote! {Some(#value)};
                                }
                            }
                            Meta::NameValue(named_value) if named_value.path.is_ident("action") => {
                                if let Expr::Lit(ExprLit {
                                    lit: Lit::Int(key), ..
                                }) = &named_value.value
                                {
                                    let value = key.base10_parse::<u16>().unwrap();
                                    action_value = quote! {Some(#value)};
                                }
                            }
                            Meta::NameValue(named_value) if named_value.path.is_ident("angle") => {
                                if let Expr::Lit(ExprLit {
                                    lit: Lit::Str(key), ..
                                }) = &named_value.value
                                {
                                    let value = key.value();
                                    angle_value = quote! {Some(#value.to_string())};
                                }
                            }
                            Meta::NameValue(named_value) if named_value.path.is_ident("handle") => {
                                if let Expr::Lit(ExprLit {
                                    lit: Lit::Str(key), ..
                                }) = &named_value.value
                                {
                                    type_value = key.value();
                                }
                            }
                            _ => {}
                        }
                    }
                }

                match type_value.as_str() {
                    "image" => image_value = quote! {Some(&self.#field_name)},
                    "atlas_layout" => atlas_layout_value = quote! {Some(&self.#field_name)},
                    _ => {}
                }

                quote! {
                    FieldInfo {
                        actor: #actor_value,
                        action: #action_value,
                        angle: #angle_value,
                        image: #image_value,
                        atlas_layout: #atlas_layout_value,
                    }
                }
            });
            let expanded = quote! {
                #[automatically_derived]
                impl ActorsTexturesCollection for #struct_name {
                    fn get_all(&self) -> Vec<FieldInfo> {
                        vec![#( #field_info ),*]
                    }
                }
            };
            return Ok(expanded);
        }
    }

    return Err(vec![]);
}