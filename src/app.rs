use leptos::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::hero::Hero;
use crate::components::stats::Stats;
use crate::components::features::Features;
use crate::components::download::Download;
use crate::components::footer::Footer;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col bg-app-bg bg-grid-pattern">
            <Navbar/>
            <main class="flex-grow">
                <Hero/>
                <Stats/>
                <Features/>
                <Download/>
            </main>
            <Footer/>
        </div>
    }
}
