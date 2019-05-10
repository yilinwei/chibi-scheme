use crate::serde::error::{Error, Result};
use crate::sexp::{
    Bool, Char, Context, Exception, Integer, Null, Pair, Rational, SExp, String, Symbol,
};
use serde::de::{self, DeserializeSeed, MapAccess, Visitor};
use serde::Deserialize;
use std::cell;
use std::convert::TryFrom;
use std::num::TryFromIntError;

pub struct Deserializer<'c> {
    input: SExp<'c>,
}

impl<'de> Deserializer<'de> {
    fn deserialize_integer<V>(&self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.input {
            SExp::Integer(i) => visitor.visit_i64(i.into()),
            o => Err(Error::ExpectedInteger(format!("{:?}", o))),
        }
    }

    fn deserialize_rational<V>(&self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.input {
            SExp::Rational(i) => visitor.visit_f64(i.into()),
            o => Err(Error::ExpectedRational(format!("{:?}", o))),
        }
    }

    fn deserialize_char<V>(&self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
        // match &self.input {
        //     SExp::Char(c) => visitor.visit_char(c.into() as char),
        //     o => Err(Error::ExpectedChar(format!("{:?}", o))),
        // }
    }

    fn deserialize_sstring<V>(&self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.input {
            SExp::String(s) => visitor.visit_string(s.into()),
            o => Err(Error::ExpectedString(format!("{:?}", o))),
        }
    }

    fn deserialize_symbol<V>(&self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.input {
            SExp::Symbol(s) => {
                let sstring = String::from(s);
                // Something to do with deref?
                visitor.visit_string((&sstring).into())
            }

            o => Err(Error::ExpectedSymbol(format!("{:?}", o))),
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::DeserializeAnyNotSupported)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::DeserializeIgnoredAnyNotSupported)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.input {
            SExp::Bool(b) => visitor.visit_bool(b.into()),
            o => Err(Error::ExpectedBoolean(format!("{:?}", o))),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_rational(visitor)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_rational(visitor)
    }
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // TODO: Need to go from a c char to a char
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_sstring(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_sstring(visitor)
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
        self.deserialize_symbol(visitor)
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
        let pair_or_end = match &self.de.input {
            SExp::Pair(p) => Ok(Some(p)),
            SExp::Null(_) => Ok(None),
            o => Err(Error::ExpectedPairOrEndOfAssocList(format!("{:?}", o))),
        }?;

        match pair_or_end {
            Some(pair) => {
                let head = pair.car();
                match head {
                    SExp::Pair(kv_pair) => {
                        let symbol = kv_pair.car();
                        let mut new_de = Deserializer { input: symbol };
                        seed.deserialize(&mut new_de).map(Some)
                    }
                    o => Err(Error::ExpectedPair(format!("{:?}", o))),
                }
            }
            None => Ok(None),
        }
    }
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let pair = match &self.de.input {
            SExp::Pair(p) => Ok(p),
            o => Err(Error::ExpectedPairOrEndOfAssocList(format!("{:?}", o))),
        }?;

        let head = pair.car();
        match head {
            SExp::Pair(kv_pair) => {
                let value = kv_pair.cdr();
                let mut new_de = Deserializer { input: value };
                let r = seed.deserialize(&mut new_de);
                // // Move the deserializer forwards
                self.de.input = pair.cdr();
                r
            }
            o => Err(Error::ExpectedPair(format!("{:?}", o))),
        }
    }
}

pub fn from_sexp<'a, T>(s: SExp<'a>) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer { input: s };
    T::deserialize(&mut deserializer)
}

mod tests {

    use crate::serde::de;
    use crate::sexp;
    use crate::sexp::{Context, Integer, Rational, SExp};
    use chibi_scheme_sys;
    use serde::Deserialize;
    use std::cmp::PartialEq;
    use std::fmt::Debug;
    use std::i32;
    use std::i64;

    #[test]
    fn test_deserialize_bool() {
        let mut assertions: Vec<(SExp, bool)> = vec![
            (SExp::from(sexp::FALSE), false),
            (SExp::from(sexp::TRUE), true),
        ];
        assert_all(&mut assertions)
    }

    #[test]
    fn test_deserialize_i64() {
        let mut assertions: Vec<(SExp, i64)> = vec![
            (SExp::from(Integer::from(1)), 1),
            (SExp::from(Integer::from(0)), 0),
            (SExp::from(Integer::from(-1)), -1),
            (SExp::from(Integer::from(i32::MAX as i64)), i32::MAX as i64),
            (
                SExp::from(Integer::from(chibi_scheme_sys::SEXP_MAX_FIXNUM)),
                chibi_scheme_sys::SEXP_MAX_FIXNUM,
            ),
        ];
        assert_all(&mut assertions)
    }

    #[test]
    fn test_deserialize_i32() {
        let mut assertions: Vec<(SExp, i32)> = vec![
            (SExp::from(Integer::from(1)), 1),
            (SExp::from(Integer::from(0)), 0),
            (SExp::from(Integer::from(-1)), -1),
            (SExp::from(Integer::from(i32::MAX as i64)), i32::MAX),
        ];
        assert_all(&mut assertions)
    }

    #[test]
    fn test_deserialize_f64() {
        let context = Context::default();
        let mut assertions: Vec<(SExp, f64)> = vec![
            (SExp::from(context.flonum(1.0)), 1.0),
            (SExp::from(context.flonum(0.0)), 0.0),
            (SExp::from(context.flonum(-1.5)), -1.5),
        ];
        assert_all(&mut assertions)
    }

    #[test]
    fn test_deserialize_f32() {
        let context = Context::default();
        let mut assertions: Vec<(SExp, f32)> = vec![
            (SExp::from(context.flonum(1.0)), 1.0),
            (SExp::from(context.flonum(0.0)), 0.0),
            (SExp::from(context.flonum(-1.5)), -1.5),
        ];
        assert_all(&mut assertions)
    }

    #[test]
    fn test_deserialize_string() {
        let context = Context::default();
        let mut assertions: Vec<(SExp, String)> = vec![
            (SExp::from(context.string("a")), "a".to_string()),
            (SExp::from(context.string("b")), "b".to_string()),
            (SExp::from(context.string("c")), "c".to_string()),
        ];
        assert_all(&mut assertions)
    }

    #[test]
    fn test_deserialize_struct() {
        let context = Context::default();
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
        assert_eq!(expected, de::from_sexp(foo).unwrap());
    }

    #[test]
    fn test_deserialize_nested_struct() {
        let context = Context::default();
        #[derive(Deserialize, PartialEq, Debug)]
        struct Foo {
            bar: bool,
            foo: i32,
            baz: f64,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        struct Bar {
            cow: bool,
            qux: Foo,
        }

        let context = Context::default();
        let bar = context
            .eval_string("'( (cow . #t) ( qux . ((foo . 3) (bar . #f) (baz . 5.5)) ) )")
            .unwrap();
        let expected = Bar {
            cow: true,
            qux: Foo {
                bar: false,
                foo: 3,
                baz: 5.5,
            },
        };
        assert_eq!(expected, de::from_sexp(bar).unwrap());
    }

    fn assert_all<'a, T>(table: &mut Vec<(SExp<'a>, T)>)
    where
        T: Deserialize<'a>,
        T: Debug,
        T: PartialEq,
    {
        table.drain(..).for_each(|(sexp, expected)| {
            let result = de::from_sexp::<T>(sexp);
            assert!(
                result.is_ok(),
                "Expected {:?} but got error {:?}",
                expected,
                result.err()
            );
            assert_eq!(result.unwrap(), expected);
        });
    }
}
