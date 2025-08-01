use dioxus::prelude::*;
use hearth_core::{SettingsManager, AppSettings, Theme, BackendId, RemoteBackendConfig, LocalBackendConfig, LlmProviderConfig};

pub fn provide_settings_context(manager: SettingsManager) {
    use_context_provider(|| Signal::new(manager));
}

pub fn use_settings() -> Signal<SettingsManager> {
    use_context()
}

// Convenience hooks for common settings operations
pub fn use_theme() -> (Theme, impl Fn(Theme) + Clone) {
    let settings = use_settings();
    let theme = settings.read().get().theme.clone();
    let set_theme = move |new_theme: Theme| {
        let mut s = settings.clone();
        s.write().update_theme(new_theme);
    };
    (theme, set_theme)
}

pub fn use_backend_selection() -> (Option<BackendId>, impl Fn(Option<BackendId>) + Clone) {
    let settings = use_settings();
    let selected = settings.read().get_selected_backend().clone();
    let set_selected = move |backend_id: Option<BackendId>| {
        let mut s = settings.clone();
        s.write().set_selected_backend(backend_id);
    };
    (selected, set_selected)
}

pub fn use_remote_backends() -> (Vec<RemoteBackendConfig>, impl Fn(RemoteBackendConfig) + Clone, impl Fn(String) + Clone) {
    let settings = use_settings();
    let backends = settings.read().get().remote_backends.clone();
    
    let add_backend = {
        let settings = settings.clone();
        move |backend: RemoteBackendConfig| {
            let mut s = settings.clone();
            s.write().add_remote_backend(backend);
        }
    };
    
    let remove_backend = {
        let settings = settings.clone();
        move |id: String| {
            let mut s = settings.clone();
            s.write().remove_remote_backend(&id);
        }
    };
    
    (backends, add_backend, remove_backend)
}
