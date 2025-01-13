/// Used for testing the `Reflective` derive macro.
#[allow(dead_code)]
#[derive(Reflective)]
enum ReflectiveEnum {
    VariantOne,
    VariantTwo,
}

/// Used for testing the `Reflective` derive macro.
#[allow(dead_code)]
#[derive(Reflective)]
struct ReflectiveStruct {
    data: ReflectiveEnum,
    field_1: u32,
    field_2: i32,
}

/// Used for testing the `Reflective` derive macro.
#[allow(dead_code)]
#[derive(Reflective)]
union ReflectiveUnion {
    field_1: u32,
    field_2: i32,
}

/// The `Reflective` derive macro ensures that the order of the fields is also in the same way.
#[test]
fn test_derive_reflective() {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Equality checks
    ////////////////////////////////////////////////////////////////////////////////////////////////
    assert_eq!(
        ReflectiveEnum::get_variants(),
        &["VariantOne", "VariantTwo"]
    );
    assert_eq!(
        ReflectiveStruct::get_fields(),
        &["data", "field_1", "field_2"]
    );
    assert_eq!(ReflectiveUnion::get_fields(), &["field_1", "field_2"]);

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Inequality checks
    ////////////////////////////////////////////////////////////////////////////////////////////////
    assert_ne!(
        ReflectiveEnum::get_variants(),
        &["VariantTwo", "VariantOne"]
    );
    assert_ne!(
        ReflectiveStruct::get_fields(),
        &["field_2", "field_1", "data"]
    );
    assert_ne!(ReflectiveUnion::get_fields(), &["field_2", "field_1"]);
}
