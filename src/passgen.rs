const INTERAL_ERR_STRING: &'static str = "Internal error.";

#[derive(Debug)]
pub struct PasswordConfig {
    num_words: u64,
    capitalization_enabled: bool,
    punctuation_enabled: bool,
    number_enabled: bool,
}

pub fn config_from_sciter_value(data: sciter::Value) -> Result<PasswordConfig, String> {
    let num_words = data.get_item("num-words").to_int().ok_or(INTERAL_ERR_STRING)?;
    let caps_enabled = data.get_item("capitalization-enabled").to_bool().ok_or(INTERAL_ERR_STRING)?;
    let punct_enabled = data.get_item("punctuation-enabled").to_bool().ok_or(INTERAL_ERR_STRING)?;
    let num_enabled = data.get_item("number-enabled").to_bool().ok_or(INTERAL_ERR_STRING)?;
    
    Ok(PasswordConfig{
        num_words: num_words as u64,
        capitalization_enabled: caps_enabled,
        punctuation_enabled: punct_enabled,
        number_enabled: num_enabled,
    })
}
