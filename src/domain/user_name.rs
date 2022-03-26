use unicode_segmentation::UnicodeSegmentation;

#[derive(thiserror::Error, Debug)]
pub enum UserNameError {
    #[error("is empty or whitespace")]
    IsEmptyOrWhitespace,
    #[error("Is too long")]
    IsTooLong,
    #[error("Forbidden characters")]
    ForbiddenCharacters
}

#[derive(Debug)]
pub struct UserName(String);

impl UserName {
    pub fn parse(s: String) -> Result<UserName, UserNameError> {
        if s.trim().is_empty() {
            return Err(UserNameError::IsEmptyOrWhitespace);
        }

        if s.graphemes(true).count() > 255 {
            return Err(UserNameError::IsTooLong);
        }

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        if s.chars().any(|g| forbidden_characters.contains(&g)) {
            return Err(UserNameError::ForbiddenCharacters);
        }

        Ok(Self(s))
    }

}

impl AsRef<String> for UserName {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {

}