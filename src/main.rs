use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

mod components;
mod types;
mod content;
mod engine;

use components::NumberGoUp;

fn main() {
    console_error_panic_hook::set_once(); 

    mount_to_body(|| {
        view! {
            <Router>
                <main>
                    <Routes fallback=|| "Not found.">
                        <Route path=path!("/*any") view=NumberGoUp />
                    </Routes>
                </main>
            </Router>
        }
    })
}
