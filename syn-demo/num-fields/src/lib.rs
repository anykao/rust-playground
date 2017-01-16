extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;

#[proc_macro_derive(NumFields)]
pub fn num_fields(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    // Parse the string representation into a syntax tree
    let ast = syn::parse_macro_input(&source).unwrap();

    // Build the output
    let expanded = expand_num_fields(&ast);

    // Return the generated impl as a TokenStream
    expanded.parse().unwrap()
}

fn expand_num_fields(ast: &syn::MacroInput) -> quote::Tokens {
    let n = match ast.body {
        syn::Body::Struct(ref data) => data.fields().len(),
        syn::Body::Enum(_) => panic!("#[derive(NumFields)] can only be used with structs"),
    };

    // Used in the quasi-quotation below as `#name`
    let name = &ast.ident;

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        // The generated impl
        impl #impl_generics NumFields for #name #ty_generics #where_clause {
            fn num_fields() -> usize {
                #n
            }
        }
    }
}
