use serde::Deserialize;

#[derive(Deserialize)]
pub struct Checkin {
    pub student_registration: String,
}

impl Checkin {
    pub fn validate_checkin(&self) -> Result<(), &'static str> {
        if self.student_registration.len() != 9
            || !self.student_registration.chars().all(|c| c.is_ascii_digit())
        {
            return Err("invalid_student_registration");
        }

        Ok(())
    }
}