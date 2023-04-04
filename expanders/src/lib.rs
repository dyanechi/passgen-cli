extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro]
pub fn enum_with_props(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let data = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("The macro can only be applied to enums"),
    };

    let variants = &data.variants;
    let variant_idents: Vec<&Ident> = variants.iter().map(|variant| &variant.ident).collect();
    let variant_fields: Vec<&Fields> = variants.iter().map(|variant| &variant.fields).collect();

    let props_name = Ident::new(&format!("{}Props", name), name.span());
    let fields = match &variant_fields[0] {
        Fields::Unnamed(fields_unnamed) => &fields_unnamed.unnamed,
        _ => panic!("The enum variants must have unnamed fields"),
    };

    let field_types: Vec<_> = fields.iter().map(|field| &field.ty).collect();
    let field_idents: Vec<_> = (0..fields.len()).map(|i| Ident::new(&format!("field_{}", i), name.span())).collect();

    let expanded = quote! {
        pub struct #props_name {
            #(pub #field_idents: #field_types),*
        }

        pub enum #name {
            #(#variant_idents),*
        }

        impl #name {
            // pub fn props(&self) -> #props_name {
            //     match self {
            //         #(
            //             #name::#variant_idents => #props_name {
            //                 #(#field_idents: #variant_fields),*
            //             },
            //         )*
            //     }
            // }
        }
    };

    TokenStream::from(expanded)
}
