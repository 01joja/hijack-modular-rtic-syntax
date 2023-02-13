use proc_macro2::TokenStream as TokenStream2;
use syn::{parse, ItemMod};

use crate::{
    ast::{TaskModule},
    parse::util,
};

impl TaskModule {
    pub(crate) fn parse(module: ItemMod) -> parse::Result<Self> {
        
        let name = module.ident;
                            

        let mut items = vec![];
        if let Some(i) = module.content{
            items = i.1;
        }

        match module.vis{
            syn::Visibility::Public(_) => (), // Ok! its public.
            _ => {
                return Err(parse::Error::new(
                    name.span(),
                    &format!(
                        "module {} is needs to be public. Make sure your pass generates: \n\"pub mod {}{{\n...\n}}\"",
                        name.to_string(),name.to_string()
                    ),
                ));
            },
        }

        
        return Ok(TaskModule {
            items,
        });

    }
}
