pub mod conditions;
pub mod elements;
pub mod includes;
pub mod loops;
pub mod macros;
pub mod properties;

use crate::error::XacroError;
use xmltree::Element;

pub trait FeatureProcessor {
    fn process(
        &self,
        xml: Element,
    ) -> Result<Element, XacroError>;
}
