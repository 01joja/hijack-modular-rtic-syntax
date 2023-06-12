use syn::{parse, Stmt, Item, spanned::Spanned,};

use crate::{
    ast::{MainFunction},
    parse::util,
};



impl MainFunction{
    pub(crate) fn parse(stmts: Vec<Stmt>) -> parse::Result<Self>{

        let mut main_stmts = vec![];
        let mut resource_init = vec![];

        for stmt in stmts{
            

            if let Stmt::Item(item) = stmt.clone() {
                if let Item::Fn(item_func) = item.clone() {
                    
                    if let Some(_) = item_func
                        .attrs
                        .iter()
                        .position(|attr| util::attr_eq(attr, "__post_init")){
                            
                            if resource_init.len() > 0{
                                return Err(parse::Error::new(
                                    item.span(),
                                    "__resouce_init is defined multiple times in __rtic_main, passes error",
                                ));
                            }
                            resource_init = item_func.block.stmts;
                        }else{
                            main_stmts.push(stmt);
                        }
                } else {
                    main_stmts.push(stmt);
                }
            } else {
                main_stmts.push(stmt);
            }

        }
        
        return Ok(
            MainFunction{
                post_init: resource_init,
                pre_init: main_stmts,
            }
        );
    }
}