use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answerx(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
