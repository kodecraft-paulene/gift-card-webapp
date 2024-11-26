use crate::components::forms::loginform::Loginform;
use crate::components::layouts::footer::Footer;
use leptos::*;
#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <div class="select-none mt-[100px]">
            <Loginform />
            </div>
        </div>
        <Footer/>
    }
}
