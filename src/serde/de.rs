// TODO: put coprod in a separate module
use super::super::{
    Bool, Char, Context, Exception, Integer, Null, Pair, Rational, SExp, String, Symbol,
};
use super::error::{Error, Result};
use serde::de::{self, DeserializeSeed, MapAccess, Visitor};
use serde::Deserialize;

pub struct Deserializer<'de> {
    input: SExp<'de>,
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("Visit bool");
        visitor.visit_bool(
            self.input
                .expect_ref::<Bool, _>()
                .ok_or(Error::ExpectedBoolean)?
                .into(),
        )
    }
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(i64::from(
            self.input
                .expect_ref::<Integer, _>()
                .ok_or(Error::ExpectedInteger)?,
        ) as i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(i64::from(
            self.input
                .expect_ref::<Integer, _>()
                .ok_or(Error::ExpectedInteger)?,
        ) as i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(i64::from(
            self.input
                .expect_ref::<Integer, _>()
                .ok_or(Error::ExpectedInteger)?,
        ) as i32)
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(
            self.input
                .expect_ref::<Integer, _>()
                .ok_or(Error::ExpectedInteger)?
                .into(),
        )
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(i64::from(
            self.input
                .expect_ref::<Integer, _>()
                .ok_or(Error::ExpectedInteger)?,
        ) as u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(i64::from(
            self.input
                .expect_ref::<Integer, _>()
                .ok_or(Error::ExpectedInteger)?,
        ) as u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(i64::from(
            self.input
                .expect_ref::<Integer, _>()
                .ok_or(Error::ExpectedInteger)?,
        ) as u32)
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(i64::from(
            self.input
                .expect_ref::<Integer, _>()
                .ok_or(Error::ExpectedInteger)?,
        ) as u64)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(f64::from(
            self.input
                .expect_ref::<Rational, _>()
                .ok_or(Error::ExpectedRational)?,
        ) as f32)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(f64::from(
            self.input
                .expect_ref::<Rational, _>()
                .ok_or(Error::ExpectedRational)?,
        ))
    }
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    // arrays as JSON arrays of bytes. Handle that representation here.
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // need
        let assoc_list = AssocList { de: self };
        visitor.visit_map(assoc_list)
    }
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = self
            .input
            .expect_ref::<Symbol, _>()
            .ok_or(Error::ExpectedSymbol)?;
        visitor.visit_string(s.to_string())
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // TODO: Support more than bools - we need to match on the coprod
        unimplemented!();
    }
}

struct AssocList<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MapAccess<'de> for AssocList<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        let maybe_pair = self
            .de
            .input
            .expect_ref::<Pair, _>()
            .map(|p| Some(p))
            .ok_or(Error::ExpectedPair)
            .or(self
                .de
                .input
                .expect_ref::<Null, _>()
                .map(|_| None)
                .ok_or(Error::ExpectedEndOfAssocList))?;
        match maybe_pair {
            Some(pair) => {
                let head = pair.car();
                let head_pair = head.expect_ref::<Pair, _>().ok_or(Error::ExpectedPair)?;
                let symbol = head_pair.car();
                let mut new_de = Deserializer { input: symbol };
                seed.deserialize(&mut new_de).map(Some)
            }
            None => Ok(None),
        }
    }
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let pair = self
            .de
            .input
            .expect_ref::<Pair, _>()
            .ok_or(Error::ExpectedPair)?;
        let head = pair.car();
        let head_pair = head.expect_ref::<Pair, _>().ok_or(Error::ExpectedPair)?;
        let value = head_pair.cdr();
        let mut new_de = Deserializer { input: value };
        let r = seed.deserialize(&mut new_de);
        // Move the deserializer forwards
        self.de.input = pair.cdr();
        r
    }
}

pub fn from_sexp<'a, T>(s: SExp<'a>) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer { input: s };
    T::deserialize(&mut deserializer)
}

#[test]
fn test_bool() {
    let c = SExp::from(Bool::TRUE);
    let expected = true;
    assert_eq!(expected, from_sexp(c).unwrap());
}

#[test]
fn test_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Foo {
        bar: bool,
        foo: i64,
        baz: f64,
    }

    let context = Context::default();
    let foo = context
        .eval_string("'((foo . 3) (bar . #f) (baz . 5.5))")
        .unwrap();
    let expected = Foo {
        bar: false,
        foo: 3,
        baz: 5.5,
    };
    assert_eq!(expected, from_sexp(foo).unwrap());
}

#[test]
fn test_position_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Position {
        x: f64,
        y: f64,
        z: f64,
    }

    let context = Context::default();
    let foo = context
        .eval_string("'((x . 1.0) (y . 1.0) (z . 1.0))")
        .unwrap();
    let expected = Position {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    assert_eq!(expected, from_sexp(foo).unwrap());
}
