use chemistru_macro_constants as constants;
use chemistru_macro_map as map;

#[macro_export]
macro_rules! element {
    ($mass:literal) => {
        $crate::elements::PROTON_NUMBER_MAP.get(&$mass).unwrap()
    };
}

pub mod elements {
    super::constants::elements_consts!();
    super::map::element_map!();
}

pub mod prelude {
    pub use super::element;
    pub use super::elements::*;
}
