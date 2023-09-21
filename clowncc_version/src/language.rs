macro_rules! implement {
    ($([$id_snake:ident, $name_str:expr]),* $(,)?) => {
        $crate::common_macros::define_info_enum!{
            Language: $([$id_snake, $name_str]),*,
        }
    }
}
pub(super) use implement;
