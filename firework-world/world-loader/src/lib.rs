use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use regex::Regex;
use std::fs;
use syn::{
    self,
    parse::{Parse, ParseStream},
    parse_macro_input, LitBool, LitStr, Result, Token,
};

struct WorldArgs {
    path: LitStr,
    flat: LitBool,
}

impl Parse for WorldArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let path: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let flat: LitBool = input.parse()?;
        Ok(WorldArgs { path, flat })
    }
}

#[proc_macro]
pub fn world(input: TokenStream) -> TokenStream {
    let WorldArgs { path, flat } = parse_macro_input!(input as WorldArgs);

    let world_path = path.value();
    let world_is_flat = flat.value;

    let paths = fs::read_dir(world_path).unwrap();

    let regex = Regex::new(r"(?m)r\.(-?\d*)\.(-?\d*)\.mca").unwrap();

    let mut output = TokenStream2::new();

    for path in paths {
        let path = path.unwrap().path();

        let Some(path_str) = path.to_str() else {
            continue;
        };

        let path_str = format!(".{}", path_str);

        let Some(file_name) = path.file_name() else {
            continue;
        };

        let Some(extension) = path.extension() else {
            continue;
        };

        let file_name = file_name.to_str().expect("File name must be a string");
        let extension = extension.to_str().expect("Extension must be a string");

        if extension != "mca" {
            continue;
        }

        let captures = regex
            .captures(file_name)
            .expect("File name must match regex");

        let x: i32 = captures
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .expect("X must be an i32");
        let z: i32 = captures
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .expect("Z must be an i32");
        output.extend(quote! {
            world.add_region(#x, #z, include_bytes!(#path_str).as_slice());
        });
    }

    quote! {
        {
            use firework_world::{World, Region};
            let world = World::new(#world_is_flat);
            #output
            world
        }
    }
    .into()
}