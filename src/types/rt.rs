//! Types for Type 2 Group: RadioText.

/// Flag used to clear the screen if a change occurs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextAB(pub bool);

/// Position of the Text Segments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextAddressCode(pub u8);

/// Partial Segment of RadioText.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioTextSegment(pub [char; 2]);
