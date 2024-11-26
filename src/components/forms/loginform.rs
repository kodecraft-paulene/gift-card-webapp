use leptos::*;

#[component]
pub fn Loginform(is_show: ReadSignal<bool>) -> impl IntoView {
    let (is_forgot, set_is_forgot) = create_signal(false);

    create_effect(move |_| {
        set_is_forgot.set(!is_show.get());
    });
    
    view! {
        <div>
            <div>
                {move || if is_forgot.get() {
                    view! {
                        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
                            <form class="space-y-4">
                                <div class="mb-2">
                                    <label for="email" class="block text-sm font-medium text-gray-700 mb-2">"Email Address"</label>
                                    <input
                                        id="email"
                                        name="email"
                                        type="text"
                                        required
                                        placeholder="Email Address"
                                        class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm input input-md"
                                    />
                                </div>


                                <div class="flex justify-center">
                                    <button
                                        type="submit"
                                        class="btn btn-accent text-white w-full px-4 py-2 rounded-md shadow-sm hover:bg-indigo-700 focus:ring-2 focus:ring-indigo-500"
                                    >
                                        "Submit"
                                    </button>
                                </div>
                            </form>
                            <button
                                class="text-left text-sm text-gray-500 mt-4 "
                                on:click=move |_| set_is_forgot(false)
                            >
                                "Back to Login"
                            </button>

                            <p class="mt-10 text-center text-sm text-gray-500">
                                "Not a member? "
                                <a
                                    href="#"
                                    class="font-semibold text-indigo-600 hover:text-indigo-500"
                                >
                                    "Join us"
                                </a>
                            </p>
                        </div>
                    }
                } else {
                    view! {
                        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
                            <form class="space-y-4">
                                <div class="mb-2">
                                    <label for="email" class="block text-sm font-medium text-gray-700 mb-2">"Email Address"</label>
                                    <input
                                        id="email"
                                        name="email"
                                        type="text"
                                        required
                                        placeholder="Email Address"
                                        class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm input input-md"
                                    />
                                </div>

                                <div class="mb-2">
                                    <label for="password" class="block text-sm font-medium text-gray-700 mb-2">"Password"</label>
                                    <input
                                        id="password"
                                        name="password"
                                        type="password"
                                        required
                                        placeholder="•••••••••"
                                        class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm input input-md"
                                    />
                                </div>

                                <div class="flex justify-center">
                                    <button
                                        type="submit"
                                        class="btn btn-accent text-white w-full px-4 py-2 rounded-md shadow-sm hover:bg-indigo-700 focus:ring-2 focus:ring-indigo-500"
                                    >
                                        "Submit"
                                    </button>
                                </div>
                            </form>

                            <button
                                class="text-left text-sm text-gray-500 mt-4"
                                on:click=move |_| set_is_forgot(true)
                            >
                                "Forgot Password"
                            </button>

                            <p class="mt-10 text-center text-sm text-gray-500">
                                "Not a member? "
                                <a
                                    href="#"
                                    class="font-semibold text-indigo-600 hover:text-indigo-500"
                                >
                                    "Join us"
                                </a>
                            </p>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
