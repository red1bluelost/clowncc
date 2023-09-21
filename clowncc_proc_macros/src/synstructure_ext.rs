use crate::syn_ext::FieldsExt;

use proc_macro2::TokenStream;
use synstructure::VariantInfo;

pub(crate) trait VariantInfoExt {
    fn is_unit(&self) -> bool;
    fn construct_unit(&self) -> TokenStream;
}

impl VariantInfoExt for VariantInfo<'_> {
    fn is_unit(&self) -> bool {
        self.ast().fields.is_unit()
    }

    fn construct_unit(&self) -> TokenStream {
        self.construct(|_, _| -> TokenStream {
            unreachable!("expected variant to be a unit")
        })
    }
}
