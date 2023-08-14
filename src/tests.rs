
#[test]
fn tests_are_found() {
    assert_eq!(1,1);
}


use crate::StructRefIter;
use crate::StructFieldNames;
use crate::TypeNames;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum PartnerStatus {
    Single,
    Partnered,
    Married
}

#[derive(
derive_macro::StructRefIter,
Debug,
derive_macro::TypeNames,
derive_macro::StructFieldNames
)]
pub struct Person {
    pub name: String,
    pub age: u64,
    pub id: u128,
    pub partner_status: PartnerStatus
}

#[test]
fn struct_ref_iter_and_type_names_and_struct_field_names() {
    let person_1 = Person{
        name: "Jane Citizen".to_string(),
        age: 25,
        id: 123_456_789,
        partner_status: PartnerStatus::Partnered
    };

    let mut type_names = Person::type_names_type();
    let mut type_names = type_names.iter();
    let mut  field_names = Person::struct_field_names_type();
    let mut struct_values = person_1.struct_ref_iter();
    let mut index = 0..;

    loop {
        // Try to move to next on all the iterators
        let type_name = type_names.next();
        let field_name = field_names.next();
        let value_enum = struct_values.next();
        let i = index.next();

        // All of the iterators should finish at exactly the same time
        if type_name.is_none() {
            assert!(field_name.is_none());
            assert!(value_enum.is_none());

            // All are finished at the same time as intended (except the range, which we aren't texting).
            break;
        }
        else {
            assert!(field_name.is_some());
            assert!(value_enum.is_some());
        }

        // Unwrap all the iterators we are testing, because we have checked them all by now
        let type_name = type_name.unwrap();
        let field_name = field_name.unwrap();
        let value_enum = value_enum.unwrap();

        // This iterator shouldn't ever end before the others
        let i = i.unwrap();

        // Check their values are as expected
        match i {
            0 => {
                let field_value = match value_enum {
                    StructRefValue_Person::T_0 (v) => v,
                    _ => {
                        panic!("Value {} had wrong type", i)
                    }
                };

                assert_eq!(type_name, "String");
                assert_eq!(field_name, "name");
                assert_eq!(field_value, "Jane Citizen");
            },
            1 => {
                let field_value = match value_enum {
                    StructRefValue_Person::T_1 (v) => v,
                    _ => {
                        panic!("Value {} had wrong type", i)
                    }
                };

                assert_eq!(type_name, "u64");
                assert_eq!(field_name, "age");
                assert_eq!(field_value, &25u64);
            },
            2 => {
                let field_value = match value_enum {
                    StructRefValue_Person::T_2 (v) => v,
                    _ => {
                        panic!("Value {} had wrong type", i)
                    }
                };

                assert_eq!(type_name, "u128");
                assert_eq!(field_name, "id");
                assert_eq!(field_value, & 123_456_789u128);
            },
            3 => {
                let field_value = match value_enum {
                    StructRefValue_Person::T_3 (v) => v,
                    _ => {
                        panic!("Value {} had wrong type", i)
                    }
                };

                assert_eq!(type_name, "PartnerStatus");
                assert_eq!(field_name, "partner_status");
                assert_eq!(field_value, &PartnerStatus::Partnered);
            }
            _ => unreachable!("Iterators went too long")
        }
    }


}





