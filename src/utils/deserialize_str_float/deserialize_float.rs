use serde::{
    de::{self},
    Deserialize, Deserializer,
};
/*
 * Used to deserialize Option(string) into f32 because
 * html input type number gives strings.
 */
pub fn de_f32_from_opt_string<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Deserialize::deserialize(deserializer)?;
    match value {
        Some(value) => {
            let parsed = value.parse::<f32>();
            if parsed.is_err() {
                return Err(de::Error::custom(format!(
                    "Failed to parse {} to f32",
                    value
                )));
            }
            Ok(Some(parsed.expect("Should be valid now.")))
        }
        None => Ok(None),
    }
}

/*
 * Used to deserialize string into f32 because
 * html input type number gives strings.
 */
pub fn _de_f32_from_string<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;
    let parsed = value.parse::<f32>();
    if parsed.is_err() {
        return Err(de::Error::custom(format!(
            "Failed to parse {} to f32",
            value
        )));
    }
    Ok(parsed.expect("Should be valid now."))
}

pub fn de_f64_from_opt_string<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Deserialize::deserialize(deserializer)?;
    match value {
        Some(value) => {
            let parsed = value.parse::<f64>();
            if parsed.is_err() {
                return Err(de::Error::custom(format!(
                    "Failed to parse {} to f64",
                    value
                )));
            }
            Ok(Some(parsed.expect("Should be valid now.")))
        }
        None => Ok(None),
    }
}

pub fn de_f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;
    let parsed = value.parse::<f64>();
    if parsed.is_err() {
        return Err(de::Error::custom(format!(
            "Failed to parse {} to f64",
            value
        )));
    }
    Ok(parsed.expect("Should be valid now."))
}
