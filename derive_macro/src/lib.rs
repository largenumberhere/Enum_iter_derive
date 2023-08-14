#![feature(type_name_of_val)]

use proc_macro::TokenStream;
use proc_macro_error::{abort, emit_warning, ResultExt};
use quote::{ToTokens, TokenStreamExt};
use std::any::Any;
use std::fmt::{format, Debug};
use std::hash::{Hash, Hasher};
use std::ops::Range;
use syn::__private::str;
use syn::spanned::Spanned;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, Item, Type};

#[proc_macro_derive(PrintEnumVariants)] //Same as trait
pub fn print_enum_variants_derive(item: TokenStream) -> TokenStream {
    // Convert to an abstract syntax tree for easier manipulation
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate the output
    let stream2 = impl_print_enum_variants_derive(ast);

    // Debug
    //println!("{stream2}");

    // Convert output from proc_macro2::TokensStream to proc_macro::TokenStream
    let stream1 = stream2.into();

    stream1
}

fn impl_print_enum_variants_derive(ast: DeriveInput) -> proc_macro2::TokenStream {
    let identifier = ast.ident;

    let enum_object = match ast.data {
        syn::Data::Enum(e) => e,
        other_type @ _ => panic!(
            "Type '{:?}' is not supported by this macro",
            other_type.type_id()
        ),
    };

    let enum_variant_identifiers: Vec<Ident> = enum_object
        .variants
        .iter()
        .map(|v| v.ident.clone())
        .collect();

    let enum_variants_names: Vec<String> = enum_variant_identifiers
        .into_iter()
        .map(|i| i.to_string())
        .collect();

    quote::quote! {
        impl PrintEnumVariants for #identifier {
            fn print_enum_variants(&self) {
                let mut to_print = String::new();
                #(
                    std::fmt::write(&mut to_print,  std::format_args!{"{}\n", #enum_variants_names} ).unwrap();
                )*

                println!("{}", to_print);
            }
        }
    }
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(FlatStructValues)]
pub fn flat_struct_values_derive(item: TokenStream) -> TokenStream {
    // Convert to an abstract syntax tree for easier manipulation
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate the output
    let stream2 = match impl_flat_struct_values_derive(ast) {
        Ok(v) => v,
        Err(e) => {
            abort!("{}", e.message)
        }
    };

    // Debug
    //println!("{stream2}");

    // Convert output from proc_macro2::TokensStream to proc_macro::TokenStream
    let stream1 = stream2.into();

    stream1
}

struct MacroError {
    message: String,
}

fn impl_flat_struct_values_derive(
    ast: DeriveInput,
) -> Result<proc_macro2::TokenStream, MacroError> {
    // Get struct
    let struct_data: DataStruct = match ast.data {
        syn::Data::Struct(s) => s,
        other @ _ => panic!("Only structes are supported for this macro"),
    };

    // Get struct name
    let identifier = ast.ident;

    // Get field identifiers
    let fields: Vec<Option<Ident>> = struct_data.fields.iter().map(|f| f.ident.clone()).collect();

    // Get struct types
    let field_types: Vec<syn::Type> = struct_data.fields.iter().map(|f| f.ty.clone()).collect();

    let t0 = match field_types.first() {
        // There are no fields so the impl should return an empty vec
        None => {
            emit_warning!("{}", format!("'{identifier}' does not contain any fields. Implementing the trait FlatStructValues currently serves no purpose"));

            return Ok(quote::quote! {
                impl FlatStructValues<()> for #identifier {
                    fn flat_struct_values(&self) -> Vec<()> {
                        vec![()]
                    }
                }
            });
        }
        Some(v) => v,
    };

    // check all type are the same
    for (i, t) in field_types.iter().enumerate().skip(1) {
        if t.clone() != t0.clone() {
            return Err(MacroError{
                message: format!("All fields types must be the same on the struct. Field 0 did not match field number {i}.")
            });
        }
    }

    Ok(quote::quote! {
        impl FlatStructValues<#t0> for #identifier {
            fn flat_struct_values(&self) -> Vec<#t0> {
                let mut output = vec![];
                #(
                    output.push(self.#fields);
                )*

                output
            }

        }
    })
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(FlatStructIter)]
pub fn flat_struct_iter_derive(item: TokenStream) -> TokenStream {
    // Convert to an abstract syntax tree for easier manipulation
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate the output
    let stream2 = match impl_flat_struct_iter(ast) {
        Ok(v) => v,
        Err(e) => {
            abort!("{}", e.message)
        }
    };

    //Debug
    //println!("{stream2}");

    // Convert output from proc_macro2::TokensStream to proc_macro::TokenStream
    let stream1 = stream2.into();

    stream1
}

fn impl_flat_struct_iter(ast: DeriveInput) -> Result<proc_macro2::TokenStream, MacroError> {
    // Get struct
    let struct_data: DataStruct = match ast.data.clone() {
        syn::Data::Struct(s) => s,
        _ => abort!("{}", "Only structes are supported for this macro"),
    };

    // Get struct name
    let identifier = ast.ident.clone();

    // Get field identifiers
    let fields: Vec<Option<Ident>> = struct_data.fields.iter().map(|f| f.ident.clone()).collect();

    // Get struct types
    let field_types: Vec<syn::Type> = struct_data.fields.iter().map(|f| f.ty.clone()).collect();

    let t0 = match field_types.first() {
        // There are no fields so the impl should return an empty vec
        None => {
            abort!("{}", "Struct does not contain any fields");
        }
        Some(v) => v,
    };

    // check all type are the same
    for (i, t) in field_types.iter().enumerate().skip(1) {
        if t.clone() != t0.clone() {
            return Err(MacroError{
                message: format!("All field types must be the same on the struct. Field 0 did not match field number {i}.")
            });
        }
    }

    let struct_name = format!("{}FlatStructIter", identifier.to_string());
    let struct_name = Ident::new(&struct_name, ast.span());

    let index = 0usize..;

    Ok(quote::quote! {

        struct #struct_name <'a,> {
            position: usize,
            structref: &'a #identifier,
        }

        impl<'a> Iterator for #struct_name<'a> {
            type Item = &'a #t0;

            fn next(&mut self) -> Option<Self::Item> {
                let value = match self.position{
                    #(
                        #index => { Some(& self.structref.#fields) }
                    ),*
                    _ => {None}
                };

                self.position += 1;

                value
            }
        }

        impl<'a> FlatStructIter<'a, #t0, #struct_name<'a>> for #identifier {
            fn flat_struct_iter(&'a self) -> #struct_name<'a> {
                #struct_name {
                    position: 0,
                    structref: self
                }
            }
        }
    })
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(StructToTuple)]
pub fn struct_to_tuple_derive(item: TokenStream) -> TokenStream {
    // Convert to an abstract syntax tree for easier manipulation
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate the output
    let stream2 = match impl_struct_to_tuple_derive(ast) {
        Ok(v) => v,
        Err(e) => {
            abort!("{}", e.message)
        }
    };

    //Debug
    //println!("{stream2}");

    // Convert output from proc_macro2::TokensStream to proc_macro::TokenStream
    let stream1 = stream2.into();

    stream1
}

fn impl_struct_to_tuple_derive(ast: DeriveInput) -> Result<proc_macro2::TokenStream, MacroError> {
    // Get struct
    let struct_data: DataStruct = match ast.data.clone() {
        syn::Data::Struct(s) => s,
        _ => abort!("{}", "Only structes are supported for this macro"),
    };

    // Get struct name
    let identifier = ast.ident.clone();

    // Get field identifiers
    let fields: Vec<Option<Ident>> = struct_data.fields.iter().map(|f| f.ident.clone()).collect();

    //Make a counter to increment for each T
    let ti = 0..(fields.len());
    let ti = {
        let mut vec = Vec::new();
        for t in ti {
            vec.push(format!("T{}", t));
        }

        vec
    };

    // Get struct types
    let field_types: Vec<syn::Type> = struct_data.fields.iter().map(|f| f.ty.clone()).collect();

    Ok(quote::quote! {
        impl StructToTuple<(#(#field_types),*)> for #identifier {
            fn struct_to_tuple(self) -> (#(#field_types),*) {
                (
                    #( self.#fields ),*
                )
            }
        }
    })
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(TypeNames)]
pub fn type_names_derive(item: TokenStream) -> TokenStream {
    // Convert to an abstract syntax tree for easier manipulation
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Generate the output
    let stream2 = match impl_type_names_derive(ast) {
        Ok(v) => v,
        Err(e) => {
            abort!("{}", e.message)
        }
    };

    //Debug
    //println!("{stream2}");

    // Convert output from proc_macro2::TokensStream to proc_macro::TokenStream
    let stream1 = stream2.into();

    stream1
}

fn impl_type_names_derive(ast: DeriveInput) -> Result<proc_macro2::TokenStream, MacroError> {
    // Get struct
    let struct_data: DataStruct = match ast.data.clone() {
        syn::Data::Struct(s) => s,
        _ => abort!("{}", "Only structes are supported for this macro"),
    };

    // Get struct name
    let identifier = ast.ident.clone();

    // Get struct types
    let field_types: Vec<syn::Type> = struct_data.fields.iter().map(|f| f.ty.clone()).collect();

    Ok(quote::quote! {
        impl TypeNames for #identifier {
            fn type_names(&self) -> Vec<String> {
                vec![
                    #(
                        stringify!(#field_types).to_string()
                    ),*
                ]
            }

            fn type_names_type() -> Vec<String> {
                vec![
                    #(
                        stringify!(#field_types).to_string()
                    ),*
                ]
            }
        }
    })
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(StructIter)]
pub fn struct_iter_derive(item: TokenStream) -> TokenStream {
    // Convert to an abstract syntax tree for easier manipulation
    let ast = syn::parse(item);
    let ast: DeriveInput = match ast {
        Ok(v) => v,
        Err(e) => {
            abort!("Failed to parse token input to macro! {:?}", e)
        }
    };

    // Generate the output or present error
    let stream2 = match impl_struct_to_iter(ast) {
        Ok(v) => v,
        Err(e) => {
            abort!("{}", e.message)
        }
    };

    //Debug
    //println!("{stream2}");

    // Convert output from proc_macro2::TokensStream to proc_macro::TokenStream
    let stream1 = stream2.into();

    stream1
}

struct RawNumber(usize);
impl quote::ToTokens for RawNumber {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(proc_macro2::Literal::usize_unsuffixed(self.0));
    }
}

struct RawNumberIter {
    number: usize,
    max: usize,
}

impl Iterator for RawNumberIter {
    type Item = RawNumber;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.number.clone();
        if current > self.max {
            return None;
        }

        self.number += 1;

        Some(RawNumber(current))
    }
}

impl From<Range<usize>> for RawNumberIter {
    fn from(value: Range<usize>) -> Self {
        RawNumberIter {
            number: value.start,
            max: value.end,
        }
    }
}

fn impl_struct_to_iter(ast: DeriveInput) -> Result<proc_macro2::TokenStream, MacroError> {
    // Get struct
    let struct_data: DataStruct = match ast.data.clone() {
        syn::Data::Struct(s) => s,
        _ => abort!("{}", "Only structes are supported for this macro"),
    };

    // Get struct name
    let identifier = ast.ident.clone();

    // Get struct field types
    let field_types: Vec<syn::Type> = struct_data.fields.iter().map(|f| f.ty.clone()).collect();

    let counter0 = (0..(field_types.len()));
    let counter1: RawNumberIter = (0..(field_types.len())).into();
    let counter2 = (0..(field_types.len()));
    let counter3 = (0..(field_types.len()));
    let counter4 = (0..(field_types.len()));
    let counter5 = (0..(field_types.len()));

    let enum_variant_type_contained = {
        let mut variants = field_types.clone();
        variants.dedup();
        variants
    };
    let enum_variant_type_contained1 = enum_variant_type_contained.clone();
    let enum_variant_type_contained2 = enum_variant_type_contained.clone();

    let enum_variant_names = {
        let mut idents = Vec::new();
        for i in 0..enum_variant_type_contained.len() {
            idents.push(format!("T_{}", i));
        }

        let idents: Vec<Ident> = idents
            .iter()
            .map(|i| {
                let ident: Ident =
                    syn::parse_str(i).expect("Failed to parse enum_vairant_identifier token");
                ident
            })
            .collect();

        idents
    };
    let enum_variant_names1 = enum_variant_names.clone();

    let enum_variant_mappings = {
        let mut idents = Vec::new();
        let mut last_type: Type = field_types.first().unwrap().clone();
        let mut enum_index = 1;

        idents.push(enum_variant_names.first().unwrap().clone());

        for f in field_types.iter().skip(1) {
            if f.clone() != last_type {
                enum_index += 1;
                last_type = f.clone();
            }
            idents.push(enum_variant_names[enum_index - 1].clone());
        }

        idents
    };

    let struct_value_identifier: Ident = {
        let val = format!("StructValue_{}", identifier);
        let val = syn::parse_str(&val).expect("Failed to parse struct_value_identifier token");
        val
    };

    let iter_identifier: Ident = {
        let val = format!("StructIter_{}", identifier);
        let val = syn::parse_str(&val).expect("Failed to parse iter_identifier token");
        val
    };

    Ok(quote::quote! {
        #[derive(Debug)]
        #[derive(Clone)]
        pub enum #struct_value_identifier {
            #( #enum_variant_names ( #enum_variant_type_contained ) ),*
        }

        #(
            impl From<#struct_value_identifier> for Option <#enum_variant_type_contained1> {
                fn from(value: #struct_value_identifier) -> Option <#enum_variant_type_contained2> {
                    match value {
                        #struct_value_identifier :: #enum_variant_names1 (v) => Some(v),
                        _ => None
                    }
                }
            }
        )*

        struct #iter_identifier{
            index: usize,
            inner:
            (
                #(
                    #field_types
                ),*
            )
        }

        impl Iterator for #iter_identifier{
            type Item = #struct_value_identifier;

            fn next(&mut self) -> Option<Self::Item> {
                let val = match self.index {
                    #(
                        #counter0 => #struct_value_identifier::#enum_variant_mappings (self.inner.#counter1)
                    ),*
                    ,
                    _=> return None
                };

                self.index+=1;

                Some(val)
            }
        }

        use enum_iter_derive::StructToTuple;
        impl StructIter<#struct_value_identifier, #iter_identifier,( #( #field_types),* )> for #identifier {
            fn struct_iter(self) -> #iter_identifier {
                let tuple = self.struct_to_tuple();
                #iter_identifier {
                    index: 0usize,
                    inner :tuple
                }
            }
        }

        impl #iter_identifier {
            fn into_inner(self) -> ( #(#field_types),* ) {
                self.inner
            }
        }
    })
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(StructFieldNames)]
pub fn struct_field_names_derive(item: TokenStream) -> TokenStream {
    // Convert to an abstract syntax tree for easier manipulation
    let ast = syn::parse(item);
    let ast: DeriveInput = match ast {
        Ok(v) => v,
        Err(e) => {
            abort!("Failed to parse token input to macro! {:?}", e)
        }
    };

    // Generate the output or present error
    let stream2 = match impl_struct_field_names_derive(ast) {
        Ok(v) => v,
        Err(e) => {
            abort!("{}", e.message)
        }
    };

    //Debug
    //println!("{stream2}");

    // Convert output from proc_macro2::TokensStream to proc_macro::TokenStream
    let stream1 = stream2.into();

    stream1
}

fn impl_struct_field_names_derive(
    ast: DeriveInput,
) -> Result<proc_macro2::TokenStream, MacroError> {
    let struct_data = match ast.data {
        Data::Struct(s) => s,
        _ => {
            return Err(MacroError {
                message: "This derive only supports Structs".to_string(),
            })
        }
    };

    let field_identifiers: Vec<String> = struct_data
        .fields
        .iter()
        .map(|f| f.ident.clone().expect("Field must have a type").to_string())
        .collect();

    let struct_identifier = ast.ident;

    let iterator_struct_name = {
        let val = format!("FieldNamesIter_{}", struct_identifier);
        let val: Ident =
            syn::parse_str(&val).expect("Failed to generate iterator_struct_name identifier token");
        val
    };

    let counter: std::ops::Range<usize> = 0..field_identifiers.len();

    Ok(quote::quote! {
        struct #iterator_struct_name {
            position: usize,
        }

        impl Iterator for #iterator_struct_name {
            type Item = &'static str;

            fn next(&mut self) -> Option<&'static str> {
                let identifier = match self.position {
                    #( # counter =>  #field_identifiers ),* ,
                    _ => return None
                };

                self.position +=1;

                Some(identifier)
            }
        }

        impl StructFieldNames<&'static str, #iterator_struct_name> for #struct_identifier {
            fn struct_field_names(&self) -> #iterator_struct_name {
                #iterator_struct_name {
                    position: 0
                }
            }

            fn struct_field_names_type() -> #iterator_struct_name {
                #iterator_struct_name {
                    position: 0
                }
            }
        }


    })
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(StructRefIter)]
pub fn struct_ref_iter_derive(item: TokenStream) -> TokenStream {
    // Convert to an abstract syntax tree for easier manipulation
    let ast = syn::parse(item);
    let ast: DeriveInput = match ast {
        Ok(v) => v,
        Err(e) => {
            abort!("Failed to parse token input to macro! {:?}", e)
        }
    };

    // Generate the output or present error
    let stream2 = match impl_struct_ref_iter_derive(ast) {
        Ok(v) => v,
        Err(e) => {
            abort!("{}", e.message)
        }
    };

    //Debug
    //println!("{stream2}");

    // Convert output from proc_macro2::TokensStream to proc_macro::TokenStream
    let stream1 = stream2.into();

    stream1
}

fn impl_struct_ref_iter_derive(ast: DeriveInput) -> Result<proc_macro2::TokenStream, MacroError> {
    // Get struct
    let struct_data: DataStruct = match ast.data.clone() {
        syn::Data::Struct(s) => s,
        _ => abort!("{}", "Only structes are supported for this macro"),
    };

    // Get struct name
    let identifier = ast.ident.clone();

    // Get struct field types
    let field_types: Vec<syn::Type> = struct_data.fields.iter().map(|f| f.ty.clone()).collect();

    //Get struct field identifiers
    let field_identifiers: Vec<Ident> = struct_data.fields.iter().map(|f| f.ident.clone().expect("Fields must all be named")).collect();



    let enum_variant_type_contained = {
        let mut variants = field_types.clone();
        variants.dedup();
        variants
    };

    let enum_variant_names = {
        let mut idents = Vec::new();
        for i in 0..enum_variant_type_contained.len() {
            idents.push(format!("T_{}", i));
        }

        let idents: Vec<Ident> = idents
            .iter()
            .map(|i| {
                let ident: Ident =
                    syn::parse_str(i).expect("Failed to parse enum_vairant_identifier token");
                ident
            })
            .collect();

        idents
    };

    let enum_variant_mappings = {
        let mut idents = Vec::new();
        let mut last_type: Type = field_types.first().unwrap().clone();
        let mut enum_index = 1;

        idents.push(enum_variant_names.first().unwrap().clone());

        for f in field_types.iter().skip(1) {
            if f.clone() != last_type {
                enum_index += 1;
                last_type = f.clone();
            }
            idents.push(enum_variant_names[enum_index - 1].clone());
        }

        idents
    };

    let iter_struct_name: Ident = {
        let val = format!("StructRefIter_{}", identifier);
        let val = syn::parse_str(&val).expect("Failed to make iter_struct_name");
        val
    };

    let struct_value_identifier: Ident = {
        let val = format!("StructRefValue_{}", identifier);
        let val = syn::parse_str(&val).expect("Failed to parse struct_value_identifier token");
        val
    };


    let counter0 = (0..(field_types.len()));

    Ok(quote::quote!{
        #[derive(Clone)]
        enum #struct_value_identifier <'a> {
            #( #enum_variant_names ( &'a #enum_variant_type_contained ) ),*
        }

        struct #iter_struct_name <'a> {
            position: usize,
            struct_ref: &'a #identifier
        }

        impl<'a> Iterator for #iter_struct_name <'a> {
            type Item = #struct_value_identifier <'a>;

            fn next(&mut self) -> Option<Self::Item> {
                let item = match self.position {
                    #( #counter0 => #struct_value_identifier :: #enum_variant_mappings  ( & self.struct_ref.#field_identifiers ) ),* ,
                    _ => return None
                };

                self.position += 1;
                Some(item)
            }
        }

        impl<'a> StructRefIter <'a, #struct_value_identifier <'a>, #iter_struct_name <'a>, #identifier >  for #identifier {
            fn struct_ref_iter(&'a self) ->  #iter_struct_name {
                #iter_struct_name {
                    position: 0,
                    struct_ref: self
                }
            }
        }

    })
}