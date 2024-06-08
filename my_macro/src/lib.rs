use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_attribute]
pub fn test_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    // `args` 用于接收属性参数的输入流，
    println!("attr: {:#?}", attr);
    // `item` 用于接收被处理的输入流。
    println!("item: {:#?}", item);

    // 暂时不做任何处理，直接返回原始输入。
    item
}

#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut result = item.to_string();
    result.push_str(" // This is my custom attribute!");
    result.parse().unwrap()
}

#[proc_macro_attribute]
pub fn my_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_name = attr.to_string();
    let mut result = item.to_string();
    result.push_str(&format!("struct {} {{", struct_name));
    result.push_str("data: i32 }");
    result.parse().unwrap()
}

#[proc_macro_derive(MyDebug)]
pub fn custom(input: TokenStream) -> TokenStream{
    // 派生宏的处理逻辑
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_token_stream = ast.ident.to_token_stream();
    // 名称字符串
    let struct_name = struct_token_stream.to_string();
    println!("struct_name {}", struct_name);
    let expand = quote::quote! {
        // 在代码中需要使用 TokenStream
        impl #struct_token_stream {
                fn my_debug(&self) {{
                    println!("{} 自定义的派生宏!", #struct_name);
                }}
        }
    };
    expand.into()
}