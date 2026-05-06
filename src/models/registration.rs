use serde::Deserialize;

#[derive(Deserialize)]
pub struct Registration {
    pub name: String,
    pub student_registration: u32,
    pub course_name: String,
    pub course_period: u32,
    pub coffee_break: bool,
}

impl Registration {
    pub fn validate_registration(&self) -> Result<(), &'static str> {
        if self.name.is_empty()
            || self.name.len() > 255
            || self.name.chars().any(|c| c.is_ascii_digit())
        {
            return Err("invalid_name");
        }

        if self.student_registration < 100_000_000 || self.student_registration > 999_999_999 {
            return Err("invalid_student_registration");
        }

        if self.course_name.is_empty()
            || self.course_name.len() > 255
            || self.course_name.chars().all(|c| c.is_ascii_digit())
        {
            return Err("invalid_course_name");
        }

        if self.course_period < 1 || self.course_period > 12 {
            return Err("invalid_course_period");
        }

        Ok(())
    }
}
