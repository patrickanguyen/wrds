use wrds::{
    decoder::Decoder,
    types::{Message, Metadata, ProgrammeIdentifier, ProgrammeType, TrafficProgram},
};

/// Verifies that:
///   - Decoder will do nothing if empty RDS message is decoded.
#[test]
fn empty_message() {
    let blocks = Message::new(None, None, None, None);
    let mut decoder = Decoder::default();
    let metadata = decoder.decode(&blocks);
    assert_eq!(*metadata, Metadata::default())
}

/// Verifies that:
///   - Decoder will use PI from Block 1 if provided.
#[test]
fn block1_pi() {
    const EXPECTED_PI: u16 = 0x1234;

    let blocks = Message::new(Some(EXPECTED_PI), None, None, None);
    let mut decoder = Decoder::default();
    let metadata = decoder.decode(&blocks);
    assert_eq!(
        *metadata,
        Metadata {
            pi: Some(ProgrammeIdentifier(EXPECTED_PI)),
            ..Default::default()
        }
    )
}

/// Verifies that:
///   - Decoder will use the PI from Block 3 if Block 1 is not provided and if Group Variant is Type B.
#[test]
fn block3_pi() {
    const EXPECTED_PI: u16 = 0x5678;

    let blocks = Message::new(None, Some(0xBEEF), Some(EXPECTED_PI), None);
    let mut decoder = Decoder::default();
    let metadata = decoder.decode(&blocks);
    assert_eq!(
        *metadata,
        Metadata {
            pty: Some(ProgrammeType(0x17)),
            tp: Some(TrafficProgram(true)),
            pi: Some(ProgrammeIdentifier(EXPECTED_PI)),
            type0: None
        }
    )
}
