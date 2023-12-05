use std::{env, fs, path::PathBuf};

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::Ident;

#[proc_macro]
pub fn generate_days(_input: TokenStream) -> TokenStream {
  let path = "src/days";

  let dir = match env::var_os("CARGO_MANIFEST_DIR") {
    Some(manifest_dir) => PathBuf::from(manifest_dir).join(path),
    None => PathBuf::from(path),
  };

  let days = fs::read_dir(dir).unwrap().count();
  let counter = (1..days).map(|id| Ident::new(&format!("day{:0>2}", id), Span::call_site().into()));

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
