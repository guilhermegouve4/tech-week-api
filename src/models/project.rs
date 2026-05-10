use serde::Deserialize;

#[derive(Deserialize)]
pub struct Project {
    pub submitter_name: String,
    pub submitter_registration: String,
    pub project_name: String,
    pub description: String,
}

impl Project {
    pub fn validate_project(&self) -> Result<(), &'static str> {
        if self.submitter_name.is_empty() || self.submitter_name.len() > 255 {
            return Err("invalid_submitter_name");
        }

        if self.submitter_registration.len() != 9
            || !self.submitter_registration.chars().all(|c| c.is_ascii_digit())
        {
            return Err("invalid_ra");
        }

        if self.project_name.is_empty() || self.project_name.len() > 255 {
            return Err("invalid_project_name");
        }

        if self.description.is_empty() || self.description.len() > 500 {
            return Err("invalid_description");
        }

        Ok(())
    }
}
