extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;
fn ty_is_f32(ty: &syn::Type) -> bool {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() == 1 && p.path.segments[0].ident == "f32" {
            return true
        }
    }
    false
}
fn ty_is_option(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != "Option" {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(ref inner_type) = p.path.segments[0].arguments {
            if inner_type.args.len() != 1 {
                return None;
            }
            let inner_ty = inner_type.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty{
                return Some(t);
            }
        } 
    }
    None
}
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // let _ = input;
    let ast = parse_macro_input!(input as DeriveInput);
    
    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = syn::Ident::new(&bname, name.span());
    let data = ast.data;
    let fields = match data {
        syn::Data::Struct(
            syn::DataStruct {
                fields: 
                    syn::Fields::Named(
                        syn::FieldsNamed {
                            named: _named, 
                            .. 
                        }
                ),
                ..
            }) => _named,
        _ => unimplemented!(),
    };
    
    let optionised = fields.iter().map(|f|{
        let fname = &f.ident;
        let ty = &f.ty;
        if ty_is_option(&ty).is_some(){
            quote! {
                #fname: #ty
            }
        } else {
            quote! {
                #fname: std::option::Option<#ty>
            }
        }
    });
  
    let builder_methods = fields.iter().map(|f|{
        let fname = &f.ident;
        let fty = &f.ty;
        if let Some(inner_ty) = ty_is_option(&fty){
            quote! {
                pub fn #fname(&mut self, #fname: #inner_ty) -> &mut Self {
                    self.#fname = Some(#fname); 
                    self
                }
            }
        } else {
            quote! {
                pub fn #fname(&mut self, #fname: #fty) -> &mut Self {
                    self.#fname = Some(#fname); 
                    self
                }
            }
        }
    });
 
    let build_fields = fields.iter().map(|f|{
        let fname = &f.ident;
        if ty_is_option(&f.ty).is_some(){
            quote! {
                #fname: self.#fname.clone()
            }
        } else {
            quote!{
                #fname: self.#fname.clone().ok_or(concat!(stringify!(#fname), " is not set"))?
            }
        }
    });
    let empty_build_fields = fields.iter().map(|f|{
        let fname = &f.ident;
        
        if ty_is_f32(&f.ty) {
            quote!{
                #fname: Some(0.0)
            }
        } else {
            quote!{
                #fname: Some(0)
            }
        }
        
        
    });
    let expanded = quote!{
        pub struct #bident {
            #(#optionised,)*
        }

        impl #bident {
            #(#builder_methods)*

            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#build_fields,)*
                })
            }
        }

        impl #name {
            pub fn builder() -> #bident {
                #bident {
                    #(#empty_build_fields,)*
                }
            }
        }
    };
    expanded.into()
}