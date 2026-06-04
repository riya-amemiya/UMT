//! Tests for instance_of function

use umt_rust::validate::umt_instance_of;

struct Animal {
    #[allow(dead_code)]
    name: String,
}

struct Plant {
    #[allow(dead_code)]
    species: String,
}

#[test]
fn test_instance_of_accepts_instances_of_the_type() {
    let pet = Animal {
        name: String::from("rex"),
    };
    assert!(umt_instance_of::<Animal>(&pet));
}

#[test]
fn test_instance_of_rejects_instances_of_unrelated_types() {
    let pet = Animal {
        name: String::from("rex"),
    };
    assert!(!umt_instance_of::<Plant>(&pet));
}

#[test]
fn test_instance_of_rejects_primitive_types() {
    let pet = Animal {
        name: String::from("rex"),
    };
    assert!(!umt_instance_of::<String>(&pet));
    assert!(!umt_instance_of::<i32>(&pet));
}

#[test]
fn test_instance_of_matches_primitive_values() {
    let text = String::from("rex");
    assert!(umt_instance_of::<String>(&text));
    assert!(!umt_instance_of::<i32>(&text));

    let number = 42_i32;
    assert!(umt_instance_of::<i32>(&number));
    assert!(!umt_instance_of::<String>(&number));
}
