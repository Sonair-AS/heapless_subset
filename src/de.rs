use crate::{len_type::LenType, String, Vec};
use core::{fmt, marker::PhantomData};
use serde::de::{self, Deserialize, Deserializer, Error, SeqAccess};

impl<'de, T, LenT: LenType, const N: usize> Deserialize<'de> for Vec<T, N, LenT>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor<'de, T, LenT: LenType, const N: usize>(PhantomData<(&'de (), T, LenT)>);

        impl<'de, T, LenT, const N: usize> serde::de::Visitor<'de> for ValueVisitor<'de, T, LenT, N>
        where
            T: Deserialize<'de>,
            LenT: LenType,
        {
            type Value = Vec<T, N, LenT>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Vec::new();

                while let Some(value) = seq.next_element()? {
                    if values.push(value).is_err() {
                        return Err(A::Error::invalid_length(values.capacity() + 1, &self))?;
                    }
                }

                Ok(values)
            }
        }
        deserializer.deserialize_seq(ValueVisitor(PhantomData))
    }
}

impl<'de, LenT: LenType, const N: usize> Deserialize<'de> for String<N, LenT> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor<'de, LenT: LenType, const N: usize>(PhantomData<(&'de (), LenT)>);

        impl<'de, LenT: LenType, const N: usize> de::Visitor<'de> for ValueVisitor<'de, LenT, N> {
            type Value = String<N, LenT>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a string no more than {} bytes long", N as u64)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut s = String::new();
                s.push_str(v)
                    .map_err(|_| E::invalid_length(v.len(), &self))?;
                Ok(s)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut s = String::new();

                s.push_str(
                    core::str::from_utf8(v)
                        .map_err(|_| E::invalid_value(de::Unexpected::Bytes(v), &self))?,
                )
                .map_err(|_| E::invalid_length(v.len(), &self))?;

                Ok(s)
            }
        }

        deserializer.deserialize_str(ValueVisitor(PhantomData))
    }
}
