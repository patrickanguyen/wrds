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
    const CAPITAL_I_DOT_ABOVE: u8 = 0xB5;
    const SMALL_N_ACUTE: u8 = 0xB6;
    const SMALL_U_DOUBLE_ACUTE: u8 = 0xB7;
    const MIKRO_SIGN: u8 = 0xB8;
    const INVERTED_QUESTION_MARK: u8 = 0xB9;
    const DIVISION_SIGN: u8 = 0xBA;
    const DEGREE_SIGN: u8 = 0xBB;
    const FRACTION_QUARTER: u8 = 0xBC;
    const FRACTION_HALF: u8 = 0xBD;
    const FRACTION_THREE_QUARTERS: u8 = 0xBE;
    const SECTION_SIGN: u8 = 0xBF;
    const CAPTIAL_A_ACUTE: u8 = 0xC0;
    const CAPITAL_A_GRAVE: u8 = 0xC1;
    const CAPITAL_E_ACUTE: u8 = 0xC2;
    const CAPITAL_E_GRAVE: u8 = 0xC3;
    const CAPITAL_I_ACUTE: u8 = 0xC4;
    const CAPITAL_I_GRAVE: u8 = 0xC5;
    const CAPITAL_O_ACUTE: u8 = 0xC6;
    const CAPITAL_O_GRAVE: u8 = 0xC7;
    const CAPITAL_U_ACUTE: u8 = 0xC8;
    const CAPITAL_U_GRAVE: u8 = 0xC9;
    const CAPITAL_R_CARON: u8 = 0xCA;
    const CAPITAL_C_CARON: u8 = 0xCB;
    const CAPITAL_S_CARON: u8 = 0xCC;
    const CAPITAL_Z_CARON: u8 = 0xCD;
    const CAPITAL_ETH: u8 = 0xCE;
    const CAPITAL_L_MIDDLE_DOT: u8 = 0xCF;
    const CAPITAL_A_CIRCUMFLEX: u8 = 0xD0;
    const CAPITAL_A_DIAERESIS: u8 = 0xD1;
    const CAPITAL_E_CIRCUMFLEX: u8 = 0xD2;
    const CAPITAL_A_DIAERESIS_2: u8 = 0xD3;
    const CAPITAL_I_CIRCUMFLEX: u8 = 0xD4;
    const CAPITAL_I_DIAERESIS: u8 = 0xD5;
    const CAPITAL_O_CIRCUMFLEX: u8 = 0xD6;
    const CAPITAL_O_DIAERESIS: u8 = 0xD7;
    const CAPITAL_U_CIRCUMFLEX: u8 = 0xD8;
    const CAPITAL_U_DIAERESIS: u8 = 0xD9;
    const SMALL_R_CARON: u8 = 0xDA;
    const SMALL_C_CARON: u8 = 0xDB;
    const SMALL_S_CARON: u8 = 0xDC;
    const SMALL_Z_CARON: u8 = 0xDD;
    const SMALL_D_STROKE: u8 = 0xDE;
    const SMALL_L_MIDDLE_DOT: u8 = 0xDF;
    const CAPITAL_A_TILDE: u8 = 0xE0;
    const CAPITAL_A_RING: u8 = 0xE1;
    const CAPITAL_AE: u8 = 0xE2;
    const CAPITAL_LIGATURE_OE: u8 = 0xE3;
    const SMALL_Y_CIRCUMFLEX: u8 = 0xE4;
    const CAPITAL_Y_ACUTE: u8 = 0xE5;
    const CAPITAL_O_TILDE: u8 = 0xE6;
    const CAPITAL_O_STROKE: u8 = 0xE7;
    const CAPITAL_THORN: u8 = 0xE8;
    const CAPITAL_ENG: u8 = 0xE9;
    const CAPITAL_R_ACUTE: u8 = 0xEA;
    const SMALL_C_ACUTE: u8 = 0xEB;
    const CAPITAL_S_ACUTE: u8 = 0xEC;
    const CAPITAL_Z_ACUTE: u8 = 0xED;
    const CAPITAL_T_STROKE: u8 = 0xEE;
    const SMALL_ETH: u8 = 0xEF;
    const SMALL_A_TILDE: u8 = 0xF0;
    const SMALL_A_RING: u8 = 0xF1;
    const SMALL_AE: u8 = 0xF2;
    const SMALL_LIGATURE_OE: u8 = 0xF3;
    const SMALL_W_CIRCUMFLEX: u8 = 0xF4;
    const SMALL_Y_ACUTE: u8 = 0xF5;
    const SMALL_O_TILDE: u8 = 0xF6;
    const SMALL_O_STROKE: u8 = 0xF7;
    const SMALL_THORN: u8 = 0xF8;
    const SMALL_ENG: u8 = 0xF9;
    const SMALL_R_ACUTE: u8 = 0xFA;
    const SMALL_C_ACUTE_2: u8 = 0xFB;
    const SMALL_S_ACUTE: u8 = 0xFC;
    const SMALL_Z_ACUTE: u8 = 0xFD;
    const SMALL_T_STROKE: u8 = 0xFE;

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
        CAPITAL_I_DOT_ABOVE => Ok('İ'),
        SMALL_N_ACUTE => Ok('ń'),
        SMALL_U_DOUBLE_ACUTE => Ok('ű'),
        MIKRO_SIGN => Ok('µ'),
        INVERTED_QUESTION_MARK => Ok('¿'),
        DIVISION_SIGN => Ok('÷'),
        DEGREE_SIGN => Ok('º'),
        FRACTION_QUARTER => Ok('¼'),
        FRACTION_HALF => Ok('½'),
        FRACTION_THREE_QUARTERS => Ok('¾'),
        SECTION_SIGN => Ok('§'),
        CAPTIAL_A_ACUTE => Ok('Á'),
        CAPITAL_A_GRAVE => Ok('À'),
        CAPITAL_E_ACUTE => Ok('É'),
        CAPITAL_E_GRAVE => Ok('È'),
        CAPITAL_I_ACUTE => Ok('Í'),
        CAPITAL_I_GRAVE => Ok('Ì'),
        CAPITAL_O_ACUTE => Ok('Ó'),
        CAPITAL_O_GRAVE => Ok('Ò'),
        CAPITAL_U_ACUTE => Ok('Ú'),
        CAPITAL_U_GRAVE => Ok('Ù'),
        CAPITAL_R_CARON => Ok('Ř'),
        CAPITAL_C_CARON => Ok('Č'),
        CAPITAL_S_CARON => Ok('Š'),
        CAPITAL_Z_CARON => Ok('Ž'),
        CAPITAL_ETH => Ok('Ð'),
        CAPITAL_L_MIDDLE_DOT => Ok('Ŀ'),
        CAPITAL_A_CIRCUMFLEX => Ok('Â'),
        CAPITAL_A_DIAERESIS => Ok('Ä'),
        CAPITAL_E_CIRCUMFLEX => Ok('Ê'),
        CAPITAL_A_DIAERESIS_2 => Ok('Ë'),
        CAPITAL_I_CIRCUMFLEX => Ok('Î'),
        CAPITAL_I_DIAERESIS => Ok('Ï'),
        CAPITAL_O_CIRCUMFLEX => Ok('Ô'),
        CAPITAL_O_DIAERESIS => Ok('Ö'),
        CAPITAL_U_CIRCUMFLEX => Ok('Û'),
        CAPITAL_U_DIAERESIS => Ok('Ü'),
        SMALL_R_CARON => Ok('ř'),
        SMALL_C_CARON => Ok('č'),
        SMALL_S_CARON => Ok('š'),
        SMALL_Z_CARON => Ok('ž'),
        SMALL_D_STROKE => Ok('đ'),
        SMALL_L_MIDDLE_DOT => Ok('ŀ'),
        CAPITAL_A_TILDE => Ok('Ã'),
        CAPITAL_A_RING => Ok('Å'),
        CAPITAL_AE => Ok('Æ'),
        CAPITAL_LIGATURE_OE => Ok('Œ'),
        SMALL_Y_CIRCUMFLEX => Ok('ŷ'),
        CAPITAL_Y_ACUTE => Ok('Ý'),
        CAPITAL_O_TILDE => Ok('Õ'),
        CAPITAL_O_STROKE => Ok('Ø'),
        CAPITAL_THORN => Ok('Þ'),
        CAPITAL_ENG => Ok('Ŋ'),
        CAPITAL_R_ACUTE => Ok('Ŕ'),
        SMALL_C_ACUTE => Ok('Ć'),
        CAPITAL_S_ACUTE => Ok('Ś'),
        CAPITAL_Z_ACUTE => Ok('Ź'),
        CAPITAL_T_STROKE => Ok('Ŧ'),
        SMALL_ETH => Ok('ð'),
        SMALL_A_TILDE => Ok('ã'),
        SMALL_A_RING => Ok('å'),
        SMALL_AE => Ok('æ'),
        SMALL_LIGATURE_OE => Ok('œ'),
        SMALL_W_CIRCUMFLEX => Ok('ŵ'),
        SMALL_Y_ACUTE => Ok('ý'),
        SMALL_O_TILDE => Ok('õ'),
        SMALL_O_STROKE => Ok('ø'),
        SMALL_THORN => Ok('þ'),
        SMALL_ENG => Ok('ŋ'),
        SMALL_R_ACUTE => Ok('ŕ'),
        SMALL_C_ACUTE_2 => Ok('ć'),
        SMALL_S_ACUTE => Ok('ś'),
        SMALL_Z_ACUTE => Ok('ź'),
        SMALL_T_STROKE => Ok('ŧ'),
        PRINTABLE_MIN..=PRINTABLE_MAX => Ok(char::from(byte)),
        _ => Err(RdsCharError::InvalidRdsChar(byte)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_printable_range() {
        // Test lower and upper bounds of printable ASCII
        assert_eq!(to_basic_rds_char(0x20), Ok(' '));
        assert_eq!(to_basic_rds_char(0x7E), Ok('¯')); // OVERLINE
        assert_eq!(to_basic_rds_char(0x21), Ok('!'));
        assert_eq!(to_basic_rds_char(0x7D), Ok('}'));
        assert_eq!(to_basic_rds_char(0xFE), Ok('ŧ')); // SMALL_T_STROKE
    }

    #[test]
    fn test_control_characters() {
        assert_eq!(to_basic_rds_char(0x0A), Ok('\n'));
        assert_eq!(to_basic_rds_char(0x0D), Ok('\r'));
    }

    #[test]
    fn test_invalid_characters() {
        // Below printable range
        assert!(matches!(
            to_basic_rds_char(0x00),
            Err(RdsCharError::InvalidRdsChar(0x00))
        ));
        // Unassigned 0x7F
        assert!(matches!(
            to_basic_rds_char(0x7F),
            Err(RdsCharError::InvalidRdsChar(0x7F))
        ));
        // Above defined range
        assert!(matches!(
            to_basic_rds_char(0xFF),
            Err(RdsCharError::InvalidRdsChar(0xFF))
        ));
    }

    #[test]
    fn test_special_mappings() {
        assert_eq!(to_basic_rds_char(0x24), Ok('¤')); // CURRENCY_SIGN
        assert_eq!(to_basic_rds_char(0x5E), Ok('―')); // HORIZONTAL_BAR
        assert_eq!(to_basic_rds_char(0x60), Ok('║')); // DOUBLE_VERTICAL_LINE
        assert_eq!(to_basic_rds_char(0x7E), Ok('¯')); // OVERLINE
    }

    #[test]
    fn test_extended_rds_chars() {
        // Test a few extended mappings
        assert_eq!(to_basic_rds_char(0xF1), Ok('å'));
        assert_eq!(to_basic_rds_char(0xF2), Ok('æ'));
        assert_eq!(to_basic_rds_char(0xF3), Ok('œ'));
        assert_eq!(to_basic_rds_char(0xF4), Ok('ŵ'));
        assert_eq!(to_basic_rds_char(0xF5), Ok('ý'));
        assert_eq!(to_basic_rds_char(0xF6), Ok('õ'));
        assert_eq!(to_basic_rds_char(0xF7), Ok('ø'));
        assert_eq!(to_basic_rds_char(0xF8), Ok('þ'));
        assert_eq!(to_basic_rds_char(0xF9), Ok('ŋ'));
        assert_eq!(to_basic_rds_char(0xFA), Ok('ŕ'));
        assert_eq!(to_basic_rds_char(0xFB), Ok('ć'));
        assert_eq!(to_basic_rds_char(0xFC), Ok('ś'));
        assert_eq!(to_basic_rds_char(0xFD), Ok('ź'));
        assert_eq!(to_basic_rds_char(0xFE), Ok('ŧ'));
    }
}
