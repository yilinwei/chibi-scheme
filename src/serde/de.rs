use crate::serde::error::{Error, Result};
use crate::sexp::{
    Bool, Char, Context, Exception, Integer, Null, Pair, Rational, SExp, String, Symbol,
};
use serde::de::{self, DeserializeSeed, MapAccess, Visitor};
use serde::Deserialize;
use std::cell;
use std::rc::Rc;
use std::convert::TryFrom;
use std::num::TryFromIntError;

pub struct Deserializer<'c> {
    input: SExp<'c>,
}

impl Deserializer<'_> {
    fn expect_bool(&self) -> Result<bool> {
        self.input
            .expect_ref::<Bool, _>()
            .ok_or(Error::ExpectedBoolean(format!("{:?}", self.input)))
            .map(|b| b.into())
    }

    fn try_from_integer<E: Into<TryFromIntError>, I: TryFrom<i32, Error = E>>(i: &Integer) -> Result<I> {
        I::try_from(i32::from(i)).map_err(|e| Error::IntegerTooLargeForBytes(e.into()))
    }

    fn expect_integer(&self) -> Result<&Integer> {
        self.input
            .expect_ref::<Integer, _>()
            .ok_or(Error::ExpectedInteger)
    }

    fn expect_i<E: Into<TryFromIntError>, I: TryFrom<i32, Error = E>>(&self) -> Result<I> {
        self.expect_integer()
            .and_then(|i| Deserializer::try_from_integer(i))
    }

    fn expect_rational<I: From<f32>>(&self) -> Result<I> {
        self.input
            .expect_ref::<Rational, _>()
            .ok_or(Error::ExpectedRational)
            .map(|i| I::from(f32::from(i)))
    }

    fn expect_symbol(&self) -> Result<&Symbol> {
        self
            .input
            .expect_ref::<Symbol, _>()
            .ok_or(Error::ExpectedSymbol)
    }
    fn expect_char(&self) -> Result<char> {
        self
            .input
            .expect_ref::<Char, _>()
            .ok_or(Error::ExpectedChar)
            .map(|c| char::from(c))
    }
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
        visitor.visit_bool(self.expect_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.expect_i::<_, i8>()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.expect_i::<_, i16>()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.expect_i::<_, i32>()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!("bignums")
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.expect_i::<_, u8>()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.expect_i::<_, u16>()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.expect_i::<_, u32>()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!("bignums")
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.expect_rational::<f32>()?)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.expect_rational::<f64>()?)
    }
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.expect_char()?)
    }

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
        visitor.visit_string(self.expect_symbol()?.to_string())
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // TODO: There must be a better way of doing this
        let cell = cell::RefCell::new(Some(visitor));
        let r: Result<V::Value> = self.input.0.to_ref().fold(frunk::hlist![
            |b: &Bool| (&mut cell.borrow_mut()).take().unwrap().visit_bool(b.into()),
            |c: &Char| (&mut cell.borrow_mut()).take().unwrap().visit_char(char::from(c)),
            |i: &Integer|  (&mut cell.borrow_mut()).take().unwrap().visit_i32(i32::from(i)),
            |i: &Rational| (&mut cell.borrow_mut()).take().unwrap().visit_f32(f32::from(i)),
            |n: &Null| unimplemented!(),
            |p: &Pair<'a>| unimplemented!(),
            |s: &String<'a>| unimplemented!(),
            |s: &Exception<'a>| unimplemented!(),
            |s: &Symbol<'a>| unimplemented!()
        ]);
        unimplemented!()
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
        foo: i32,
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

#[test]
fn test_casting() {
    assert_eq!(1 as i8, 1);
    let x: i64 = 12312324;
    assert_eq!(x as i8, 4);
}

#[test]
fn test_expected_boolean() {
    let c = SExp::from(Integer::from(12));
    let expected = true;
    assert_eq!(
        Some(Error::ExpectedBoolean("12".to_string())),
        from_sexp::<bool>(c).err()
    );
}
