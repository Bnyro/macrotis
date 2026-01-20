/// Create a method {field_name}_default() -> {field_type} which returns the default value
/// of the field from the [Default::default] implementation of the struct.
///
/// Example usage:
///
/// ```rs
/// struct Foo {
///     a: usize,
/// }
///
/// impl Default for Foo {
///     fn default() -> Self {
///         Self { a: 24 }
///     }
/// }
///
/// make_default_value_getter!(Foo, a, usize);
///
/// assert_eq!(Foo::a_default(), 24);
/// ```
macro_rules! make_default_value_getter {
    ($name:ident, $field: ident, $ret:ty) => {
        paste::paste! {
            impl $name {
                pub fn [<$field _default>]() -> $ret {
                    $name::default().$field
                }
            }
        }
    };
}

pub(crate) use make_default_value_getter;
