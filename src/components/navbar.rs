use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="w-full bg-app-bg border-b border-sidebar-border relative z-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-14">
                    <div class="flex items-center gap-3 cursor-pointer">
                        <div class="w-6 h-6 flex items-center justify-center">
                            <img src="assets/icon.png" alt="Claritas Logo" class="w-full h-full object-contain" />
                        </div>
                        <span class="text-sidebar-title font-bold text-[16px] tracking-[3px] uppercase">Claritas</span>
                    </div>
                    
                    <div class="flex md:hidden">
                        <a href="#download" class="btn-secondary !text-[11px] !px-3 !py-1.5 border border-sidebar-border bg-sidebar-bg">
                            "Download"
                        </a>
                    </div>
                    <div class="hidden md:block">
                        <div class="ml-10 flex items-baseline space-x-2">
                            <a href="#features" class="btn-secondary">
                                <span class="nav-icon">"#"</span>
                                "Features"
                            </a>
                            <a href="#download" class="btn-secondary">
                                <span class="nav-icon">"⬇"</span>
                                "Download"
                            </a>
                            <a href="https://github.com/claritas-rs/claritas" target="_blank" rel="noopener noreferrer" class="btn-secondary">
                                <span class="nav-icon">"</>"</span>
                                "GitHub"
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
