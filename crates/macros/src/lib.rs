extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    if input.sig.asyncness.is_none() {
        return syn::Error::new_spanned(input.sig.fn_token, "function is not async")
            .to_compile_error()
            .into();
    }
    let original_name = &input.sig.ident;
    let inner_name = syn::Ident::new(
        &format!("__iridium_inner_{}", original_name),
        original_name.span(),
    );

    let inner_fn = {
        let mut f = input.clone();
        f.sig.ident = inner_name.clone();
        f
    };

    let output_type = &input.sig.output;
    let vis = &input.vis;
    let attrs = &input.attrs;

    let out = quote! {
        #(#attrs)*
        #vis fn #original_name() #output_type {

            iridium::server::init_logging();

            let config = iridium::server::Config::load();

            iridium::server::log::info!("╭──────────────────────────────────────────╮");
            iridium::server::log::info!("│ 💎 Iridium Server                        │");
            iridium::server::log::info!("│ 🆔 ID:      {:<28} │", config.id);
            iridium::server::log::info!("│ 🌍 Address: {:<28} │", config.address);
            iridium::server::log::info!("│ 🔌 Port:    {:<28} │", config.port);
            iridium::server::log::info!("╰──────────────────────────────────────────╯");

            let rt = iridium::server::tokio::runtime::Builder::new_multi_thread().enable_all().build().expect("failed to create runtime");
            rt.block_on(async {
                #inner_name().await
            })
        }
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #inner_fn
    };

    TokenStream::from(out)
}
