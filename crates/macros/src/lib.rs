extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, ItemFn, parse_macro_input};

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

    //let output_type = &input.sig.output;
    let vis = &input.vis;
    let attrs = &input.attrs;

    let out = quote! {
        #(#attrs)*
        #vis fn #original_name() {

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
                let server_instance = #inner_name().await;
                iridium::server::iridium_server::bootstrap(server_instance, config).await;
            })
        }
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #inner_fn
    };

    TokenStream::from(out)
}

#[proc_macro_derive(Packet)]
pub fn packet_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("packets derive only named fields are supported"),
        },
        _ => panic!("packets derive only structs are supported"),
    };

    let read = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {
            #name: <#ty as crate::serial::PacketRead>::read(buffer)?,
        }
    });

    let write = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            crate::serial::PacketWrite::write(&self.#name, buffer)?;
        }
    });

    let expanded = quote! {
        impl crate::serial::PacketRead for #name {
            fn read<Buffer: bytes::Buf>(buffer: &mut Buffer) -> Result<Self, crate::serial::PacketError> {
                Ok(Self {
                    #(#read)*
                })
            }
        }

        impl crate::serial::PacketWrite for #name {
            fn write<Buffer: bytes::BufMut>(&self, buffer: &mut Buffer) -> Result<(), crate::serial::PacketError> {
                #(#write)*
                Ok(())
            }
        }

    };

    TokenStream::from(expanded)
}
