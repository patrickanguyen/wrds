//! Contains types for RDS Decoder.

pub mod psaf;
pub mod rt;

/// First Block in RDS Message.
/// Contains information about the PI code
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block1(pub u16);

/// Second block in RDS Message.
/// Contains information about Group Type code, Program Type code,
/// and Group Type specific information.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block2(pub u16);

/// Third block in RDS Message.
/// Contains information on either the Group specific payload or
/// PI code, depending on the Message Group Type (A or B).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block3(pub u16);

/// Fourth block in RDS Message.
/// Contains information on the Group specific payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block4(pub u16);

/// Predefined Program Type to identify different type of
/// programming by genre.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgramType(pub u8);

/// Unique code that identifies the station.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgramIdentifier(pub u16);

/// Identifies whether the station broadcasts traffic announcements.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrafficProgramCode(pub bool);

/// RDS Message Group Type
/// The B0 bit-field is added as the LSB.
#[derive(Clone, Copy, Debug, PartialEq, Eq, enumn::N)]
#[repr(u8)]
pub enum GroupType {
    ZeroA,
    ZeroB,
    OneA,
    OneB,
    TwoA,
    TwoB,
    ThreeA,
    ThreeB,
    FourA,
    FourB,
    FiveA,
    FiveB,
    SixA,
    SixB,
    SevenA,
    SevenB,
    EightA,
    EightB,
    NineA,
    NineB,
    TenA,
    TenB,
    ElevenA,
    ElevenB,
    TwelveA,
    TwelveB,
    ThirteenA,
    ThirteenB,
    FourteenA,
    FourteenB,
    FifteenA,
    FifteenB,
}

/// Payload of Radio Data System Message.
/// Contents of the payload depends on the Group Type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Payload {
    ZeroA(ZeroAPayload),
    ZeroB(ZeroBPayload),
    OneA,
    OneB,
    TwoA(TwoAPayload),
    TwoB(TwoBPayload),
    ThreeA,
    ThreeB,
    FourA,
    FourB,
    FiveA,
    FiveB,
    SixA,
    SixB,
    SevenA,
    SevenB,
    EightA,
    EightB,
    NineA,
    NineB,
    TenA,
    TenB,
    ElevenA,
    ElevenB,
    TwelveA,
    TwelveB,
    ThirteenA,
    ThirteenB,
    FourteenA,
    FourteenB,
    FifteenA,
    FifteenB,
}

/// Decoded Radio Data System Message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub pi: ProgramIdentifier,
    pub group_type: GroupType,
    pub tp: TrafficProgramCode,
    pub pty: ProgramType,
    pub payload: Payload,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZeroAPayload {
    pub ta: psaf::TrafficAnnouncementCode,
    pub ms: psaf::MusicSpeechCode,
    pub di: psaf::DecoderIdentifier,
    pub af: (psaf::AlternativeFreqCode, psaf::AlternativeFreqCode),
    pub ps_segment: psaf::PsSegment,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZeroBPayload {
    pub ta: psaf::TrafficAnnouncementCode,
    pub ms: psaf::MusicSpeechCode,
    pub di: psaf::DecoderIdentifier,
    pub pi: ProgramIdentifier,
    pub ps_segment: psaf::PsSegment,
}

/// Payload for Group Type 2A
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TwoAPayload {
    pub text_ab: rt::TextAB,
    pub text_addr_code: rt::TextAddressCode,
    pub rt_segment: (rt::RadioTextSegment, rt::RadioTextSegment),
}

/// Payload for Group Type 2B
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TwoBPayload {
    pub text_ab: rt::TextAB,
    pub text_addr_code: rt::TextAddressCode,
    pub pi: ProgramIdentifier,
    pub rt_segment: rt::RadioTextSegment,
}
