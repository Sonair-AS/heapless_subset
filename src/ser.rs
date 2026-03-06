use crate::{
    len_type::LenType,
    string::{StringInner, StringStorage},
    vec::{VecInner, VecStorage},
};
use serde::ser::{Serialize, SerializeSeq, Serializer};

impl<T, LenT: LenType, St: VecStorage<T>> Serialize for VecInner<T, LenT, St>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<LenT: LenType, S: StringStorage + ?Sized> Serialize for StringInner<LenT, S> {
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
    where
        SER: Serializer,
    {
        serializer.serialize_str(self)
    }
}
