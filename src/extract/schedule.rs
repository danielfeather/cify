use std::str::FromStr;

use chrono::{NaiveDate, Weekday};
use serde::{
    de::{IntoDeserializer, Visitor},
    Deserialize,
};

use crate::error::RecordParsingError;

use super::TransactionType;

#[derive(Debug, Deserialize, Clone)]
pub enum StpIndicator {
    #[serde(rename = "C")]
    Cancellation,
    #[serde(rename = "N")]
    New,
    #[serde(rename = "O")]
    Overlay,
    #[serde(rename = "P")]
    Permanent,
}

#[derive(Debug, Clone)]
pub struct BasicSchedule {
    pub transaction_type: TransactionType,
    pub train_uid: String,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub days: Vec<Weekday>,
    // pub bank_holiday_running: bool,
    // pub train_status: String,
    // pub train_category: String,
    // pub train_identity: String,
    // pub headcode: String,
    // pub course_indicator: String,
    // pub pcc_tsc: String,
    // pub business_sector: String,
    // pub power_type: String,
    // pub timing_load: String,
    // pub speed: String,
    // pub operating_chars: String,
    // pub train_class: String,
    // pub sleepers: String,
    // pub reservations: bool,
    // pub connector_indicator: bool,
    // pub catering_code: String,
    // pub service_branding: String,
    pub stp_indicator: StpIndicator,
}

impl FromStr for BasicSchedule {
    type Err = RecordParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(RecordParsingError::NonAscii);
        }

        let stripped = match s.len() {
            78 => s,
            80 => {
                if &s[0..2] != "BS" {
                    return Err(RecordParsingError::UnexpectedRecordIdentity("BS"));
                }

                &s[2..]
            }
            _ => return Err(RecordParsingError::InvalidLength),
        };

        let train_uid = stripped[1..7].to_string();

        let from = NaiveDate::parse_from_str(&stripped[7..13], "%d%m%y").map_err(|_| {
            RecordParsingError::InvalidField("Date Runs From", stripped[7..13].to_string())
        })?;

        let to = NaiveDate::parse_from_str(&stripped[13..19], "%d%m%y").map_err(|_| {
            RecordParsingError::InvalidField("Date Runs To", stripped[13..19].to_string())
        })?;

        let transaction_type = TransactionType::deserialize(stripped[0..1].into_deserializer())
            .map_err(|_: serde::de::value::Error| {
                RecordParsingError::InvalidField("Transaction Type", stripped[0..1].to_string())
            })?;

        let days: Vec<Weekday> = stripped[19..26]
            .chars()
            .enumerate()
            .filter_map(|(i, value)| {
                if value == '0' {
                    return None;
                } else {
                    return Weekday::try_from(i as u8).ok();
                }
            })
            .collect();

        Ok(BasicSchedule {
            transaction_type,
            train_uid,
            from,
            to,
            days,
            // bank_holiday_running: todo!(),
            // train_status: todo!(),
            // train_category: todo!(),
            // train_identity: todo!(),
            // headcode: todo!(),
            // course_indicator: todo!(),
            // pcc_tsc: todo!(),
            // business_sector: todo!(),
            // power_type: todo!(),
            // timing_load: todo!(),
            // speed: todo!(),
            // operating_chars: todo!(),
            // train_class: todo!(),
            // sleepers: todo!(),
            // reservations: todo!(),
            // connector_indicator: todo!(),
            // catering_code: v[68..72].to_string(),
            // service_branding: v[72..76].to_string(),
            stp_indicator: StpIndicator::deserialize(stripped[77..].into_deserializer()).map_err(
                |_: serde::de::value::Error| {
                    RecordParsingError::InvalidField("STP Indicator", stripped[77..].to_string())
                },
            )?,
        })
    }
}

impl<'de> Deserialize<'de> for BasicSchedule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BasicScheduleVisitor;

        impl<'de> Visitor<'de> for BasicScheduleVisitor {
            type Value = BasicSchedule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct BasicSchedule")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                BasicSchedule::from_str(v).map_err(|e| E::custom(e))
            }
        }

        deserializer.deserialize_str(BasicScheduleVisitor)
    }
}
