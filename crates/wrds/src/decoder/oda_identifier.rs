use core::{error::Error, fmt};

use heapless::index_map::FnvIndexMap;

use crate::types::{GroupType, GroupVariant};

/// ODA (Open Data Applications) application error
#[derive(Debug, PartialEq, Eq)]
pub enum OdaError {
    /// Error for unknown ODA application identifier (AID)
    UnknownAid(u16),
    /// Error for exceeding maximum number of tracked ODA applications
    MaxAppsExceeded,
}

impl fmt::Display for OdaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownAid(aid) => write!(f, "Unknown ODA application identifier: {aid:#04x}"),
            Self::MaxAppsExceeded => {
                write!(f, "Exceeded maximum number of tracked ODA applications")
            }
        }
    }
}

impl Error for OdaError {}

/// RadioText Plus application identifier (AID)
const RT_PLUS_AID: u16 = 0x4BD7;

/// ODA (Open Data Applications) application types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OdaApplication {
    /// Radio Text Plus (0x4BD7)
    RtPlus,
}

impl TryFrom<u16> for OdaApplication {
    type Error = OdaError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            RT_PLUS_AID => Ok(OdaApplication::RtPlus),
            _ => Err(OdaError::UnknownAid(value)),
        }
    }
}

/// Key for identifying ODA applications
#[derive(Debug, PartialEq, Eq, Hash)]
struct OdaKey {
    group_type: GroupType,
    group_variant: GroupVariant,
}

/// Maximum number of ODA applications to track
///
/// This must be a power of 2
const MAX_ODA_APPS: usize = 4;

#[derive(Debug)]
pub struct OdaIdentifier {
    app_map: FnvIndexMap<OdaKey, OdaApplication, MAX_ODA_APPS>,
}

impl OdaIdentifier {
    /// Create a new ODA identifier
    pub fn new() -> Self {
        Self {
            app_map: FnvIndexMap::new(),
        }
    }

    /// Add a new ODA application with the given group type, variant, and AID
    ///
    /// # Errors
    /// Returns an error if the AID is unknown or if the maximum number of applications is exceeded
    pub fn add_new_app(
        &mut self,
        group_type: GroupType,
        group_variant: GroupVariant,
        app: OdaApplication,
    ) -> Result<(), OdaError> {
        let key = OdaKey {
            group_type,
            group_variant,
        };
        self.app_map
            .insert(key, app)
            .map_err(|_| OdaError::MaxAppsExceeded)?;
        Ok(())
    }

    pub fn is_registered(&self, group_type: GroupType, group_variant: GroupVariant) -> bool {
        let key = OdaKey {
            group_type,
            group_variant,
        };
        self.app_map.contains_key(&key)
    }

    /// Get the ODA application for the given group type and variant
    ///
    /// Returns `None` if no application is found
    pub fn get_app(
        &self,
        group_type: GroupType,
        group_variant: GroupVariant,
    ) -> Option<OdaApplication> {
        let key = OdaKey {
            group_type,
            group_variant,
        };
        self.app_map.get(&key).copied()
    }
}
