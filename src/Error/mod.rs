
/// Agregation of errors
///
pub enum LogInError {
    InvalidCredentials,
    DatabaseError(String),
}
