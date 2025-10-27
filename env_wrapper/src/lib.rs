extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro]
pub fn wrap_env(input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = input_fn.sig.ident.clone();
    let inner_name = syn::Ident::new(&format!("{}_inner", fn_name), fn_name.span());
    let output_type = input_fn.sig.output.clone();
    let inputs = input_fn.sig.inputs.clone();
    let block = input_fn.block;

    let arg_count = inputs.len();
    let arg_pop = match arg_count {
        0 => quote! {},
        1 => quote! {
            "LD.W %d4, [%a10], 0",
        },
        2 => quote! {
            "LD.W %d4, [%a10], 4",
            "LD.W %d5, [%a10], 0",
        },
        _ => todo!()
    };

    let return_instr = match output_type {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, _) => quote! { "mov %d0, %d2",},
    };

    let expanded = quote! {
        #[naked]
        pub unsafe extern "C" fn #fn_name() {
            core::arch::naked_asm!(
                #arg_pop
                concat!("call ", stringify!(#inner_name)),
                #return_instr
                "ret"
            );
        }

        #[inline(never)]
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #inner_name(#inputs) #output_type {
            #block
        }
    };

    TokenStream::from(expanded)
}
