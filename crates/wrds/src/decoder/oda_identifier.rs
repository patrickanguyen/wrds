// SPDX-FileCopyrightText: 2026 Patrick Nguyen
//
// SPDX-License-Identifier: MPL-2.0

use crate::types::{GroupType, GroupVariant};
use core::{error::Error, fmt};

/// ODA (Open Data Applications) application error
#[derive(Debug, PartialEq, Eq)]
pub enum OdaError {
    /// Error for unknown ODA application identifier (AID)
    UnknownAid(u16),
    /// Error for exceeding maximum number of tracked ODA applications
    #[allow(unused)]
    MaxAppsExceeded,
    /// Invalid group type
    InvalidGroupType(GroupType, GroupVariant),
}

impl fmt::Display for OdaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownAid(aid) => write!(f, "Unknown ODA application identifier: {aid:#04x}"),
            Self::MaxAppsExceeded => {
                write!(f, "Exceeded maximum number of tracked ODA applications")
            }
            Self::InvalidGroupType(gt, gv) => {
                write!(f, "Invalid group type and variant: {}{:?}", gt.0, gv)
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct OdaKey {
    group_type: GroupType,
    group_variant: GroupVariant,
}

#[cfg(not(feature = "heapless"))]
type AppMap = std::collections::HashMap<OdaKey, OdaApplication>;

/// Maximum number of ODA applications to track
///
/// This must be a power of 2
#[cfg(feature = "heapless")]
const MAX_ODA_APPS: usize = 4;

#[cfg(feature = "heapless")]
type AppMap = heapless::index_map::FnvIndexMap<OdaKey, OdaApplication, MAX_ODA_APPS>;

#[derive(Debug)]
pub struct OdaIdentifier {
    app_map: AppMap,
}

impl OdaIdentifier {
    /// Create a new ODA identifier
    pub fn new() -> Self {
        let app_map = AppMap::new();
        Self { app_map }
    }

    /// Add a new ODA application with the given group type, variant, and AID
    ///
    /// # Errors
    /// Returns an error if invalid group type or if the maximum number of applications is exceeded
    pub fn add_new_app(
        &mut self,
        group_type: GroupType,
        group_variant: GroupVariant,
        app: OdaApplication,
    ) -> Result<(), OdaError> {
        if !Self::is_possible_oda_group(group_type, group_variant) {
            return Err(OdaError::InvalidGroupType(group_type, group_variant));
        }
        let key = OdaKey {
            group_type,
            group_variant,
        };
        self.insert_map(key, app)
    }

    /// Checks whether the current group type and variant is registered
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

    /// Checks whether group type and variant is a possible ODA group
    pub fn is_possible_oda_group(group_type: GroupType, group_variant: GroupVariant) -> bool {
        matches!(
            (group_type.0, group_variant),
            (1, GroupVariant::B)
                | (3, GroupVariant::B)
                | (4, GroupVariant::B)
                | (10, GroupVariant::B)
                | (5..=9, _)
                | (11..=13, _)
        )
    }

    /// Helper function to insert key and app into map.
    ///
    /// This is needed because [`heapless::index_map::FnvIndexMap`] and
    /// [`std::collections::HashMap`] have different method signatures.
    fn insert_map(&mut self, key: OdaKey, app: OdaApplication) -> Result<(), OdaError> {
        #[cfg(feature = "heapless")]
        {
            self.app_map
                .insert(key, app)
                .map_err(|_| OdaError::MaxAppsExceeded)?;
        }
        #[cfg(not(feature = "heapless"))]
        {
            self.app_map.insert(key, app);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that:
    ///   - `get_app()` will return `None` when no application has been added
    #[test]
    fn test_get_app_nothing() {
        let oda_identifier = OdaIdentifier::new();
        assert_eq!(oda_identifier.get_app(GroupType(5), GroupVariant::A), None)
    }

    /// Verifies that:
    ///   - `get_app()` will return `Some` when an application has been added
    #[test]
    fn test_get_app_some() {
        const GROUP_TYPE: GroupType = GroupType(5);
        const GROUP_VARIANT: GroupVariant = GroupVariant::A;
        const ODA_APP: OdaApplication = OdaApplication::RtPlus;

        let mut oda_identifier = OdaIdentifier::new();
        oda_identifier
            .add_new_app(GROUP_TYPE, GROUP_VARIANT, ODA_APP)
            .unwrap();

        assert_eq!(
            oda_identifier.get_app(GROUP_TYPE, GROUP_VARIANT),
            Some(ODA_APP)
        )
    }

    /// Verifies that
    ///   `is_registered` will return true if the provided group type and
    ///    variant has been registered
    #[test]
    fn test_is_registered() {
        const GROUP_TYPE: GroupType = GroupType(5);
        const GROUP_VARIANT: GroupVariant = GroupVariant::A;
        const ODA_APP: OdaApplication = OdaApplication::RtPlus;

        let mut oda_identifier = OdaIdentifier::new();
        oda_identifier
            .add_new_app(GROUP_TYPE, GROUP_VARIANT, ODA_APP)
            .unwrap();

        assert!(oda_identifier.is_registered(GROUP_TYPE, GROUP_VARIANT))
    }

    /// Verifies that:
    ///   - `is_possible_oda_group` returns true if is a possible ODA group
    #[test]
    fn is_possible_oda_group_valid() {
        const GROUP_TYPE: GroupType = GroupType(5);
        const GROUP_VARIANT: GroupVariant = GroupVariant::A;

        assert!(OdaIdentifier::is_possible_oda_group(
            GROUP_TYPE,
            GROUP_VARIANT
        ))
    }
}
