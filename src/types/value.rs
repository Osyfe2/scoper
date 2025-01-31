use super::BaseInfo;


pub(crate) struct CounterData
{
    pub(crate) base: BaseInfo,
    pub(crate) value: Value,
}

#[derive(Clone, Copy)]
pub enum Value
{
    UInt(u64),
    IInt(i64),
    Float(f64),
}

macro_rules! FromForValue {
    ($version: ident, $( $typ: ty ),+) => {
        $(
            impl From<$typ> for Value
            {
                fn from(value: $typ) -> Self {
                    Value::$version(value.into())
                }
            }
        )+
    };
}

FromForValue!(Float, f64, f32);
FromForValue!(UInt, u64, u32, u16, u8);
FromForValue!(IInt, i64, i32, i16, i8);

impl From<usize> for Value
{
    fn from(value: usize) -> Self {
        Value::UInt(value as u64)
    }
}

impl From<isize> for Value
{
    fn from(value: isize) -> Self {
        Value::IInt(value as i64)
    }
}