use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;

mod components;
use components::{NumberGoUp, };


fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! {
        <Router>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=NumberGoUp />    
                </Routes>    
            </main>
        </Router>
    })
}
