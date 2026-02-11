//! Lattice Demo Library
//!
//! This module exposes the demo application's business logic for testing.

use leptos::prelude::*;

/// Basket panel logic and solver-backed view construction.
pub mod basket;

/// Asynchronous estimate worker integration and UI signals.
pub mod estimates;

/// Product fixture loading and product-panel UI models/components.
pub mod products;

/// Promotion fixture loading and promotion display utilities.
pub mod promotions;

/// Publish a live-region announcement by updating message text and bumping its id.
pub fn announce(live_message: RwSignal<(u64, String)>, message: String) {
    live_message.update(|(id, text)| {
        *id = id.saturating_add(1);
        *text = message;
    });
}

#[cfg(test)]
mod tests {
    use leptos::prelude::*;

    use super::*;

    // Test announce function
    #[test]
    fn test_announce_updates_message() {
        let live_message = RwSignal::new((0_u64, String::new()));

        announce(live_message, "Test message".to_string());

        let (id, message) = live_message.get_untracked();

        assert_eq!(id, 1);
        assert_eq!(message, "Test message");
    }

    #[test]
    fn test_announce_increments_id() {
        let live_message = RwSignal::new((0_u64, String::new()));

        announce(live_message, "Message 1".to_string());
        announce(live_message, "Message 2".to_string());
        announce(live_message, "Message 3".to_string());

        let (id, message) = live_message.get_untracked();

        assert_eq!(id, 3);
        assert_eq!(message, "Message 3");
    }

    #[test]
    fn test_announce_handles_empty_message() {
        let live_message = RwSignal::new((0_u64, String::new()));

        announce(live_message, String::new());

        let (id, message) = live_message.get_untracked();

        assert_eq!(id, 1);
        assert_eq!(message, "");
    }

    #[test]
    fn test_announce_handles_long_message() {
        let live_message = RwSignal::new((0_u64, String::new()));
        let long_message = "a".repeat(1000);

        announce(live_message, long_message.clone());

        let (_id, message) = live_message.get_untracked();

        assert_eq!(message, long_message);
    }

    #[test]
    fn test_announce_handles_unicode() {
        let live_message = RwSignal::new((0_u64, String::new()));

        announce(live_message, "Hello 世界 🌍".to_string());

        let (_id, message) = live_message.get_untracked();

        assert_eq!(message, "Hello 世界 🌍");
    }

    #[test]
    fn test_announce_id_overflow_handling() {
        let live_message = RwSignal::new((u64::MAX, String::new()));

        announce(live_message, "Test".to_string());

        let (id, _message) = live_message.get_untracked();

        // saturating_add should keep it at MAX
        assert_eq!(id, u64::MAX);
    }

    #[test]
    fn test_announce_preserves_previous_id() {
        let live_message = RwSignal::new((42_u64, "Old message".to_string()));

        announce(live_message, "New message".to_string());

        let (id, message) = live_message.get_untracked();

        assert_eq!(id, 43);
        assert_eq!(message, "New message");
    }
}
