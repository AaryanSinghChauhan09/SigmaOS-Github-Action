// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-github-action: github action component model.
//!
//! Real implementation: actions, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
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
pub enum GithubActionError {
    EmptyActionName,
    DuplicateAction(String),
    EmptyEventSubject,
}

impl fmt::Display for GithubActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyActionName => write!(f, "action name must not be empty"),
            Self::DuplicateAction(id) => write!(f, "duplicate action {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for GithubActionError {}

pub struct GithubAction {
    pub name: String,
    actions: BTreeMap<String, Action>,
    events: BTreeMap<String, Event>,
}

impl GithubAction {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            actions: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_action(&mut self, item: Action) -> Result<(), GithubActionError> {
        if item.name.trim().is_empty() {
            return Err(GithubActionError::EmptyActionName);
        }
        let id = item.id.clone();
        if self.actions.contains_key(&id) {
            return Err(GithubActionError::DuplicateAction(id.clone()));
        }
        self.actions.insert(id, item);
        Ok(())
    }

    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    pub fn get_action(&self, id: &str) -> Option<&Action> {
        self.actions.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), GithubActionError> {
        if event.subject.trim().is_empty() {
            return Err(GithubActionError::EmptyEventSubject);
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

    fn sample_action(id: &str, name: &str) -> Action {
        Action {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = GithubAction::new("GithubAction");
        assert_eq!(m.name, "GithubAction");
        assert_eq!(m.action_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_action() {
        let mut m = GithubAction::new("GithubAction");
        m.add_action(sample_action("a1", "Alpha")).unwrap();
        assert_eq!(m.action_count(), 1);
        assert!(m.get_action("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = GithubAction::new("GithubAction");
        assert!(m.add_action(sample_action("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = GithubAction::new("GithubAction");
        m.add_action(sample_action("x", "One")).unwrap();
        assert!(m.add_action(sample_action("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = GithubAction::new("GithubAction");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
