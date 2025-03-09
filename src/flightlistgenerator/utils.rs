use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use strict_yaml_rust::StrictYaml;
use time::{Duration, PrimitiveDateTime};

use super::extracttime::extract_date;
use super::extracttime::extract_duration;

fn mandatory_key<'a>(yaml: &'a StrictYaml, key: &str) -> Result<&'a str> {
    if let StrictYaml::String(s) = &yaml[key] {
        Ok(s)
    } else {
        bail!("key [{}] not found in yaml element :\n{:?}", key, yaml);
    }
}

pub fn mandatory_datetime(yaml: &StrictYaml, key: &str) -> Result<PrimitiveDateTime> {
    let raw_datetime = mandatory_key(yaml, key)?;
    let datetime = extract_date(raw_datetime)
        .with_context(|| format!("In field [{}] : {}", key, raw_datetime))?;
    Ok(datetime)
}

pub fn optional_string<'a>(yaml: &'a StrictYaml, key: &str) -> Option<&'a str> {
    match &yaml[key] {
        StrictYaml::String(s) => Some(s),
        _ => None,
    } // match
}

pub fn mandatory_string<'a>(yaml: &'a StrictYaml, key: &str) -> Result<&'a str> {
    match &yaml[key] {
        StrictYaml::String(s) => Ok(s),
        _ => {
            bail!(format!("mandatory key {} not present", key));
        }
    } // match
}

/// Fetch a duration, returns error if duration not properly formatted
pub fn optional_duration(yaml: &StrictYaml, key: &str) -> Result<Option<Duration>> {
    //
    match optional_string(yaml, key) {
        Some(raw_duration) => {
            let duration =
                extract_duration(&raw_duration).with_context(|| format!("in field : [{}]", key))?;
            Ok(Some(duration))
        }
        None => Ok(None),
    }
}

pub fn optional_u8(yaml: &StrictYaml, key: &str) -> Result<Option<u8>> {
    match optional_string(yaml, key) {
        Some(raw_number) => {
            let number = raw_number
                .parse::<u8>()
                .with_context(|| format!("in field : [{}]", key))?;
            Ok(Some(number))
        }
        None => Ok(None),
    }
}

pub fn optional_bool(yaml: &StrictYaml, key: &str) -> Result<Option<bool>> {
    match optional_string(yaml, key) {
        Some(raw_boolean) => {
            let boolean = raw_boolean
                .parse::<bool>()
                .with_context(|| format!("in field : [{}]", key))?;
            Ok(Some(boolean))
        }
        None => Ok(None),
    }
}

// pub fn optional_f32(yaml: &StrictYaml, key: &str) -> Result<Option<f32>> {
//     match optional_string(yaml, key) {
//         Some(raw_number) => {
//             let number = raw_number
//                 .parse::<f32>()
//                 .with_context(|| format!("in field : [{}]", key))?;
//             Ok(Some(number))
//         }
//         None => Ok(None),
//     }
// }

// fn validate_airport(
//     apt_iata: &Option<String>,
//     apt_icao: &Option<String>,
//     apt_name: &Option<String>,
// ) -> Result<()> {
//     if apt_iata == &None && apt_icao == &None && apt_name == &None {
//         bail!("Airport arrival or departure missing.\nNo IATA, ICAO or name.");
//     }
//     Ok(())
// }
