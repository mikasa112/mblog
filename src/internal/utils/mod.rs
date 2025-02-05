use validator::ValidationError;

pub mod date_utils;
pub mod log_utils;

//验证id是否大于0
pub fn id_validator(value: u32) -> Result<(), ValidationError> {
    if value >= 1 {
        Ok(())
    } else {
        Err(ValidationError::new("id需要大于0"))
    }
}
