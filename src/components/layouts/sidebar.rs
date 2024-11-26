use crate::components::layouts::sidebaritem::SidebarItem;
use leptos::*;
#[component]
pub fn Sidebar(children: Children) -> impl IntoView {
    let (show_sidebar, set_show_sidebar) = create_signal(false);

    // Toggle Sidebar function
    let toggle_sidebar = move || {
        set_show_sidebar.update(|show| *show = !*show);
    };
    // Borrow `children` to make the closure implement `Fn` instead of `FnOnce`
    let view_fn = move || {
        view! {

                // <div class="drawer lg:drawer-open">
                //   <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
                //   <div class="drawer-content">
                //   {children()}
                //     <label for="my-drawer-2" class="btn btn-primary drawer-button lg:hidden">
                //       Open drawer
                //     </label>
                //   </div>
                //   <div class="drawer-side">
                //     <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label>
                //     <ul class="menu bg-base-200 text-base-content min-h-full w-80 p-4">

                //       <li><a>Sidebar Item 1</a></li>
                //       <li><a>Sidebar Item 2</a></li>
                //     </ul>
                //   </div>
                // </div>
                //


        <div class="fixed top-0 left-0 z-50 xl:hidden p-4 text-black bg-white w-full border">
                    <button class="mt-1" on:click=move |_| toggle_sidebar()>
                        >>>
                    </button>
                </div>

                <aside id="logo-sidebar" class=move || {
                    format!(
                        "fixed top-0 left-0 !z-40 w-64 h-screen pt-20 transition-transform bg-white border-r border-gray-200 xl:translate-x-0  {}",
                        if show_sidebar.get() { "translate-x-0" } else { "-translate-x-full" }
                    )
                } aria-label="Sidebar">
                    <div class="h-full flex flex-col px-3 pb-4 overflow-y-auto bg-white xl:mt-[-70px]">
                        <div>
                            {if show_sidebar.get() {
                                view! {
                                    <button class="fixed top-0 left-0 z-50 xl:hidden p-4 mt-1 text-black" on:click=move |_| toggle_sidebar()>

                                    </button>
                                }
                            } else {
                                view! {  <button class="fixed top-0 left-0 z-50 xl:hidden p-4 mt-1 text-black" on:click=move |_| toggle_sidebar()>

                                </button> }
                            }}

                            <div class="ms-[-65px] mb-4 hidden xl:block">
                                <img class="h-8 w-auto"   src="images/gift-card-logo.svg" alt="GiftCardLogo"/>
                            </div>

                            <div class="flex rounded-md h-[70px]">
                                <div class="ms-2 mt-4">
                                    <div class="flex">
                                        <div class="avatar">
                                            <div class="w-10 rounded-full">
                                                <img src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp" />
                                            </div>
                                        </div>
                                        <div class="ms-4 mt-[5px] items-center w-full">
                                            <label class="text-xs text-primary font-semibold">"Vendor Dummy"</label>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <hr class="my-8" />

                            <SidebarItem path="/home".to_string() label="Home".to_string() icon=view!{} />

                        </div>

                        <div class="inset-x-0 bottom-0 h-40 border-t mt-20">
                            <div class="mt-4">

                            </div>
                        </div>
                    </div>
                </aside>

                <div class="mt-20 xl:mt-[20] xl:ms-64">
                {children()}
                </div>
                 }
    };

    // Return the view wrapped in a closure, borrowing `children`
    view_fn()
}
