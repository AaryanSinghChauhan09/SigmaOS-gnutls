// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-gnutls: gnutls component model.
//!
//! Real implementation: gnutls_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GnutlsItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GnutlsError {
    EmptyGnutlsItemName,
    DuplicateGnutlsItem(String),
    EmptyEventSubject,
}

impl fmt::Display for GnutlsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGnutlsItemName => write!(f, "gnutls_item name must not be empty"),
            Self::DuplicateGnutlsItem(id) => write!(f, "duplicate gnutls_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for GnutlsError {}

pub struct Gnutls {
    pub name: String,
    gnutls_items: BTreeMap<String, GnutlsItem>,
    events: BTreeMap<String, Event>,
}

impl Gnutls {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            gnutls_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_gnutls_item(&mut self, item: GnutlsItem) -> Result<(), GnutlsError> {
        if item.name.trim().is_empty() {
            return Err(GnutlsError::EmptyGnutlsItemName);
        }
        let id = item.id.clone();
        if self.gnutls_items.contains_key(&id) {
            return Err(GnutlsError::DuplicateGnutlsItem(id.clone()));
        }
        self.gnutls_items.insert(id, item);
        Ok(())
    }

    pub fn gnutls_item_count(&self) -> usize {
        self.gnutls_items.len()
    }

    pub fn get_gnutls_item(&self, id: &str) -> Option<&GnutlsItem> {
        self.gnutls_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), GnutlsError> {
        if event.subject.trim().is_empty() {
            return Err(GnutlsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_gnutls_item(id: &str, name: &str) -> GnutlsItem {
        GnutlsItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Gnutls::new("Gnutls");
        assert_eq!(m.name, "Gnutls");
        assert_eq!(m.gnutls_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_gnutls_item() {
        let mut m = Gnutls::new("Gnutls");
        m.add_gnutls_item(sample_gnutls_item("a1", "Alpha")).unwrap();
        assert_eq!(m.gnutls_item_count(), 1);
        assert!(m.get_gnutls_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Gnutls::new("Gnutls");
        assert!(m.add_gnutls_item(sample_gnutls_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Gnutls::new("Gnutls");
        m.add_gnutls_item(sample_gnutls_item("x", "One")).unwrap();
        assert!(m.add_gnutls_item(sample_gnutls_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Gnutls::new("Gnutls");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
