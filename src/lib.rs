// SPDX-License-Identifier: GPL-3.0-only
//! SigmaOS GitHub Actions registry component.
//! Part of SigmaOS multi-repository architecture

#![no_std]

/// Component identity for the action registry.
pub struct Component {
    name: &'static str,
}

impl Component {
    pub fn new() -> Self {
        Self { name: "SigmaOS-Github-Action" }
    }
}

impl Default for Component {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_creation() {
        let c = super::Component::new();
        assert_eq!(c.name, "SigmaOS-Github-Action");
    }
}
