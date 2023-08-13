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

//#[derive(derive_macro::TypeNames)]
#[derive(derive_macro::StructToTuple)]
#[derive(derive_macro::StructIter)]
#[derive(Clone)]
struct MyStruct3{
    value0: usize,
    value1: u32,
    value2: u32
}

//use enum_iter_derive::{PrintEnumVariants, FlatStructRefs};
//use enum_iter_derive::FlatStructValues;
use enum_iter_derive::FlatStructIter;
//use enum_iter_derive::StructToTuple;
//use enum_iter_derive::TypeNames;
use enum_iter_derive::StructIter;

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




    let my_struct_3 = MyStruct3{
        value0: 0,
        value1: 1,
        value2: 3,
    };

    // let names = my_struct_3.type_names();
    // println!("{names:?}");

    // let vals = my_struct_3.struct_to_tuple();
    // println!("{vals:?}");

    let iter = my_struct_3.clone().struct_iter().enumerate();
    for (n, i) in iter{

        let v : Option<usize> = i.clone().into();
        if let Some(v) = v{
            println!("Struct field number {n} is {v} and is a usize");
        }

        let v : Option<u32> = i.clone().into();
        if let Some(v) = v{
            println!("Struct field number {n} is {v} and  is a u32");
        }
    }

    let tuple = my_struct_3.struct_iter().into_inner();
    println!("{:?}", tuple);



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

