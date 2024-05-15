use minijinja::{Error, State};

// Converts a camelcase string to a string with words separated by whitespaces.
// For example, "ElectricEnergyConsumption" becomes "Electric Energy Consumption".
pub(crate) fn camelcase_to_whitespaces(_state: &State, value: String) -> Result<String, Error> {
    Ok(value
        .chars()
        .enumerate()
        .fold(String::new(), |mut result, (i, c)| {
            // Insert a space if the current character is not uppercase and is not the first one.
            if c.is_uppercase() && i != 0 {
                result.push(' ');
            }
            result.push(c);
            result
        }))
}
