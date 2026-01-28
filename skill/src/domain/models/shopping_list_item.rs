use super::error::DomainError;

/// Maximum allowed length for an item name.
const MAX_ITEM_NAME_LENGTH: usize = 200;

/// A validated shopping list item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShoppingListItem {
    name: String,
}

impl ShoppingListItem {
    /// Creates a new ShoppingListItem from a raw name string.
    ///
    /// The name is trimmed of whitespace and validated:
    /// - Must not be empty after trimming
    /// - Must not exceed 200 characters
    ///
    /// # Errors
    /// Returns `DomainError::InvalidItemName` if validation fails.
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let name = name.into().trim().to_string();

        if name.is_empty() {
            return Err(DomainError::InvalidItemName(
                "Item name cannot be empty".to_string(),
            ));
        }

        if name.len() > MAX_ITEM_NAME_LENGTH {
            return Err(DomainError::InvalidItemName(format!(
                "Item name exceeds maximum length of {} characters",
                MAX_ITEM_NAME_LENGTH
            )));
        }

        Ok(Self { name })
    }

    /// Returns the item name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_item_with_valid_name() {
        let item = ShoppingListItem::new("Milk").unwrap();
        assert_eq!(item.name(), "Milk");
    }

    #[test]
    fn trims_whitespace_from_name() {
        let item = ShoppingListItem::new("  Bread  ").unwrap();
        assert_eq!(item.name(), "Bread");
    }

    #[test]
    fn rejects_empty_name() {
        let result = ShoppingListItem::new("");
        assert!(matches!(result, Err(DomainError::InvalidItemName(_))));
    }

    #[test]
    fn rejects_whitespace_only_name() {
        let result = ShoppingListItem::new("   ");
        assert!(matches!(result, Err(DomainError::InvalidItemName(_))));
    }

    #[test]
    fn rejects_name_exceeding_max_length() {
        let long_name = "a".repeat(201);
        let result = ShoppingListItem::new(long_name);
        assert!(matches!(result, Err(DomainError::InvalidItemName(_))));
    }

    #[test]
    fn accepts_name_at_max_length() {
        let max_name = "a".repeat(200);
        let item = ShoppingListItem::new(max_name.clone()).unwrap();
        assert_eq!(item.name(), max_name);
    }
}