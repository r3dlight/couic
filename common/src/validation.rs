use crate::CompositeError;

/// Trait for validation:
/// 1. Input is deserialized with primitive types (serde never fails on content)
/// 2. Input is converted to domain type, accumulating all validation errors
pub trait ValidateFrom: Sized {
    /// The type to deserialize from.
    type Input: serde::de::DeserializeOwned;

    fn validate_from(input: Self::Input) -> Result<Self, CompositeError>;
}
