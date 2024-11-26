use leptos::*;

/// A parameterized incrementing button
#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <div class="navbar bg-primary text-white flex justify-center md:justify-start">
            <img
                class="h-8 w-auto"
                src="images/gift-card-logo.svg"
                alt="GiftCardLogo"
            />
        </div>
    }
}
