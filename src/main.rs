// cargo +nightly  rustc --bin enum_iter_derive --  -Z macro-backtrace

// #[derive(derive_macro::PrintEnumVariants)]
// enum Letter {
//     A,
//     B,
//     C
// }
//
// #[derive(derive_macro::FlatStructIter)]
// #[derive(derive_macro::FlatStructValues)]
// struct MyStruct{
//     value_1: u32,
//     value_2: u32,
//     value_3: u32,
// }
//
// #[derive(derive_macro::FlatStructIter)]
// struct MyStruct2{
//     value0: i64,
//     value1: i64
// }

#[derive(
    derive_macro::TypeNames,
    derive_macro::StructToTuple,
    derive_macro::StructIter,
    derive_macro::StructFieldNames,
    Clone,
)]
struct MyStruct3 {
    value0: usize,
    value1: u32,
    value2: u32,
}

//use enum_iter_derive::{PrintEnumVariants, FlatStructRefs};
//use enum_iter_derive::FlatStructValues;
use enum_iter_derive::FlatStructIter;
//use enum_iter_derive::StructToTuple;
use enum_iter_derive::StructFieldNames;
use enum_iter_derive::StructIter;
use enum_iter_derive::TypeNames;

fn main() {
    // println!("Hello, world!");
    // let letter = Letter::A;
    //
    // letter.print_enum_variants();
    //
    // let my_struct = MyStruct{
    //     value_1: 1,
    //     value_2: 2,
    //     value_3: 3
    // };
    //
    // println!("iter");
    // let iter = my_struct.flat_struct_iter();
    // for v in iter{
    //     println!("{v}");
    // }
    //
    // let my_struct_2 = MyStruct2{
    //     value0: 0,
    //     value1: 1,
    // };
    //
    // println!("iter2");
    // for v in my_struct_2.flat_struct_iter(){
    //     println!("{v}");
    // }

    let a = 1;

    let my_struct_3 = MyStruct3 {
        value0: 0,
        value1: 1,
        value2: 3,
    };

    // let names = my_struct_3.type_names();
    // println!("{names:?}");

    // let vals = my_struct_3.struct_to_tuple();
    // println!("{vals:?}");

    let iter = my_struct_3.clone().struct_iter().enumerate();
    for (n, i) in iter {
        if let (Option::<u32>::Some(v)) = i.clone().into() {
            println!("Struct field number {n} is {v} and is a usize");
        } else if let (Option::<usize>::Some(v)) = i.clone().into() {
            println!("Struct field number {n} is {v} and is a usize");
        }
    }

    let tuple = my_struct_3.clone().struct_iter().into_inner();
    println!("{:?}", tuple);

    let iter = my_struct_3.clone().struct_iter();
    for field_value in iter {
        match field_value {
            StructValue_MyStruct3::T_0(usize_value) => {
                println!("One usize found!");
            }
            StructValue_MyStruct3::T_1(u32_value) => {
                println!("One u32 found!");
            }
        }
    }

    println!("MyStruct3 {{");
    for (field_name, field_value) in my_struct_3
        .clone()
        .struct_field_names()
        .into_iter()
        .zip(my_struct_3.clone().struct_iter())
    {
        let mut val: String;

        if let (Option::<u32>::Some(v)) = field_value.clone().into() {
            val = v.to_string();
        } else if let (Option::<usize>::Some(v)) = field_value.clone().into() {
            val = v.to_string();
        } else {
            unreachable!();
        }

        println!("\t{field_name}: {val}");
    }
    println!("}}")

    //
    // //TODO: make flat_struct_values() for structs with several different fields
    // for v in my_struct.flat_struct_values(){
    //     println!("'{v:?}'");
    // }
}

// fn do_something(smt: &std::any::Any){
//
//
// }
