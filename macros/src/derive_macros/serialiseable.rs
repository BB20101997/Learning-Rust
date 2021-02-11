use quote::{quote};
use syn::{DeriveInput,Data::*,Ident,DataStruct,DataEnum};
use syn::Type;

///
/// The implementation of the procedural macro for deriving the Serializable trait
///
/// Currently only Supports named structs
///

pub fn impl_serializable_macro(ast:&DeriveInput) -> proc_macro::TokenStream{

    let name = &ast.ident;

    let mut fields_list:Vec<&Ident> = Vec::new();
    let mut types_list:Vec<&Type> = Vec::new();

    match &ast.data {
        Struct(DataStruct{fields,..})=>{
            for field in fields {
                if let Some(ident) = &field.ident{
                    fields_list.push(&ident);
                    types_list.push(&field.ty);
                }
            }
        }
        Enum(DataEnum{variants:_variants,..})=>{
            panic!("For now not available for Enums")
        }
        Union(_) => panic!("Derive not available for Unions")
    }

    let gen = quote!{

        struct _AssertSerializebility where #(#types_list:Serializable),* {}

        impl Serializable for #name {

            fn serialize(&mut self, direction: &mut ReadWrite) -> std::io::Result<()>{
                #(self.#fields_list.serialize(direction)?;)*
                Ok(())
            }
        }

    };
    gen.into()
}
