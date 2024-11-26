use crate::components::layouts::modal::Modal;
use crate::components::forms::loginform::Loginform;
use crate::components::layouts::gallerysection::GallerySection;
use crate::components::layouts::herosection::HeroSection;
use leptos::*;

#[component]
pub fn Landing() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);

    let open_modal = move || set_is_open.set(true);
    let close_modal = move || set_is_open.set(false);

    view! {
        <div class="select-none">
            <HeroSection open_modal=open_modal />
            <GallerySection />
            <Modal
                is_open=is_open.into() // Convert ReadSignal to Signal
                on_close=close_modal
                title=String::from("")
                is_default_size={true}
                modal_content=view! {
                    <Loginform is_show=is_open.into() />
                }
            />
        </div>
    }
}
