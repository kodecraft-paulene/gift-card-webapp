use leptos::*;

#[component]
pub fn GallerySection() -> impl IntoView {
    view! {
        <div class="relative overflow-hidden bg-white">
                <div class="pb-80 pt-16 sm:pb-40 sm:pt-24 lg:pb-48 lg:pt-40">
                    <div class="relative mx-auto max-w-7xl px-4 sm:static sm:px-6 lg:px-8">
                        <div class="sm:max-w-lg">
                            <h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-6xl">
                                Whatever You are Into, Get Into gift card.
                            </h1>
                            <p class="mt-4 text-xl text-gray-500">
                                Travel | Food | Shopping | Games | Hotels | Cars | Planes
                            </p>
                        </div>
                        <div>
                            <div class="mt-10">
                                <div
                                    aria-hidden="true"
                                    class="pointer-events-none lg:absolute lg:inset-y-0 lg:mx-auto lg:w-full lg:max-w-7xl"
                                >
                                    <div class="absolute transform sm:left-1/2 sm:top-0 sm:translate-x-8 lg:left-1/2 lg:top-1/2 lg:-translate-y-1/2 lg:translate-x-8">
                                        <div class="flex items-center space-x-6 lg:space-x-8">
                                            <div class="grid flex-shrink-0 grid-cols-1 gap-y-6 lg:gap-y-8">
                                                <div class="h-64 w-44 overflow-hidden rounded-lg sm:opacity-0 lg:opacity-100">
                                                    <img src="images/p-car.jpg" alt="" class="h-full w-full object-cover object-center" />
                                                </div>
                                                <div class="h-64 w-44 overflow-hidden rounded-lg">
                                                    <img src="images/p-food.jpg" alt="" class="h-full w-full object-cover object-center" />
                                                </div>
                                            </div>
                                            <div class="grid flex-shrink-0 grid-cols-1 gap-y-6 lg:gap-y-8">
                                                <div class="h-64 w-44 overflow-hidden rounded-lg">
                                                    <img src="images/p-shop.jpg" alt="" class="h-full w-full object-cover object-center" />
                                                </div>
                                                <div class="h-64 w-44 overflow-hidden rounded-lg">
                                                    <img src="images/p-cart.jpg" alt="" class="h-full w-full object-cover object-center" />
                                                </div>
                                                <div class="h-64 w-44 overflow-hidden rounded-lg">
                                                    <img src="images/p-hotel.jpg" alt="" class="h-full w-full object-cover object-center" />
                                                </div>
                                            </div>
                                            <div class="grid flex-shrink-0 grid-cols-1 gap-y-6 lg:gap-y-8">
                                                <div class="h-64 w-44 overflow-hidden rounded-lg">
                                                    <img src="images/p-games.jpg" alt="" class="h-full w-full object-cover object-center" />
                                                </div>
                                                <div class="h-64 w-44 overflow-hidden rounded-lg">
                                                    <img src="images/p-plane.jpg" alt="" class="h-full w-full object-cover object-center" />
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
    }
}