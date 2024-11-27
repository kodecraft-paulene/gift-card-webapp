use leptos::*;

#[component]
pub fn SidebarItem(path: String, label: String, icon: impl IntoView) -> impl IntoView {
    // Get the current path (this will need to be done through a Leptos signal or directly from the window)
    let current_path = window().location().pathname().unwrap_or_default();

    // Create signal for active state
    let (is_active, set_is_active) = create_signal(current_path == path);

    // Update active state when the path changes
    // (Leptos doesn't automatically re-check path changes like React would with useEffect)

    view! {
        <a
            href=path
            class=move || {
                if is_active.get() {
                    "flex cursor-pointer items-center gap-x-2 p-1 rounded-md text-sm mb-2 pe-10 font-semibold bg-primary text-white hover:bg-primary-focus mb-4"
                } else {
                    "flex cursor-pointer items-center gap-x-2 p-1 rounded-md text-sm mb-2 pe-10 font-semibold hover:bg-slate-100 hover:text-primary text-primary mb-4"
                }
            }
        >
            {icon}
            <div class=move || {
                if is_active.get() {
                    "text-white"
                } else {
                    "text-primary"
                }
            }>
                // Placeholder for the text, although we don't need this div for anything
            </div>
            {label}
        </a>
    }
}
