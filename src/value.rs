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

