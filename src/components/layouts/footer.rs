use leptos::*;
use chrono::Utc;

#[component]
pub fn Footer() -> impl IntoView {
    let future_date = Utc::now();
    let formatted_date = future_date.format("%Y").to_string();
    let copyright = "\u{A9}";
    view! {
        <footer class="footer footer-center bg-base-300 text-base-content p-4">
            <aside>
                <p>{copyright}{formatted_date} Kodecraft. All rights reserved.</p>
            </aside>
        </footer>
    }
}
