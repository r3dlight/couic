use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Add,
    Remove,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_action_serde_lowercase() {
        let add = Action::Add;
        let remove = Action::Remove;

        assert_eq!(serde_json::to_string(&add).unwrap(), "\"add\"");
        assert_eq!(serde_json::to_string(&remove).unwrap(), "\"remove\"");

        let deserialized_add: Action = serde_json::from_str("\"add\"").unwrap();
        let deserialized_remove: Action = serde_json::from_str("\"remove\"").unwrap();
        assert_eq!(deserialized_add, Action::Add);
        assert_eq!(deserialized_remove, Action::Remove);
    }
}
