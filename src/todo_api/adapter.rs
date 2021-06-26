#[macro_export]
macro_rules! val {
    (B => $bval:expr) => {{
        let mut attr = AttributeValue::default();
        attr.bool = Some($bval);
        attr
    }};
    (L => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.l = Some($val);
        attr
    }};
    (S => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.s = $val;
        attr
    }};
    (M => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.m = Some($val);
        attr
    }};
}
