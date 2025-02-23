use std::borrow::Cow;
use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::{from_bytes, Value};
use crate::{ByteArray, IntArray, LongArray, Tag};

use super::builder::Builder;
use super::Single;
use serde::{Deserialize, Serialize};

#[test]
fn error_impls_sync_send() {
    fn i<T: Clone + Send + Sync + std::error::Error>(_: T) {}
    i(Error::invalid_tag(1));
}

#[test]
fn descriptive_error_on_gzip_magic() {
    let r = from_bytes::<()>(&[0x1f, 0x8b]);
    assert!(matches!(r, Result::Err(_)));
    let e = r.unwrap_err();
    assert!(e.to_string().to_lowercase().contains("gzip"));
}

#[test]
fn simple_byte() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        abc: i8,
        def: i8,
    }

    let payload = Builder::new()
        .tag(Tag::Compound)
        .name("object")
        .tag(Tag::Byte)
        .name("abc")
        .byte_payload(123)
        .tag(Tag::Byte)
        .name("def")
        .byte_payload(111)
        .tag(Tag::End)
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.abc, 123);
    assert_eq!(v.def, 111);
    Ok(())
}

#[test]
fn simple_floats() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        f: f32,
        d: f64,
    }

    let payload = Builder::new()
        .tag(Tag::Compound)
        .name("object")
        .float("f", 1.23)
        .double("d", 2.34)
        .tag(Tag::End)
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.f, 1.23);
    assert_eq!(v.d, 2.34);

    Ok(())
}

#[test]
fn bool_from_integral() {
    #[derive(Deserialize)]
    struct V {
        byte_true: bool,
        byte_false: bool,
        short: bool,
        int: bool,
        long: bool,
    }

    let payload = Builder::new()
        .start_compound("object")
        .byte("byte_true", 1)
        .byte("byte_false", 0)
        .short("short", 2)
        .int("int", 3)
        .long("long", 4)
        .tag(Tag::End)
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert!(v.byte_true);
    assert!(!v.byte_false);
    assert!(v.short);
    assert!(v.int);
    assert!(v.long);
}

#[test]
fn bool_from_none_integral() {
    #[derive(Deserialize)]
    struct V {
        _b: bool,
    }

    let payload = Builder::new()
        .start_compound("object")
        .string("_b", "true") // intentionally does NOT work.
        .tag(Tag::End)
        .build();

    let v: Result<V> = from_bytes(payload.as_slice());

    assert!(v.is_err());
}

#[test]
#[cfg(not(no_integer128))]
fn i128_from_int_array() {
    #[derive(Deserialize)]
    struct V {
        max: u128,
        min: i128,
        zero: i128,
        counting: u128,
    }

    let payload = Builder::new()
        .start_compound("object")
        .tag(Tag::IntArray)
        .name("max")
        .int_payload(4)
        // All bits are 1
        .int_array_payload(&[u32::MAX as i32; 4])
        .tag(Tag::IntArray)
        .name("min")
        .int_payload(4)
        // Only first bit is 1
        .int_array_payload(&[1 << 31, 0, 0, 0])
        .tag(Tag::IntArray)
        .name("zero")
        .int_payload(4)
        .int_array_payload(&[0; 4])
        .tag(Tag::IntArray)
        .name("counting")
        .int_payload(4)
        .int_array_payload(&[1, 2, 3, 4])
        .tag(Tag::End)
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.max, u128::MAX);
    assert_eq!(v.min, i128::MIN);
    assert_eq!(v.zero, 0);
    // Calculated with: 1 << 96 | 2 << 64 | 3 << 32 | 4
    assert_eq!(v.counting, 79228162551157825753847955460); 
}

#[test]
#[cfg(not(no_integer128))]
fn i128_from_invalid_int_array() {
    #[derive(Deserialize)]
    struct V {
        _i: i128,
    }

    let payload = Builder::new()
        .start_compound("object")
        .tag(Tag::IntArray)
        .name("_i")
        .int_payload(3)
        .int_array_payload(&[1, 2, 3])
        .tag(Tag::End)
        .build();
    let v: Result<V> = from_bytes(payload.as_slice());
    assert!(v.is_err());

    // Although number of bytes is correct, won't be accepted
    let payload = Builder::new()
        .start_compound("object")
        .tag(Tag::ByteArray)
        .name("_i")
        .int_payload(16)
        .int_array_payload(&[1; 16])
        .tag(Tag::End)
        .build();
    let v: Result<V> = from_bytes(payload.as_slice());
    assert!(v.is_err());
}

#[test]
fn simple_short_to_i16() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        abc: i16,
    }

    let payload = Builder::new()
        .tag(Tag::Compound)
        .name("object")
        .tag(Tag::Short)
        .name("abc")
        .short_payload(256)
        .tag(Tag::End)
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.abc, 256);
    Ok(())
}

#[test]
fn simple_short_to_u16() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        abc: u16,
    }

    let payload = Builder::new()
        .tag(Tag::Compound)
        .name("object")
        .tag(Tag::Short)
        .name("abc")
        .short_payload(256)
        .tag(Tag::End)
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.abc, 256);
    Ok(())
}

#[test]
fn short_to_u16_out_of_range_errors() {
    #[derive(Deserialize)]
    struct V {
        _abc: u16,
    }

    let payload = Builder::new()
        .tag(Tag::Compound)
        .name("object")
        .tag(Tag::Short)
        .name("_abc")
        .short_payload(-123)
        .tag(Tag::End)
        .build();

    let v: Result<V> = from_bytes(payload.as_slice());
    assert!(v.is_err());
}

#[test]
fn multiple_fields() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        a: u8,
        b: u16,
    }

    let payload = Builder::new()
        .tag(Tag::Compound)
        .name("object")
        .tag(Tag::Byte)
        .name("a")
        .byte_payload(123)
        .tag(Tag::Short)
        .name("b")
        .short_payload(1024)
        .tag(Tag::End)
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.a, 123);
    assert_eq!(v.b, 1024);
    Ok(())
}

#[test]
fn numbers_into_u32() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        a: u32,
        b: u32,
        c: u32,
    }

    let payload = Builder::new()
        .tag(Tag::Compound)
        .name("object")
        .tag(Tag::Byte)
        .name("a")
        .byte_payload(123)
        .tag(Tag::Short)
        .name("b")
        .short_payload(2 << 8)
        .tag(Tag::Int)
        .name("c")
        .int_payload(2 << 24)
        .tag(Tag::End)
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.a, 123);
    assert_eq!(v.b, 2 << 8);
    assert_eq!(v.c, 2 << 24);
    Ok(())
}

#[test]
fn string_into_ref_str() -> Result<()> {
    #[derive(Deserialize)]
    struct V<'a> {
        a: &'a str,
    }

    let payload = Builder::new()
        .tag(Tag::Compound)
        .name("object")
        .tag(Tag::String)
        .name("a")
        .string_payload("hello")
        .tag(Tag::End)
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!("hello", v.a);

    Ok(())
}

#[test]
fn string_into_string() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        a: String,
    }

    let payload = Builder::new()
        .tag(Tag::Compound)
        .name("object")
        .tag(Tag::String)
        .name("a")
        .string_payload("hello")
        .tag(Tag::End)
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!("hello", v.a);

    Ok(())
}

#[test]
fn nested_compound() -> Result<()> {
    #[derive(Deserialize)]
    struct Nested {
        b: u32,
    }

    #[derive(Deserialize)]
    struct V {
        a: u32,
        nested: Nested,
    }

    let payload = Builder::new()
        .tag(Tag::Compound)
        .name("object")
        .tag(Tag::Byte)
        .name("a")
        .byte_payload(123)
        .tag(Tag::Compound)
        .name("nested")
        .tag(Tag::Byte)
        .name("b")
        .byte_payload(1)
        .tag(Tag::End)
        .tag(Tag::End)
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.a, 123);
    assert_eq!(v.nested.b, 1);
    Ok(())
}

#[test]
fn unwanted_byte() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        a: u32,
    }

    let payload = Builder::new()
        .tag(Tag::Compound)
        .name("object")
        .tag(Tag::Byte)
        .name("a")
        .byte_payload(123)
        .tag(Tag::Byte)
        .name("b")
        .byte_payload(1)
        .tag(Tag::End)
        .build();

    // requires impl of deserialize_ignored_any
    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.a, 123);
    Ok(())
}

#[test]
fn unwanted_primative_payloads() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        a: u32,
    }

    let payload = Builder::new()
        .start_compound("object")
        .byte("a", 123)
        .short("b", 1)
        .int("c", 2)
        .long("d", 3)
        .string("e", "test")
        .float("f", 1.23)
        .double("g", 2.34)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.a, 123);
    Ok(())
}

#[test]
fn simple_hashmap() {
    let payload = Builder::new()
        .start_compound("object")
        .int("a", 1)
        .int("b", 2)
        .end_compound()
        .build();

    let v: HashMap<&str, i32> = from_bytes(payload.as_slice()).unwrap();
    assert_eq!(v["a"], 1);
    assert_eq!(v["b"], 2);
}

#[test]
fn simple_hashmap_with_enum() {
    let payload = Builder::new()
        .start_compound("object")
        .int("a", 1)
        .string("b", "2")
        .end_compound()
        .build();

    #[derive(Deserialize, PartialEq, Debug)]
    #[serde(untagged)]
    enum E<'a> {
        Int(i32),
        String(&'a str),
    }

    let v: HashMap<&str, E> = from_bytes(payload.as_slice()).unwrap();
    assert_eq!(v["a"], E::Int(1));
    assert_eq!(v["b"], E::String("2"));
}

#[test]
fn nested_hashmaps_with_enums() {
    let payload = Builder::new()
        .start_compound("object")
        .int("a", 1)
        .start_compound("b")
        .int("inner", 2)
        .end_compound()
        .end_compound()
        .build();

    #[derive(Deserialize, PartialEq, Debug)]
    #[serde(untagged)]
    enum E<'a> {
        Int(i32),
        #[serde(borrow)]
        Map(HashMap<&'a str, i32>),
    }

    let v: HashMap<&str, E> = from_bytes(payload.as_slice()).unwrap();
    assert_eq!(v["a"], E::Int(1));
    match v["b"] {
        E::Map(ref map) => assert_eq!(map["inner"], 2),
        _ => panic!(),
    }
}

#[test]
fn simple_list() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        a: Vec<u32>,
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_list("a", Tag::Byte, 3)
        .byte_payload(1)
        .byte_payload(2)
        .byte_payload(3)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.a, [1, 2, 3]);
    Ok(())
}

#[test]
fn list_of_compounds() -> Result<()> {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Inner {
        a: u32,
    }

    #[derive(Deserialize)]
    struct V {
        inner: Option<Vec<Inner>>,
        after: i8,
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_list("inner", Tag::Compound, 3)
        .byte("a", 1)
        .start_compound("ignored")
        .end_compound()
        .end_compound()
        .byte("a", 2)
        .end_compound()
        .byte("a", 3)
        .end_compound()
        .byte("after", 123)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(
        v.inner,
        Some(vec![Inner { a: 1 }, Inner { a: 2 }, Inner { a: 3 }])
    );
    assert_eq!(v.after, 123);
    Ok(())
}

#[test]
fn complex_nesting() -> Result<()> {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Inner {
        a: u32,
        b: Option<Vec<i32>>,
    }

    #[derive(Deserialize)]
    struct V {
        inner: Vec<Inner>,
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_list("inner", Tag::Compound, 3)
        .byte("a", 1)
        .start_list("b", Tag::Int, 2)
        .int_payload(1)
        .int_payload(2)
        .start_compound("ignored")
        .end_compound()
        .end_compound()
        .byte("a", 2)
        .end_compound()
        .byte("a", 3)
        .end_compound()
        .byte("after", 123)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(
        v.inner,
        vec![
            Inner {
                a: 1,
                b: Some(vec![1, 2])
            },
            Inner { a: 2, b: None },
            Inner { a: 3, b: None }
        ]
    );
    Ok(())
}

#[test]
fn optional() -> Result<()> {
    #[derive(Deserialize)]
    struct V<'a> {
        opt1: Option<&'a str>,
        opt2: Option<&'a str>,
    }

    let payload = Builder::new()
        .start_compound("object")
        .string("opt1", "hello")
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(v.opt1, Some("hello"));
    assert_eq!(v.opt2, None);
    Ok(())
}

#[test]
fn unit_just_requires_presense() -> Result<()> {
    #[derive(Deserialize)]
    struct Foo;

    #[derive(Deserialize)]
    struct V {
        _unit: (),
        _unit_struct: Foo,
    }

    let payload = Builder::new()
        .start_compound("object")
        .byte("_unit", 0)
        .byte("_unit_struct", 0)
        .end_compound()
        .build();

    assert!(from_bytes::<V>(payload.as_slice()).is_ok());
    Ok(())
}

#[test]
fn unit_not_present_errors() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        _unit: (),
    }

    let payload = Builder::new()
        .start_compound("object")
        .end_compound()
        .build();

    assert!(from_bytes::<V>(payload.as_slice()).is_err());
    Ok(())
}

#[test]
fn ignore_compound() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        a: u8,
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_compound("inner")
        .byte("ignored", 1)
        .end_compound()
        .start_compound("inner")
        .byte("ignored", 1)
        .end_compound()
        .byte("a", 123)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert_eq!(v.a, 123);

    Ok(())
}

#[test]
fn ignore_primitives_in_ignored_compound() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        a: u8,
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_compound("ignoreall")
        .float("ignored", 1.23)
        .double("ignored", 1.234)
        .byte("ig", 1)
        .short("ig", 2)
        .int("ig", 3)
        .long("ig", 4)
        .string("ig", "hello")
        .end_compound()
        .byte("a", 123)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert_eq!(v.a, 123);

    Ok(())
}

#[test]
fn ignore_list() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        a: u8,
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_list("ignored", Tag::Byte, 2)
        .byte_payload(1)
        .byte_payload(2)
        .byte("a", 123)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert_eq!(v.a, 123);

    Ok(())
}

#[test]
fn ignore_list_of_compound() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        a: u8,
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_list("ignored", Tag::Compound, 2)
        .byte("a", 1) // ignored!
        .end_compound()
        .end_compound()
        .byte("a", 123)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert_eq!(v.a, 123);

    Ok(())
}

#[test]
fn byte_array_from_list_bytes() -> Result<()> {
    #[derive(Deserialize)]
    struct V<'a> {
        arr: &'a [u8],
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_list("arr", Tag::Byte, 3)
        .byte_payload(1)
        .byte_payload(2)
        .byte_payload(3)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert_eq!(v.arr, [1, 2, 3]);

    Ok(())
}

#[test]
fn byte_array_from_nbt_short_list() -> Result<()> {
    #[derive(Deserialize)]
    struct V<'a> {
        arr: &'a [u8],
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_list("arr", Tag::Short, 3)
        .short_payload(1)
        .short_payload(2)
        .short_payload(3)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert_eq!(v.arr, [0, 1, 0, 2, 0, 3]);

    Ok(())
}

#[test]
fn byte_array_from_nbt_int_list() -> Result<()> {
    #[derive(Deserialize)]
    struct V<'a> {
        arr: &'a [u8],
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_list("arr", Tag::Int, 2)
        .int_payload(1)
        .int_payload(2)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert_eq!(v.arr, [0, 0, 0, 1, 0, 0, 0, 2]);

    Ok(())
}

#[test]
fn byte_array_from_nbt_long_list() -> Result<()> {
    #[derive(Deserialize)]
    struct V<'a> {
        arr: &'a [u8],
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_list("arr", Tag::Long, 2)
        .long_payload(1)
        .long_payload(2)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert_eq!(v.arr, [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2]);

    Ok(())
}

#[test]
fn newtype_struct() -> Result<()> {
    #[derive(Deserialize)]
    struct Inner(u8);

    #[derive(Deserialize)]
    struct V {
        a: Inner,
    }

    let payload = Builder::new()
        .start_compound("object")
        .byte("a", 123)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert_eq!(v.a.0, 123);

    Ok(())
}

#[test]
fn vec_from_nbt_byte_array() -> Result<()> {
    #[derive(Deserialize)]
    struct V {
        a: ByteArray,
        b: IntArray,
        c: LongArray,
    }

    let payload = Builder::new()
        .start_compound("object")
        .tag(Tag::ByteArray)
        .name("a")
        .int_payload(3)
        .byte_array_payload(&[1, 2, 3])
        .tag(Tag::IntArray)
        .name("b")
        .int_payload(3)
        .int_array_payload(&[4, 5, 6])
        .tag(Tag::LongArray)
        .name("c")
        .int_payload(3)
        .long_array_payload(&[7, 8, 9])
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert!(v.a.iter().eq(&[1, 2, 3]));
    assert_eq!(*v.b, [4, 5, 6]);
    assert_eq!(*v.c, [7, 8, 9]);
    Ok(())
}

#[derive(Deserialize)]
struct Blockstates<'a>(&'a [u8]);

#[test]
fn blockstates() -> Result<()> {
    #[derive(Deserialize)]
    struct V<'a> {
        #[serde(borrow)]
        states: Blockstates<'a>,
    }

    let payload = Builder::new()
        .start_compound("object")
        .tag(Tag::LongArray)
        .name("states")
        .int_payload(3)
        .long_payload(1)
        .long_payload(2)
        .long_payload(3)
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert_eq!(
        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3],
        v.states.0
    );

    Ok(())
}

#[test]
fn ignore_integral_arrays() -> Result<()> {
    #[derive(Deserialize)]
    struct V {}

    let payload = Builder::new()
        .start_compound("object")
        .tag(Tag::ByteArray)
        .name("a")
        .int_payload(3)
        .byte_array_payload(&[1, 2, 3])
        .tag(Tag::IntArray)
        .name("b")
        .int_payload(3)
        .int_array_payload(&[4, 5, 6])
        .tag(Tag::LongArray)
        .name("c")
        .int_payload(3)
        .long_array_payload(&[7, 8, 9])
        .end_compound()
        .build();

    assert!(from_bytes::<V>(payload.as_slice()).is_ok());
    Ok(())
}

#[test]
fn fixed_array() -> Result<()> {
    #[derive(Deserialize)]
    struct Inner<'a> {
        a: &'a [u8],
    }
    #[derive(Deserialize)]
    pub struct Level<'a> {
        #[serde(borrow)]
        inner: [Inner<'a>; 3],
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_list("inner", Tag::Compound, 3)
        .byte_array("a", &[1, 2, 3])
        .end_compound()
        .byte_array("a", &[4, 5, 6])
        .end_compound()
        .byte_array("a", &[7, 8, 9])
        .end_compound() // end of list
        .end_compound() // end of outer compound
        .build();

    let v: Level = from_bytes(payload.as_slice())?;
    assert_eq!([1, 2, 3], v.inner[0].a);
    assert_eq!([4, 5, 6], v.inner[1].a);
    assert_eq!([7, 8, 9], v.inner[2].a);
    Ok(())
}

#[test]
fn type_mismatch_string() -> Result<()> {
    #[derive(Deserialize, Debug)]
    pub struct V {
        _a: String,
    }

    let payload = Builder::new()
        .start_compound("object")
        .int("_a", 123)
        .end_compound() // end of outer compound
        .build();

    let res = from_bytes::<V>(payload.as_slice());

    assert!(res.is_err());
    Ok(())
}

#[test]
fn basic_palette_item() -> Result<()> {
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct PaletteItem {
        name: String,
        #[allow(dead_code)]
        properties: HashMap<String, String>,
    }

    let payload = Builder::new()
        .start_compound("object")
        .start_compound("Properties")
        .string("lit", "false")
        .end_compound()
        .string("Name", "minecraft:redstone_ore")
        .end_compound()
        .build();

    let res: PaletteItem = from_bytes(payload.as_slice())?;

    assert_eq!(res.name, "minecraft:redstone_ore");

    Ok(())
}

#[test]
fn basic_unit_variant_enum() {
    #[derive(Deserialize, Debug, PartialEq)]
    enum Letter {
        #[serde(rename = "a")]
        A,
        B,
        C,
    }

    #[derive(Deserialize, Debug)]
    struct V {
        letter: Letter,
    }
    let payload = Builder::new()
        .start_compound("")
        .string("letter", "a")
        .end_compound()
        .build();

    let res: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(res.letter, Letter::A);
}

#[test]
fn basic_newtype_variant_enum() {
    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(untagged)]
    enum Letter {
        A(u32),
        B(String),
    }

    #[derive(Deserialize, Debug)]
    struct V {
        letter: Letter,
    }
    let payload = Builder::new()
        .start_compound("")
        .string("letter", "abc") // should deserialize as B?
        .end_compound()
        .build();

    let res: V = from_bytes(payload.as_slice()).unwrap();

    assert_eq!(res.letter, Letter::B("abc".to_owned()));
}

#[test]
fn unit_variant_enum() -> Result<()> {
    #[derive(Deserialize, PartialEq, Debug)]
    enum E {
        A,
        B,
        C,
    }
    #[derive(Deserialize)]
    struct V {
        e1: E,
        e2: E,
        e3: E,
    }

    let payload = Builder::new()
        .start_compound("object")
        .string("e1", "A")
        .string("e2", "B")
        .string("e3", "C")
        .end_compound()
        .build();

    let v: V = from_bytes(payload.as_slice())?;
    assert_eq!(v.e1, E::A);
    assert_eq!(v.e2, E::B);
    assert_eq!(v.e3, E::C);
    Ok(())
}

#[test]
fn integrals_in_fullvalue() {
    let payload = Builder::new()
        .start_compound("object")
        .int("a", 1)
        .int("b", 2)
        .end_compound()
        .build();

    let v: Value = from_bytes(payload.as_slice()).unwrap();
    match v {
        Value::Compound(ref map) => {
            let a = &map["a"];
            match a {
                Value::Int(i) => assert_eq!(*i, 1),
                _ => panic!("{:?}", a),
            }
        }
        _ => panic!(),
    }
}

#[test]
fn floating_in_fullvalue() {
    let payload = Builder::new()
        .start_compound("object")
        .float("a", 1.0)
        .double("b", 2.0)
        .float("c", 3.0)
        .end_compound()
        .build();

    let val: Value = from_bytes(payload.as_slice()).unwrap();
    match val {
        Value::Compound(ref map) => {
            let a = &map["a"];
            match a {
                Value::Float(f) => assert_eq!(*f, 1.0),
                _ => panic!("{:?}", a),
            }
            let b = &map["b"];
            match b {
                Value::Double(f) => assert_eq!(*f, 2.0),
                _ => panic!("{:?}", a),
            }
            let c = &map["c"];
            match c {
                Value::Float(f) => assert_eq!(*f, 3.0),
                _ => panic!("{:?}", a),
            }
        }
        _ => panic!(),
    }
}

#[test]
fn byte_array_in_fullvalue() {
    let payload = Builder::new()
        .start_compound("object")
        .byte_array("a", &[1, 2, 3])
        .end_compound()
        .build();

    let v: Value = from_bytes(payload.as_slice()).unwrap();
    match v {
        Value::Compound(ref map) => {
            let a = &map["a"];
            match a {
                Value::ByteArray(arr) => assert!(arr.iter().eq(&[1, 2, 3])),
                _ => panic!("{:?}", a),
            }
        }
        _ => panic!(),
    }
}

#[test]
fn int_array_in_fullvalue() {
    let payload = Builder::new()
        .start_compound("object")
        .int_array("a", &[1, 2, 3])
        .end_compound()
        .build();

    let v: Value = from_bytes(payload.as_slice()).unwrap();
    match v {
        Value::Compound(ref map) => {
            let a = &map["a"];
            match a {
                Value::IntArray(arr) => assert_eq!(&**arr, &[1, 2, 3]),
                _ => panic!("incorrect value: {:?}", a),
            }
        }
        _ => panic!(),
    }
}

#[test]
fn trailing_bytes() {
    // Can't really see a way to assert that there are no trailing bytes. We
    // don't return how far in to the input we got.
    let mut input = Builder::new().start_compound("").end_compound().build();
    input.push(1);
    let _v: Value = from_bytes(&input).unwrap();
}

#[test]
fn cesu8_string_in_nbt() {
    // In the builder we always convert to java cesu8 form for strings anyway,
    // but this test is more explicit and includes some unicode that actually
    // has a different representation in cesu8 and utf-8.
    let modified_unicode_str = cesu8::to_java_cesu8("😈");

    let input = Builder::new()
        .start_compound("")
        .tag(Tag::String)
        .name("hello")
        .raw_len(modified_unicode_str.len())
        .raw_bytes(&modified_unicode_str)
        .end_compound()
        .build();

    let _v: Value = from_bytes(&input).unwrap();
}

#[test]
fn cannot_borrow_cesu8_if_diff_repr() {
    #[derive(Deserialize, Debug)]
    pub struct V<'a> {
        _name: &'a str,
    }

    let modified_unicode_str = cesu8::to_java_cesu8("😈");

    let input = Builder::new()
        .start_compound("")
        .tag(Tag::String)
        .name("_name")
        .raw_len(modified_unicode_str.len())
        .raw_bytes(&modified_unicode_str)
        .end_compound()
        .build();

    let v: Result<V> = from_bytes(&input);
    assert!(v.is_err());
}

#[test]
fn can_borrow_cesu8_if_same_repr() {
    #[derive(Deserialize, Debug)]
    pub struct V<'a> {
        name: &'a str,
    }

    let modified_unicode_str = cesu8::to_java_cesu8("abc");

    let input = Builder::new()
        .start_compound("")
        .tag(Tag::String)
        .name("name")
        .raw_len(modified_unicode_str.len())
        .raw_bytes(&modified_unicode_str)
        .end_compound()
        .build();

    let v: Result<V> = from_bytes(&input);
    assert!(v.is_ok());
    assert_eq!("abc", v.unwrap().name);
}

#[test]
fn can_cow_cesu8() {
    #[derive(Deserialize, Debug)]
    pub struct V<'a> {
        owned: Cow<'a, str>,
        #[serde(borrow, deserialize_with = "crate::borrow::deserialize_cow_str")]
        borrowed: Cow<'a, str>,
    }

    let modified_unicode_str = cesu8::to_java_cesu8("😈");

    let input = Builder::new()
        .start_compound("")
        .tag(Tag::String)
        .name("owned")
        .raw_len(modified_unicode_str.len())
        .raw_bytes(&modified_unicode_str)
        .string("borrowed", "abc")
        .end_compound()
        .build();

    let v: V = from_bytes(&input).unwrap();
    assert!(matches!(v.owned, Cow::Owned(_)));
    assert_eq!("😈", v.owned);

    assert!(matches!(v.borrowed, Cow::Borrowed(_)));
    assert_eq!("abc", v.borrowed);
}

#[test]
fn large_list() {
    let input = [10, 0, 0, 9, 0, 0, 10, 4, 0, 5, 252];
    let _v: Result<Value> = from_bytes(&input);
}

#[test]
fn hashmap_with_bytes() {
    // Users should be able to decode strings as borrowed byte strings if they
    // really want to.
    let input = Builder::new()
        .start_compound("")
        .string("hello", "😈")
        .end_compound()
        .build();

    let v: HashMap<&[u8], &[u8]> = from_bytes(&input).unwrap();
    assert_eq!(
        cesu8::from_java_cesu8(v["hello".as_bytes()])
            .unwrap()
            .as_bytes(),
        "😈".as_bytes()
    );
}

#[test]
fn hashmap_with_byte_buf() {
    // Users should be able to decode strings as borrowed byte strings if they
    // really want to.
    let input = Builder::new()
        .start_compound("")
        .string("hello", "😈")
        .end_compound()
        .build();

    let _v: HashMap<&[u8], serde_bytes::ByteBuf> = from_bytes(&input).unwrap();
}

#[test]
fn chars() {
    let input = Builder::new()
        .start_compound("")
        .string("val", "a")
        .end_compound()
        .build();
    let v: Single<char> = from_bytes(&input).unwrap();
    assert_eq!('a', v.val);
}

#[test]
fn enum_variant_types() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum Letter {
        NewType(u32),
        Tuple(u8, u8, u8),
        Struct { a: String },
    }

    let newtype_input = Builder::new()
        .start_compound("")
        .string("val", "NewType")
        .end_compound()
        .build();
    let v: Result<Single<Letter>> = from_bytes(&newtype_input);
    assert!(matches!(v, Err(_)));

    let tuple_input = Builder::new()
        .start_compound("")
        .string("val", "Tuple")
        .end_compound()
        .build();

    let v: Result<Single<Letter>> = from_bytes(&tuple_input);
    assert!(matches!(v, Err(_)));

    let struct_input = Builder::new()
        .start_compound("")
        .string("val", "Struct")
        .end_compound()
        .build();
    let v: Result<Single<Letter>> = from_bytes(&struct_input);
    assert!(matches!(v, Err(_)));
}

#[test]
fn tuple_struct() {
    #[derive(Deserialize, Serialize)]
    struct Rgb(u8, u8, u8);

    let input = Builder::new()
        .start_compound("")
        .start_list("val", Tag::Byte, 3)
        .byte_payload(1)
        .byte_payload(2)
        .byte_payload(3)
        .end_compound()
        .build();

    let v: Single<Rgb> = from_bytes(&input).unwrap();
    assert!(matches!(v.val, Rgb(1, 2, 3)));
}
