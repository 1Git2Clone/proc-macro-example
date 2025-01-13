#[allow(dead_code)]
#[derive(Reflective)]
enum ReflectiveEnum {
    VariantOne,
    VariantTwo,
}

#[allow(dead_code)]
#[derive(Reflective)]
struct ReflectiveStruct {
    data: ReflectiveEnum,
    field_1: u32,
    field_2: i32,
}

#[allow(dead_code)]
#[derive(Reflective)]
union ReflectiveUnion {
    field_1: u32,
    field_2: i32,
}

#[test]
fn test_derive_reflective() {
    assert_eq!(
        ReflectiveEnum::get_variants(),
        &["VariantOne", "VariantTwo"]
    );
    assert_eq!(
        ReflectiveStruct::get_fields(),
        &["data", "field_1", "field_2"]
    );
    assert_eq!(ReflectiveUnion::get_fields(), &["field_1", "field_2"]);
}
