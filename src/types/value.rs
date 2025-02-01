use super::BaseInfo;


pub(crate) struct CounterData
{
    pub(crate) base: BaseInfo,
    pub(crate) value: Value,
}

pub enum Value
{
    UInt(u64),
    IInt(i64),
    Float(f64),
    //Other(Box<String>)
}

macro_rules! FromForValue {
    ($version: ident, $typ1:ty, $( $typ:ty),+) => {
        impl From<$typ1> for Value
        {
            fn from(value: $typ1) -> Self {
                Value::$version(value)
            }
        }
        $(
            impl From<$typ> for Value
            {
                fn from(value: $typ) -> Self {
                    Value::$version(value as $typ1)
                }
            }
        )+
    };
}

FromForValue!(Float, f64, f32);
FromForValue!(UInt, u64, u32, u16, u8, usize);
FromForValue!(IInt, i64, i32, i16, i8, isize);