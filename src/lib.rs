use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, Parser};

#[proc_macro_attribute]
pub fn multiple_runner(attr: TokenStream, item: TokenStream) -> TokenStream {
    let run_count: usize = if attr.is_empty() {
        100
    } else {
        attr.to_string().parse().expect("Can't parse run count.")
    };

    let output = match syn::Item::parse.parse(item) {
        Ok(syn::Item::Fn(mut item_fn)) => {
            let name = item_fn.sig.ident.clone();
            let name_str = name.to_string();
            let attrs = item_fn.attrs;

            item_fn.attrs = vec![];

            match item_fn.sig.asyncness {
                Some(_) => {
                    quote! {
                        #(#attrs)*
                        async fn #name() {
                            use tokio::sync::mpsc::channel;
                            use tokio::runtime::Runtime;
                            use rayon::iter::ParallelIterator;

                            #item_fn

                            let thread_pool = rayon::ThreadPoolBuilder::new()
                                .thread_name(|_| #name_str.to_string())
                                .build()
                                .unwrap();

                            thread_pool.install(|| {
                                let out = rayon::prelude::IntoParallelIterator::into_par_iter(0..#run_count)
                                    .try_for_each(|i| {
                                        std::panic::catch_unwind(|| {
                                            let rt = Runtime::new().unwrap();
                                            rt.block_on(#name()).unwrap();
                                        })
                                    });

                                if let Err(err) = out {
                                    std::panic::resume_unwind(err);
                                }
                            })
                        }
                    }
                }
                None => {
                    quote! {
                        #(#attrs)*
                        fn #name() {
                            use tokio::sync::mpsc::channel;
                            use tokio::runtime::Runtime;
                            use rayon::iter::ParallelIterator;

                            #item_fn

                            let thread_pool = rayon::ThreadPoolBuilder::new()
                                .thread_name(|_| #name_str.to_string())
                                .build()
                                .unwrap();

                            thread_pool.install(|| {
                                let out = rayon::prelude::IntoParallelIterator::into_par_iter(0..#run_count)
                                    .try_for_each(|i| std::panic::catch_unwind(|| #name().unwrap()));

                                if let Err(err) = out {
                                    std::panic::resume_unwind(err);
                                }
                            })
                        }
                    }
                }
            }
        }
        _ => panic!(),
    };

    output.into()
}
