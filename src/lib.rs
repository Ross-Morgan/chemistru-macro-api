#[macro_export]
macro_rules! element {
    ($mass:literal) => {
        $crate::elements::PROTON_NUMBER_MAP.get(&$mass).unwrap()
    };
}

pub mod elements {
    constants::elements_consts!();
    map::element_map!();
}

pub mod prelude {
    pub use super::element;
    pub use super::elements::*;
}
