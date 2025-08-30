use core::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum RdsCharError {
    InvalidRdsChar(u8),
}

impl fmt::Display for RdsCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RdsCharError::InvalidRdsChar(c) => write!(f, "Invalid RDS character {c}"),
        }
    }
}

impl Error for RdsCharError {}

pub type Result<T> = core::result::Result<T, RdsCharError>;

pub fn to_basic_rds_char(byte: u8) -> Result<char> {
    /// Minimum value for RDS printable char
    const PRINTABLE_MIN: u8 = 0x20;
    /// Max value for RDS printable char
    const PRINTABLE_MAX: u8 = 0xFE;

    /// Value for Line Feed
    const LINE_FEED: u8 = 0x0A;
    /// Value for carriage return
    const CARRIAGE_RETURN: u8 = 0x0D;

    /// RDS value for currency sign
    const CURRENCY_SIGN: u8 = 0x24;
    /// RDS value for horizontal bar
    const HORIZONTAL_BAR: u8 = 0x5E;
    /// RDS value for Double vertical line
    const DOUBLE_VERTICAL_LINE: u8 = 0x60;
    /// RDS value for overline
    const OVERLINE: u8 = 0x7E;
    /// Unassigned value 0x7F,
    const UNASSIGNED_7F: u8 = 0x7F;

    const SMALL_A_ACUTE: u8 = 0x80;
    const SMALL_A_GRAVE: u8 = 0x81;
    const SMALL_E_ACUTE: u8 = 0x82;
    const SMALL_E_GRAVE: u8 = 0x83;
    const SMALL_I_ACUTE: u8 = 0x84;
    const SMALL_I_GRAVE: u8 = 0x85;
    const SMALL_O_ACUTE: u8 = 0x86;
    const SMALL_O_GRAVE: u8 = 0x87;
    const SMALL_U_ACUTE: u8 = 0x88;
    const SMALL_U_GRAVE: u8 = 0x89;
    const CAPITAL_N_TILDE: u8 = 0x8A;
    const CAPITAL_C_CEDILLA: u8 = 0x8B;
    const CAPITAL_S_CEDILLA: u8 = 0x8C;
    const SMALL_SHARP_S: u8 = 0x8D;
    const INVERTED_EXCLAMATION: u8 = 0x8E;
    const CAPITAL_LIGATURE_IJ: u8 = 0x8F;
    const SMALL_A_CIRCUMFLEX: u8 = 0x90;
    const SMALL_A_DIAERESIS: u8 = 0x91;
    const SMALL_E_CIRCUMFLEX: u8 = 0x92;
    const SMALL_E_DIAERESIS: u8 = 0x93;
    const SMALL_I_CIRCUMFLEX: u8 = 0x94;
    const SMALL_I_DIAERESIS: u8 = 0x95;
    const SMALL_O_CIRCUMFLEX: u8 = 0x96;
    const SMALL_O_DIAERESIS: u8 = 0x97;
    const SMALL_U_CIRCUMFLEX: u8 = 0x98;
    const SMALL_U_DIAERESIS: u8 = 0x99;
    const SMALL_N_TILDE: u8 = 0x9A;
    const SMALL_C_CEDILLA: u8 = 0x9B;
    const SMALL_S_CEDILLA: u8 = 0x9C;
    const SMALL_D_BREVE: u8 = 0x9D;
    const SMALL_DOTLESS_I: u8 = 0x9E;
    const SMALL_LIGATURE_IJ: u8 = 0x9F;
    const FEMININE_ORDINAL_INDICATOR: u8 = 0xA0;
    const GREEK_SMALL_ALPHA: u8 = 0xA1;
    const COPYRIGHT_SIGN: u8 = 0xA2;
    const PER_THOUSAND_SIGN: u8 = 0xA3;
    const CAPITAL_G_BREVE: u8 = 0xA4;
    const SMALL_E_CARON: u8 = 0xA5;
    const SMALL_N_CARON: u8 = 0xA6;
    const SMALL_O_DOUBLE_ACUTE: u8 = 0xA7;
    const GREEK_SMALL_PI: u8 = 0xA8;
    const EURO_SIGN: u8 = 0xA9;
    const POUND_SIGN: u8 = 0xAA;
    const DOLLAR_SIGN: u8 = 0xAB;
    const LEFTWARD_ARROW: u8 = 0xAC;
    const UPWARD_ARROW: u8 = 0xAD;
    const RIGHTWARD_ARROW: u8 = 0xAE;
    const DOWNWARD_ARROW: u8 = 0xAF;
    const MASCULINE_ORDINAL_INDICATOR: u8 = 0xB0;
    const SUPERSCRIPT_ONE: u8 = 0xB1;
    const SUPERSCRIPT_TWO: u8 = 0xB2;
    const SUPERSCRIPT_THREE: u8 = 0xB3;
    const PLUS_MINUS_SIGN: u8 = 0xB4;

    match byte {
        LINE_FEED | CARRIAGE_RETURN => Ok(char::from(byte)),
        CURRENCY_SIGN => Ok('¤'),
        HORIZONTAL_BAR => Ok('―'),
        DOUBLE_VERTICAL_LINE => Ok('║'),
        OVERLINE => Ok('¯'),
        UNASSIGNED_7F => Err(RdsCharError::InvalidRdsChar(byte)),
        SMALL_A_ACUTE => Ok('á'),
        SMALL_A_GRAVE => Ok('à'),
        SMALL_E_ACUTE => Ok('é'),
        SMALL_E_GRAVE => Ok('è'),
        SMALL_I_ACUTE => Ok('í'),
        SMALL_I_GRAVE => Ok('ì'),
        SMALL_O_ACUTE => Ok('ó'),
        SMALL_O_GRAVE => Ok('ò'),
        SMALL_U_ACUTE => Ok('ú'),
        SMALL_U_GRAVE => Ok('ù'),
        CAPITAL_N_TILDE => Ok('Ñ'),
        CAPITAL_C_CEDILLA => Ok('Ç'),
        CAPITAL_S_CEDILLA => Ok('Ş'),
        SMALL_SHARP_S => Ok('ß'),
        INVERTED_EXCLAMATION => Ok('¡'),
        CAPITAL_LIGATURE_IJ => Ok('Ĳ'),
        SMALL_A_CIRCUMFLEX => Ok('â'),
        SMALL_A_DIAERESIS => Ok('ä'),
        SMALL_E_CIRCUMFLEX => Ok('ê'),
        SMALL_E_DIAERESIS => Ok('ë'),
        SMALL_I_CIRCUMFLEX => Ok('î'),
        SMALL_I_DIAERESIS => Ok('ï'),
        SMALL_O_CIRCUMFLEX => Ok('ô'),
        SMALL_O_DIAERESIS => Ok('ö'),
        SMALL_U_CIRCUMFLEX => Ok('û'),
        SMALL_U_DIAERESIS => Ok('ü'),
        SMALL_N_TILDE => Ok('ñ'),
        SMALL_C_CEDILLA => Ok('ç'),
        SMALL_S_CEDILLA => Ok('ş'),
        SMALL_D_BREVE => Ok('ğ'),
        SMALL_DOTLESS_I => Ok('ı'),
        SMALL_LIGATURE_IJ => Ok('ĳ'),
        FEMININE_ORDINAL_INDICATOR => Ok('ª'),
        GREEK_SMALL_ALPHA => Ok('α'),
        COPYRIGHT_SIGN => Ok('©'),
        PER_THOUSAND_SIGN => Ok('‰'),
        CAPITAL_G_BREVE => Ok('Ğ'),
        SMALL_E_CARON => Ok('ĕ'),
        SMALL_N_CARON => Ok('ň'),
        SMALL_O_DOUBLE_ACUTE => Ok('ő'),
        GREEK_SMALL_PI => Ok('π'),
        EURO_SIGN => Ok('€'),
        POUND_SIGN => Ok('₤'),
        DOLLAR_SIGN => Ok('$'),
        LEFTWARD_ARROW => Ok('←'),
        UPWARD_ARROW => Ok('↑'),
        RIGHTWARD_ARROW => Ok('→'),
        DOWNWARD_ARROW => Ok('↓'),
        MASCULINE_ORDINAL_INDICATOR => Ok('º'),
        SUPERSCRIPT_ONE => Ok('¹'),
        SUPERSCRIPT_TWO => Ok('²'),
        SUPERSCRIPT_THREE => Ok('³'),
        PLUS_MINUS_SIGN => Ok('±'),
        PRINTABLE_MIN..PRINTABLE_MAX => Ok(char::from(byte)),
        _ => Err(RdsCharError::InvalidRdsChar(byte)),
    }
}
