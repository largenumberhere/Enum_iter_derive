
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
//
#[derive(
    derive_macro::TypeNames,
    derive_macro::StructToTuple,
    derive_macro::StructIter,
    derive_macro::StructFieldNames,
    derive_macro::StructRefIter,
    Clone,
)]
//

struct MyStruct3 {
    value0: usize,
    value1: u32,
    value2: u32,
}

impl MyStruct3{
    fn do_stuff() -> () {
        println!("stuff was done");
    }
}

//use enum_iter_derive::{PrintEnumVariants, FlatStructRefs};
//use enum_iter_derive::FlatStructValues;
use enum_iter_derive::{FlatStructIter, StructRefIter};
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


    for name in MyStruct3::struct_field_names_type() {
        println!("The struct definition contains {name}");
    }

    for type_name in MyStruct3::type_names_type(){
        println!("Struct three contains the types {}",type_name);
    }

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
    println!("}}");
    //
    // for value in my_struct_3.struct_ref_iter() {
    //     if let (Option::<&u32>::Some(v)) = value.clone().into() {
    //         println!("u32! {}", v)
    //     }
    //     else if let (Option::<&usize>::Some(v)) = value.clone().into() {
    //         println!("usize! {}", v)
    //     }
    //
    // }

    let iter = my_struct_3.struct_ref_iter();
    for value in iter{
        match value {
            StructRefValue_MyStruct3::T_0(v) => {
                println!("{v}");
            },

            StructRefValue_MyStruct3::T_1(v) => {
                println!("{v}")
            }

            // Value_Mystruct3::T_0(usize_value) => {
            //     println!("usize_value {}", usize_value);
            // }
            // Value_Mystruct3::T_1(u32_value) => {
            //     println!("u32_value {}", u32_value);
            // }

        }

        //
        // if let (Result::<&u32, _>:: Ok(v)) = value.try_into() {
        //     println!("found {v}")
        //
        // }
        // else if let (Result::<&usize, _>::Ok(v)) = value.try_into() {
        //     println!("found {}", v);
        //
        // }
        // else {
        //     unreachable!()
        // }
    }




    //
    // //TODO: make flat_struct_values() for structs with several different fields
    // for v in my_struct.flat_struct_values(){
    //     println!("'{v:?}'");
    // }
}

// struct Iter_Mystruct3<'a>{
//     position: usize,
//     struct_ref: &'a MyStruct3
// }
//
// impl<'a> Iterator for Iter_Mystruct3<'a>{
//     type Item = Value_Mystruct3<'a>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//          let item: Value_Mystruct3 = match self.position {
//              0 => Value_Mystruct3::T_0( & self.struct_ref.value0),
//              1 => Value_Mystruct3::T_1( & self.struct_ref.value1),
//              2 => Value_Mystruct3::T_1( & self.struct_ref.value2),
//              _ => {
//                  return None
//              }
//          };
//
//         self.position +=1;
//
//         Some(item)
//     }
//
// }
//
// #[derive(Clone)]
// enum Value_Mystruct3<'a>{
//     T_0 (&'a usize),
//     T_1 (&'a u32)
// }
//
//
// impl<'a> StructRefIter<'a, Value_Mystruct3<'a>, Iter_Mystruct3<'a>, MyStruct3> for MyStruct3{
//     fn struct_ref_iter(&'a self) -> Iter_Mystruct3<'a> {
//         Iter_Mystruct3{
//             position: 0,
//             struct_ref: self
//         }
//
//     }
// }
