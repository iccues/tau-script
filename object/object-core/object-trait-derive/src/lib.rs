use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput};

#[proc_macro_derive(ObjectTrait)]
pub fn object_trait_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    let name = &ast.ident;
    let gen_tk = quote! {
        impl ObjectTrait for #name {}
    };
    gen_tk.into()
}
