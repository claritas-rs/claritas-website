use leptos::prelude::*;

#[component]
pub fn Stats() -> impl IntoView {
    view! {
        <section class="relative">
            <div class="divider absolute top-0 left-0"></div>
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10">
                <div class="grid grid-cols-2 md:grid-cols-4 gap-0 divide-x divide-sidebar-border border border-sidebar-border animate-on-scroll">
                    <div class="px-6 py-5 flex flex-col items-center text-center">
                        <span class="text-2xl font-bold text-sidebar-title tracking-tight">"< 50ms"</span>
                        <span class="text-[10px] text-sidebar-muted uppercase tracking-[2px] mt-1">"Startup time"</span>
                    </div>
                    <div class="px-6 py-5 flex flex-col items-center text-center">
                        <span class="text-2xl font-bold text-sidebar-title tracking-tight">"~10MB"</span>
                        <span class="text-[10px] text-sidebar-muted uppercase tracking-[2px] mt-1">"Binary size"</span>
                    </div>
                    <div class="px-6 py-5 flex flex-col items-center text-center">
                        <span class="text-2xl font-bold text-sidebar-title tracking-tight">"EPUB + PDF"</span>
                        <span class="text-[10px] text-sidebar-muted uppercase tracking-[2px] mt-1">"Formats supported"</span>
                    </div>
                    <div class="px-6 py-5 flex flex-col items-center text-center">
                        <span class="text-2xl font-bold text-sidebar-title tracking-tight">"100%"</span>
                        <span class="text-[10px] text-sidebar-muted uppercase tracking-[2px] mt-1">"Local & Private"</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
