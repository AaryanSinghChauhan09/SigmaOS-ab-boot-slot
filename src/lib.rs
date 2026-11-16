// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-ab-boot-slot: ab boot slot component model.
//!
//! Real implementation: slots, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slot {
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
pub enum AbBootSlotError {
    EmptySlotName,
    DuplicateSlot(String),
    EmptyEventSubject,
}

impl fmt::Display for AbBootSlotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySlotName => write!(f, "slot name must not be empty"),
            Self::DuplicateSlot(id) => write!(f, "duplicate slot {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AbBootSlotError {}

pub struct AbBootSlot {
    pub name: String,
    slots: BTreeMap<String, Slot>,
    events: BTreeMap<String, Event>,
}

impl AbBootSlot {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            slots: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_slot(&mut self, item: Slot) -> Result<(), AbBootSlotError> {
        if item.name.trim().is_empty() {
            return Err(AbBootSlotError::EmptySlotName);
        }
        let id = item.id.clone();
        if self.slots.contains_key(&id) {
            return Err(AbBootSlotError::DuplicateSlot(id.clone()));
        }
        self.slots.insert(id, item);
        Ok(())
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    pub fn get_slot(&self, id: &str) -> Option<&Slot> {
        self.slots.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AbBootSlotError> {
        if event.subject.trim().is_empty() {
            return Err(AbBootSlotError::EmptyEventSubject);
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

    fn sample_slot(id: &str, name: &str) -> Slot {
        Slot {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AbBootSlot::new("AbBootSlot");
        assert_eq!(m.name, "AbBootSlot");
        assert_eq!(m.slot_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_slot() {
        let mut m = AbBootSlot::new("AbBootSlot");
        m.add_slot(sample_slot("a1", "Alpha")).unwrap();
        assert_eq!(m.slot_count(), 1);
        assert!(m.get_slot("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AbBootSlot::new("AbBootSlot");
        assert!(m.add_slot(sample_slot("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AbBootSlot::new("AbBootSlot");
        m.add_slot(sample_slot("x", "One")).unwrap();
        assert!(m.add_slot(sample_slot("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AbBootSlot::new("AbBootSlot");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
