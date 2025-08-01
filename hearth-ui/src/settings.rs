use dioxus::prelude::*;
use hearth_core::SettingsManager;

pub fn provide_settings_context(manager: SettingsManager) {
    use_context_provider(|| Signal::new(manager));
}

pub fn use_settings() -> Signal<SettingsManager> {
    use_context()
}