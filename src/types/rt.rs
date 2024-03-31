/// Flag used to clear the screen if a change occurs
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextAB(pub bool);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextAddressCode(pub u8);

/// Partial Segment of RadioText
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioTextSegment(pub [char; 2]);
