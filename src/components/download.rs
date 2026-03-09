use leptos::prelude::*;

#[component]
pub fn Download() -> impl IntoView {
    view! {
        <section id="download" class="py-24 relative">
            <div class="divider absolute top-0 left-0"></div>
            <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center relative z-10">
                <h2 class="text-xl md:text-2xl font-bold mb-6 tracking-[3px]">"READY TO START READING?"</h2>
                <p class="text-sm text-sidebar-muted mb-12 tracking-wide uppercase">"Download the latest version of Claritas for your operating system."</p>
                
                <div class="grid sm:grid-cols-3 gap-4 mb-16 animate-on-scroll">
                    <a href="https://github.com/claritas-rs/claritas/releases/latest" target="_blank" class="flat-card group !p-[16px]">
                        <svg class="w-8 h-8 text-sidebar-muted group-hover:text-sidebar-text transition-colors mb-2" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M0 3.449L9.75 2.1v9.451H0m10.949-9.602L24 0v11.4H10.949M0 12.6h9.75v9.451L0 20.699M10.949 12.6H24V24l-12.9-1.801"></path></svg>
                        <span class="text-[13px] font-bold tracking-wide uppercase">"Windows"</span>
                    </a>
                    <a href="https://github.com/claritas-rs/claritas/releases/latest" target="_blank" class="flat-card group !p-[16px]">
                        <svg viewBox="0 0 384 512" fill="currentColor" class="w-8 h-8 text-sidebar-muted group-hover:text-sidebar-text transition-colors mb-2"><path d="M318.7 268.7c-.2-36.7 16.4-64.4 50-84.8-18.8-26.9-47.2-41.7-84.7-44.6-35.5-2.8-74.3 20.7-88.5 20.7-15 0-49.4-19.7-76.4-19.7C63.3 141.2 4 184.8 4 273.5q0 39.3 14.1 81.5c10.3 32 47.5 113.8 83.5 114 33.7.2 40.7-23.9 86.9-23.9 44.9 0 51.4 23.9 89.4 23.9 41.2-.1 75.4-86.3 84.7-111.4-85.7-40.4-103.8-112.5-43.9-156.4L318.7 268.7zM260.4 76.9c16.3-19.5 27.2-46.7 24.3-76.9-25.1 2-55.7 16.9-72.8 38.6-14.7 19.3-27.5 48.3-23.1 76.9 28.3 1.9 56.6-18.4 71.6-38.6z"/></svg>
                        <span class="text-[13px] font-bold tracking-wide uppercase">"macOS"</span>
                    </a>
                    <a href="https://github.com/claritas-rs/claritas/releases/latest" target="_blank" class="flat-card group !p-[16px]">
                        <svg viewBox="0 0 128 128" fill="currentColor" class="w-8 h-8 text-sidebar-muted group-hover:text-sidebar-text transition-colors mb-2"><path fill-rule="evenodd" clip-rule="evenodd" d="M113.823 104.595c-1.795-1.478-3.629-2.921-5.308-4.525-1.87-1.785-3.045-3.944-2.789-6.678.147-1.573-.216-2.926-2.113-3.452.446-1.154.864-1.928 1.033-2.753.188-.92.178-1.887.204-2.834.264-9.96-3.334-18.691-8.663-26.835-2.454-3.748-5.017-7.429-7.633-11.066-4.092-5.688-5.559-12.078-5.633-18.981a47.564 47.564 0 00-1.081-9.475C80.527 11.956 77.291 7.233 71.422 4.7c-4.497-1.942-9.152-2.327-13.901-1.084-6.901 1.805-11.074 6.934-10.996 14.088.074 6.885.417 13.779.922 20.648.288 3.893-.312 7.252-2.895 10.34-2.484 2.969-4.706 6.172-6.858 9.397-1.229 1.844-2.317 3.853-3.077 5.931-2.07 5.663-3.973 11.373-7.276 16.5-1.224 1.9-1.363 4.026-.494 6.199.225.563.363 1.429.089 1.882-2.354 3.907-5.011 7.345-10.066 8.095-3.976.591-4.172 1.314-4.051 5.413.1 3.337.061 6.705-.28 10.021-.363 3.555.008 4.521 3.442 5.373 7.924 1.968 15.913 3.647 23.492 6.854 3.227 1.365 6.465.891 9.064-1.763 2.713-2.771 6.141-3.855 9.844-3.859 6.285-.005 12.572.298 18.86.369 1.702.02 2.679.653 3.364 2.199.84 1.893 2.26 3.284 4.445 3.526 4.193.462 8.013-.16 11.19-3.359 3.918-3.948 8.436-7.066 13.615-9.227 1.482-.619 2.878-1.592 4.103-2.648 2.231-1.922 2.113-3.146-.135-5zM62.426 24.12c.758-2.601 2.537-4.289 5.243-4.801 2.276-.43 4.203.688 5.639 3.246 1.546 2.758 2.054 5.64.734 8.658-1.083 2.474-1.591 2.707-4.123 1.868-.474-.157-.937-.343-1.777-.652.708-.594 1.154-1.035 1.664-1.382 1.134-.772 1.452-1.858 1.346-3.148-.139-1.694-1.471-3.194-2.837-3.175-1.225.017-2.262 1.167-2.4 2.915-.086 1.089.095 2.199.173 3.589-3.446-1.023-4.711-3.525-3.662-7.118zm-12.75-2.251c1.274-1.928 3.197-2.314 5.101-1.024 2.029 1.376 3.547 5.256 2.763 7.576-.285.844-1.127 1.5-1.716 2.241l-.604-.374c-.23-1.253-.276-2.585-.757-3.733-.304-.728-1.257-1.184-1.919-1.762-.622.739-1.693 1.443-1.757 2.228-.088 1.084.477 2.28.969 3.331.311.661 1.001 1.145 1.713 1.916l-1.922 1.51c-3.018-2.7-3.915-8.82-1.871-11.909zM87.34 86.075c-.203 2.604-.5 2.713-3.118 3.098-1.859.272-2.359.756-2.453 2.964a101.744 101.744 0 00-.012 7.753c.061 1.77-.537 3.158-1.755 4.393-6.764 6.856-14.845 10.105-24.512 8.926-4.17-.509-6.896-3.047-9.097-6.639.98-.363 1.705-.607 2.412-.894 3.122-1.27 3.706-3.955 1.213-6.277-1.884-1.757-3.986-3.283-6.007-4.892-1.954-1.555-3.934-3.078-5.891-4.629-1.668-1.323-2.305-3.028-2.345-5.188-.094-5.182.972-10.03 3.138-14.747 1.932-4.209 3.429-8.617 5.239-12.885.935-2.202 1.906-4.455 3.278-6.388 1.319-1.854 2.134-3.669 1.988-5.94-.084-1.276-.016-2.562-.016-3.843l.707-.352c1.141.985 2.302 1.949 3.423 2.959 4.045 3.646 7.892 3.813 12.319.67 1.888-1.341 3.93-2.47 5.927-3.652.497-.294 1.092-.423 1.934-.738 2.151 5.066 4.262 10.033 6.375 15 1.072 2.524 1.932 5.167 3.264 7.547 2.671 4.775 4.092 9.813 4.07 15.272-.012 2.83.137 5.67-.081 8.482z"/></svg>
                        <span class="text-[13px] font-bold tracking-wide uppercase">"Linux"</span>
                    </a>
                </div>
                
                <div class="flat-card text-left items-start animate-on-scroll" style="animation-delay: 200ms;">
                    <h3 class="font-bold text-sm mb-2 tracking-[2px] uppercase">"Install via Cargo"</h3>
                    <p class="text-xs text-sidebar-muted mb-4">"If you have Rust installed, you can build Claritas directly from source:"</p>
                    <div class="w-full bg-input-bg rounded-[5px] p-3 flex items-center justify-between border border-input-border">
                        <code class="text-sidebar-text text-xs opacity-90 overflow-x-auto whitespace-nowrap mr-4">"cargo install --git https://github.com/claritas-rs/claritas.git"</code>
                        <button class="text-sidebar-muted hover:text-sidebar-text transition-colors flex-shrink-0" onclick="navigator.clipboard.writeText('cargo install --git https://github.com/claritas-rs/claritas.git')">
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}
