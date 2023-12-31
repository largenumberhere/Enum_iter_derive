/// These are all utility traits for working with enums and structs. The simpler ones are less flexible but are easier to use.
/// All enums or structs used must have named fields, this may be changed at a later date
/// All these traits can be derived using the derive_macro sub-project. Most of them go on structs something like this:
/// [derive(derive_macro::StructToTuple)]
/// [derive(derive_macro::StructIter)]
/// Struct A{
///     value_1: u32
/// }
/// Many of the derives return an enum.
/// It's variants will be T_0(type_0), T_1(type_1) ... etc, where type_* refers to the type of each unique type in the struct in the order they are written in code.
///

#[cfg(test)]
mod tests;

/// Makes all the stuff in the derive_macro folder visible outside this crate as enum_iter_derive::derive_macro
pub extern crate derive_macro;

/// Print all the variant names of an enum
pub trait PrintEnumVariants {
    fn print_enum_variants(&self);
}

/// Get pointers to each value in a struct at runtime. Struct must contain only a single type for all fields!
pub trait FlatStructRefs<T> {
    fn flat_struct_refs(&self) -> Vec<&T>;
}

/// Get a pointer to each value in a struct. Struct must contain only a single type for all fields!
pub trait FlatStructValues<T: Clone> {
    fn flat_struct_values(&self) -> Vec<T>;
}

/// Iterate over each struct field. Struct must contain only a single type for all fields
pub trait FlatStructIter<'a, IterItem: 'a, Iter: Iterator<Item = &'a IterItem>> {
    fn flat_struct_iter(&'a self) -> Iter;
}

/// Convert a struct to a tuple. Can use any type
pub trait StructToTuple<TOUT> {
    fn struct_to_tuple(self) -> TOUT;
}

/// Get a list of strings with the type name assigned to each value in a struct
pub trait TypeNames {
    fn type_names(&self) -> Vec<String>;
    fn type_names_type() -> Vec<String>;
}

/// Must also derive StructToTuple, for this one to work (it uses it under the hood). It works with structs that contain various types! Match on the enum you receive, it will be called StructValue_*your struct name*.
pub trait StructIter<IterItem, Iter: Iterator<Item = IterItem>, T> :StructToTuple<T> {
    fn struct_iter(self) -> Iter;
}

/// Get an iterator to reference each field in a struct, the values can be any type. Match on the enum of types you receive. The enum will be called StructRefValue_*your struct name*
pub trait StructRefIter<'a ,IterItem: 'a, Iter: Iterator<Item = IterItem>, T >   {
    fn struct_ref_iter(&'a self) -> Iter;
}

/// Get the name (as written in the code) of each field in the struct.
pub trait StructFieldNames<IterItem, Iter: Iterator<Item = IterItem>> {
    fn struct_field_names(&self) -> Iter;
    fn struct_field_names_type() -> Iter;
}