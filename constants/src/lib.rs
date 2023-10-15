extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

use chemistru_elements::raw::RawElement;

static DATA: &str = include_str!("../../periodic-table-data/periodic-table.json");

#[proc_macro]
pub fn elements_consts(_: TokenStream) -> TokenStream {
    let elements: Vec<RawElement> = serde_json::from_str(DATA).unwrap();

    let elements_init = elements.iter().map(generate_const_init);

    let tokens = quote! {
        #( #elements_init )*
    };

    TokenStream::from(tokens)
}

fn generate_const_init(element: &RawElement) -> proc_macro2::TokenStream {
    let assignment_name = proc_macro2::Ident::new(&element.name.to_uppercase().replace(' ', "_"), proc_macro2::Span::call_site());
    let name = element.name.as_str();
    let symbol = element.symbol.as_str();
    let proton_number = element.number;
    let mass_number = element.atomic_mass;

    quote! { pub const #assignment_name: chemistru_elements::Element::new(#name, #symbol, #mass_number, #proton_number); }
}