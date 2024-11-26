use leptos::*;

#[component]
//pub fn Loginform(is_show: ReadSignal<bool>) -> impl IntoView {
// create_effect(move |_| {
//     set_is_forgot.set(!is_show.get());
// });
pub fn Loginform() -> impl IntoView {
    let (is_forgot, set_is_forgot) = create_signal(false);

    view! {
  
            <div >
          
                {move || if is_forgot.get() {
                    view! {
                        <div class="md:flex items-center justify-center">
                        <div class="lg:w-1/2 w-full justify-center ">
                            <h1 class="text-2xl font-semibold mb-4 text-primary text-center">Forgot Password</h1>
                            <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm mx-10">
                                <form class="space-y-4">
                                    <div class="mb-2">
                                        <label for="email" class="block text-sm font-medium text-gray-700 mb-2">Email Address</label>
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
                                            Submit
                                        </button>
                                    </div>
                                </form>
                                <button
                                    class="text-left text-sm text-gray-500 mt-4"
                                    on:click=move |_| set_is_forgot(false)
                                >
                                    Back to Login
                                </button>
                            </div>
                        </div>
                    </div>
                    
                    }
                } else {
                    view! {
                        <div class="md:flex items-center justify-center">
                        <div class="lg:w-1/2  justify-center w-full">
                        <h1 class="text-2xl font-semibold mb-4 text-primary text-center">Sign in to your account</h1>
                            <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm mx-10">
                                <form class="space-y-4">
                                    <div class="mb-2">
                                        <label for="email" class="block text-sm font-medium text-gray-700 mb-2">Email Address</label>
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
                                        <label for="password" class="block text-sm font-medium text-gray-700 mb-2">Password</label>
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
                                            Submit
                                        </button>
                                    </div>
                                </form>
                    
                                <button
                                    class="text-left text-sm text-gray-500 mt-4"
                                    on:click=move |_| set_is_forgot(true)
                                >
                                    Forgot Password
                                </button>
                            </div>
                        </div>
                    </div>
                    
                    }
                }}
            </div>
       
    }
}
