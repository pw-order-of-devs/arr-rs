/// the order of the bits packed/unpacked
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum BitOrder {
    /// standard binary representation
    Big,
    /// reversed order
    Little,
}
