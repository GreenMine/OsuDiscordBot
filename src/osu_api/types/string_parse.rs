
use serde::{de, Deserializer};
use std::fmt;

pub fn parse_string_as_u64<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringParserVisitor;

    impl<'de> de::Visitor<'de> for StringParserVisitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("A valid u64 as a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<u64, E>
        where
            E: de::Error,
        {
            let num = value
                .parse()
                .map_err(|e| de::Error::custom(format!("Couldn't parse '{}' as a number: {}", value, e)))?;
            Ok(num)
        }
    }
    d.deserialize_any(StringParserVisitor)
}

pub fn parse_string_as_f32<'de, D>(d: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringParserVisitor;

    impl<'de> de::Visitor<'de> for StringParserVisitor {
        type Value = f32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("A valid f32 as a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<f32, E>
        where
            E: de::Error,
        {
            let num = value
                .parse()
                .map_err(|e| de::Error::custom(format!("Couldn't parse '{}' as a number: {}", value, e)))?;
            Ok(num)
        }
    }
    d.deserialize_any(StringParserVisitor)
}
