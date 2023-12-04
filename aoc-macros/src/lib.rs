use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Ident, LitInt};

#[proc_macro]
pub fn generate_days(input: TokenStream) -> TokenStream {
  let args = parse_macro_input!(input as LitInt);
  let counter = (1..=(args.base10_parse::<u64>().unwrap())).map(|id| Ident::new(&format!("day{:0>2}", id), Span::call_site().into()));

  let ast = quote! {
      vec![
          #((
              crate::days::#counter::part1 as fn(&str) -> i64,
              crate::days::#counter::part2 as fn(&str) -> i64
          )),*
      ]
  };

  ast.into()
}
