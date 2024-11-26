// src/components/layouts/modal.rs
use leptos::*;

#[component]
pub fn Modal<T: IntoView>(
    is_open: Signal<bool>,
    is_default_size: bool,
    on_close: impl Fn() + 'static, // Closure that takes no arguments
    title: String,
    modal_content: T,
) -> impl IntoView {
    let modal_box_class = move || {
        if is_default_size {
            "modal-box"  // Default size
        } else {
            "modal-box sm:max-w-[50%] h-[80%]"  // Custom size
        }
    };
    view! {
        <dialog class=move || if is_open.get() { "modal modal-open modal-bottom sm:modal-middle" } else { "modal modal-bottom sm:modal-middle" }>
            <div class=modal_box_class>
                <h3 class="font-bold text-lg">{title}</h3>
                <div class="overflow-y-auto">{modal_content}</div>
                <div class="modal-action">
                    <button class="btn" on:click=move |_| on_close()>
                        Close
                    </button>
                </div>
            </div>
        </dialog>


    }
}
