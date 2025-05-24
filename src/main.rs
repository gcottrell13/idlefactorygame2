use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

mod components;
mod content;
mod engine;
mod types;

use components::FactoryApp;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <Router>
                <main>
                    <Routes fallback=|| "Not found.">
                        <Route path=path!("/*any") view=FactoryApp />
                    </Routes>
                </main>
            </Router>
        }
    })
}
