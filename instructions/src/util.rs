

#[macro_export]
macro_rules! set_bits {
    ($value:expr, $set_to:expr, $($pos:expr),*) => {{
        let mut val = $value;
        $(
            if $pos < 32 {
                if $set_to==1 {
                    val |= 1 << $pos;
                } else  {
                    val &= !(1 << $pos);
                }
            }
        )*
        val
    }};
}
