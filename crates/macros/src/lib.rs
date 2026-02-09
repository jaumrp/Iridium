extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, ItemFn, ItemStruct, LitInt, parse::Parser, parse_macro_input,
};

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

            let rt = iridium::server::tokio::runtime::Builder::new_multi_thread().enable_all().build().expect("failed to create runtime");
            rt.block_on(async {
                let server_instance = #inner_name().await;
                iridium::server::iridium_server::bootstrap(server_instance).await;
            })
        }
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #inner_fn
    };

    TokenStream::from(out)
}
//iridium::server::log::info!("╭──────────────────────────────────────────╮");
//iridium::server::log::info!("│ 💎 Iridium Server                        │");
//iridium::server::log::info!("│ 🆔 ID:      {:<28} │", config.id);
//iridium::server::log::info!("│ 🌍 Address: {:<28} │", config.address);
//iridium::server::log::info!("│ 🔌 Port:    {:<28} │", config.port);
//iridium::server::log::info!("╰──────────────────────────────────────────╯");

#[proc_macro_derive(Packet, attributes(packet))]
pub fn packet_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut id: Option<LitInt> = None;

    for attr in &input.attrs {
        if attr.path().is_ident("packet") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("id") {
                    let value: LitInt = meta.value()?.parse()?;
                    id = Some(value);
                    Ok(())
                } else {
                    Err(meta.error("expected `id` attribute"))
                }
            });
        }
    }

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

    let write_id = if let Some(id) = id {
        quote! {
            crate::serial::PacketWrite::write(&crate::types::var_int::VarInt(#id), buffer)?;
        }
    } else {
        quote! {
            crate::serial::PacketWrite::write(&crate::types::var_int::VarInt(0x00), buffer)?;
        }
    };

    let expanded = quote! {
        impl crate::serial::PacketRead for #name {
            fn read<Buffer: bytes::Buf>(buffer: &mut Buffer) -> Result<Self, crate::serial::PacketError> {
                Ok(Self {
                    #(#read)*
                })
            }
        }

        impl crate::serial::PacketWrite for #name {
            fn write(&self, buffer: &mut bytes::BytesMut) -> Result<(), crate::serial::PacketError> {
                #write_id
                #(#write)*
                Ok(())
            }
        }

    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn event(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &item_struct.ident;

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    pub is_canceled: bool
                })
                .unwrap(),
        );
    }

    let impl_event = quote! {

      impl events::Event for #struct_name {
          fn as_any(&self) -> &dyn std::any::Any {
              self
          }

          fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
              self
          }

          fn name(&self) -> &'static str {
              stringify!(#struct_name)
          }
      }
    };

    let impl_cancelable = quote! {

        impl events::Cancelable for #struct_name {
            fn is_canceled(&self) -> bool {
                self.is_canceled
            }

            fn set_canceled(&mut self, cancelled: bool) {
                self.is_canceled = cancelled;
            }
        }

    };

    let out = quote! {
        #item_struct
        #impl_event
        #impl_cancelable
    };

    out.into()
}
