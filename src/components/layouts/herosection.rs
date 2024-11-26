use leptos::*;

#[component]
pub fn HeroSection(open_modal: impl Fn() + 'static) -> impl IntoView {
    view! {
        <div class="relative w-full h-[320px]" id="home">
            <div class="absolute inset-0 opacity-70">
                <img
                    src="images/gift-card-banner.jpg"
                    alt="Background Image"
                    class="object-cover object-center w-full h-full"
                />
            </div>
            <div class="absolute inset-9 flex flex-col md:flex-row items-center justify-between">
                <div class="md:w-1/2 mb-4 md:mb-0">
                    <h1 class="text-4xl font-bold tracking-tight text-white sm:text-5xl">
                        Ding-Dong! gift card calling!
                    </h1>
                    <p class="font-regular text-xl mb-8 mt-4 text-gray-500">
                        See the face you love Light Up with gift card.
                    </p>
                    <button
                        on:click=move |_| open_modal()
                        type="button"
                        class="btn text-white btn-accent rounded-full"
                    >
                        Shop Now
                    </button>
                </div>
            </div>
        </div>
    }
}
