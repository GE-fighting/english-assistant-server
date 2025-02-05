use crate::common::errors::ConversionError;
use crate::domain::models::unit::Unit;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use time::format_description;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitDTO {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub textbook_id: Option<i32>,
    pub sequence_number: Option<i32>,
    pub created_at: Option<String>,
    pub word_count: Option<i32>,
    pub updated_at: Option<String>,
}

impl UnitDTO {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            textbook_id: None,
            sequence_number: None,
            created_at: None,
            word_count: None,
            updated_at: None,
        }
    }
}

impl TryFrom<Unit> for UnitDTO {
    type Error = ConversionError;

    fn try_from(unit: Unit) -> Result<Self, Self::Error> {
        let format = format_description::well_known::Rfc3339;

        let created_at = match unit.created_at {
            Some(dt) => Some(dt.format(&format)?),
            None => None,
        };

        let updated_at = match unit.updated_at {
            Some(dt) => Some(dt.format(&format)?),
            None => None,
        };

        Ok(Self {
            id: unit.id,
            name: unit.name,
            textbook_id: unit.textbook_id,
            sequence_number: unit.sequence_number,
            created_at,
            word_count: unit.word_count,
            updated_at,
        })
    }
}

impl TryFrom<&Unit> for UnitDTO {
    type Error = ConversionError;

    fn try_from(unit: &Unit) -> Result<Self, Self::Error> {
        let format = format_description::well_known::Rfc3339;

        let created_at = match unit.created_at {
            Some(dt) => Some(dt.format(&format)?),
            None => None,
        };

        let updated_at = match unit.updated_at {
            Some(dt) => Some(dt.format(&format)?),
            None => None,
        };

        Ok(Self {
            id: unit.id,
            name: unit.name.clone(),
            textbook_id: unit.textbook_id,
            sequence_number: unit.sequence_number,
            created_at,
            word_count: unit.word_count,
            updated_at,
        })
    }
}

impl TryFrom<UnitDTO> for Unit {
    type Error = ConversionError;

    fn try_from(dto: UnitDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            id: dto.id,
            name: dto.name,
            textbook_id: dto.textbook_id,
            sequence_number: dto.sequence_number,
            created_at: None,
            word_count: dto.word_count,
            updated_at: None,
        })
    }
}

impl TryFrom<&UnitDTO> for Unit {
    type Error = ConversionError;

    fn try_from(dto: &UnitDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            id: dto.id,
            name: dto.name.clone(),
            textbook_id: dto.textbook_id,
            sequence_number: dto.sequence_number,
            created_at: None,
            word_count: dto.word_count,
            updated_at: None,
        })
    }
}
