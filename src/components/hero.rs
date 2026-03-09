use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    let selected_image = RwSignal::new(Option::<String>::None);
    
    view! {
        <section class="relative pt-24 pb-12 lg:pt-32 lg:pb-20 flex flex-col items-center justify-center">
            <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
                // Top label
                <div class="flex justify-center mb-6">
                    <span class="text-[10px] text-sidebar-muted tracking-[3px] uppercase border border-sidebar-border px-3 py-1">
                        "Built with Rust // No Electron"
                    </span>
                </div>

                <h1 class="text-4xl md:text-5xl font-normal mb-8 leading-tight text-center">
                    <span class="block text-sidebar-title uppercase tracking-[4px]">"Your books,"</span>
                    <span class="block text-sidebar-muted uppercase tracking-[4px] mt-2">"beautifully clear."</span>
                </h1>



                <div class="flex flex-col sm:flex-row gap-4 justify-center mb-16">
                    <a href="#download" class="btn-primary">
                        <span class="nav-icon">"↓"</span>
                        "DOWNLOAD FOR FREE"
                    </a>
                    <a href="https://github.com/claritas-rs/claritas" target="_blank" rel="noopener noreferrer" class="btn-primary">
                        <span class="nav-icon">"</>"</span>
                        "VIEW SOURCE"
                    </a>
                </div>

                // App screenshots showcase
                <div class="relative mx-auto max-w-6xl mt-8">
                    // Glow behind the screenshots
                    <div class="absolute inset-x-10 -inset-y-10 bg-[#3a3a3a] opacity-20 blur-[100px] animate-pulse-glow z-0"></div>
                    
                    <div class="relative z-10 grid grid-cols-1 md:grid-cols-2 gap-6 animate-on-scroll">
                        // Main Library View
                        <div 
                            on:click=move |_| selected_image.set(Some("assets/epub_library.png".to_string()))
                            class="border border-sidebar-border rounded-[4px] overflow-hidden shadow-2xl bg-app-bg transform transition-all cursor-pointer hover:scale-[1.02] hover:-translate-y-1 hover:border-[#4a4a4a] duration-300">
                            <div class="flex items-center gap-2 px-4 py-2 bg-sidebar-bg border-b border-sidebar-border">
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <span class="ml-2 text-[10px] text-sidebar-muted tracking-wide">"Claritas — Library"</span>
                            </div>
                            <img src="assets/epub_library.png" alt="Library View" class="w-full object-cover opacity-90" />
                        </div>

                        // Reading View
                        <div 
                            on:click=move |_| selected_image.set(Some("assets/content.png".to_string()))
                            class="border border-sidebar-border rounded-[4px] overflow-hidden shadow-2xl bg-app-bg transform transition-all cursor-pointer hover:scale-[1.02] hover:-translate-y-1 hover:border-[#4a4a4a] duration-300 md:mt-12">
                            <div class="flex items-center gap-2 px-4 py-2 bg-sidebar-bg border-b border-sidebar-border">
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <span class="ml-2 text-[10px] text-sidebar-muted tracking-wide">"Claritas — Reading"</span>
                            </div>
                            <img src="assets/content.png" alt="Reading View" class="w-full object-cover opacity-90" />
                        </div>
                        
                        // Settings View
                        <div 
                            on:click=move |_| selected_image.set(Some("assets/settings.png".to_string()))
                            class="border border-sidebar-border rounded-[4px] overflow-hidden shadow-2xl bg-app-bg transform transition-all cursor-pointer hover:scale-[1.02] hover:-translate-y-1 hover:border-[#4a4a4a] duration-300 md:-mt-6">
                            <div class="flex items-center gap-2 px-4 py-2 bg-sidebar-bg border-b border-sidebar-border">
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <span class="ml-2 text-[10px] text-sidebar-muted tracking-wide">"Claritas — Settings"</span>
                            </div>
                            <img src="assets/settings.png" alt="Settings View" class="w-full object-cover opacity-90" />
                        </div>

                        // Empty State View
                        <div 
                            on:click=move |_| selected_image.set(Some("assets/principal_page_no_books.png".to_string()))
                            class="border border-sidebar-border rounded-[4px] overflow-hidden shadow-2xl bg-app-bg transform transition-all cursor-pointer hover:scale-[1.02] hover:-translate-y-1 hover:border-[#4a4a4a] duration-300 md:mt-6">
                            <div class="flex items-center gap-2 px-4 py-2 bg-sidebar-bg border-b border-sidebar-border">
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <div class="w-2.5 h-2.5 rounded-full bg-sidebar-muted opacity-50"></div>
                                <span class="ml-2 text-[10px] text-sidebar-muted tracking-wide">"Claritas — Empty Library"</span>
                            </div>
                            <img src="assets/principal_page_no_books.png" alt="Empty Library View" class="w-full object-cover opacity-90" />
                        </div>
                    </div>
                </div>
            </div>

            // Lightbox Modal
            <Show when=move || selected_image.get().is_some()>
                <div 
                    class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-8 bg-black/90 backdrop-blur-sm transition-opacity"
                    on:click=move |_| selected_image.set(None)
                >
                    <button 
                        class="absolute top-6 right-6 text-sidebar-muted hover:text-white bg-sidebar-bg/50 hover:bg-sidebar-border rounded-full p-2 transition-colors cursor-pointer"
                        on:click=move |e| {
                            e.stop_propagation();
                            selected_image.set(None);
                        }
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                    </button>
                    
                    <div 
                        class="relative max-w-[95vw] max-h-[95vh] border border-sidebar-border rounded-lg shadow-2xl overflow-hidden animate-fade-in-up"
                        on:click=move |e| e.stop_propagation()
                    >
                        <img 
                            src=move || selected_image.get().unwrap_or_default() 
                            class="max-w-full max-h-[90vh] object-contain" 
                            alt="Expanded application screenshot"
                        />
                    </div>
                </div>
            </Show>
        </section>
    }
}
