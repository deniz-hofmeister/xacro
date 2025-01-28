pub mod condition;
pub mod element;
pub mod include;
pub mod loop_;
pub mod macro_;
pub mod property;

use crate::error::XacroError;
use xmltree::Element;

pub trait FeatureProcessor {
    fn process(
        &self,
        xml: Element,
    ) -> Result<Element, XacroError>;
}
