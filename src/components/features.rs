use leptos::prelude::*;

#[component]
pub fn Features() -> impl IntoView {
    view! {
        <section id="features" class="py-24 relative">
            <div class="divider absolute top-0 left-0"></div>
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center mb-16">
                    <h2 class="text-xl md:text-2xl font-bold mb-6 tracking-[3px]">"WHY CLARITAS?"</h2>
                    <p class="text-sm text-sidebar-muted max-w-2xl mx-auto uppercase tracking-wide">"The EPUB reader your CPU was waiting for."</p>
                </div>
                
                <div class="grid md:grid-cols-3 gap-8">
                    // Feature 1 - Origin story
                    <div class="flat-card animate-on-scroll" style="animation-delay: 100ms;">
                        <div class="text-sidebar-title opacity-70 mb-4">
                            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                        </div>
                        <h3 class="text-sm font-bold mb-3 text-sidebar-title uppercase tracking-widest">"NO ELECTRON"</h3>
                        <p class="text-xs text-sidebar-muted leading-relaxed">"Tired of bloated Electron apps eating RAM just to show text? Claritas is compiled native Rust, no Node.js, no Chromium, no overhead. Your fan stays quiet."</p>
                    </div>
                    
                    // Feature 2
                    <div class="flat-card animate-on-scroll" style="animation-delay: 200ms;">
                        <div class="text-sidebar-title opacity-70 mb-4">
                            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"></path></svg>
                        </div>
                        <h3 class="text-sm font-bold mb-3 text-sidebar-title uppercase tracking-widest">"HIGHLY CUSTOMIZABLE"</h3>
                        <p class="text-xs text-sidebar-muted leading-relaxed">"Adjust fonts, themes, spacing, and reading layout to match your exact preferences. Built to get out of the way of your reading."</p>
                    </div>
                    
                    // Feature 3
                    <div class="flat-card animate-on-scroll" style="animation-delay: 300ms;">
                        <div class="text-sidebar-title opacity-70 mb-4">
                            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"></path></svg>
                        </div>
                        <h3 class="text-sm font-bold mb-3 text-sidebar-title uppercase tracking-widest">"CROSS PLATFORM"</h3>
                        <p class="text-xs text-sidebar-muted leading-relaxed">"One codebase, three platforms. Claritas runs natively on Windows, macOS, and Linux, without virtualization or compatibility shims."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
