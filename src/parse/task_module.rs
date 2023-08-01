
use syn::{parse, ItemMod};

use crate::{
    ast::{taskModule}
};

impl taskModule {
    pub(crate) fn parse(module: &ItemMod, has_context: bool, has_monotonic: bool) -> parse::Result<Self> {
        

        let name = module.ident.clone();             

        let mut items = vec![];
        if let Some(i) = module.content.clone(){
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
        
        return Ok(taskModule {
            has_context,
            has_monotonic,
            items,
        });

    }
}
