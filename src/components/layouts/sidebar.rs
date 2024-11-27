use crate::components::layouts::sidebaritem::SidebarItem;
use crate::components::layouts::sidebardropdownitem::SidebarDropdownItem;
use leptos::*;
use crate::components::types::LinkItem; // Import from the shared module

use crate::components::ui::svg;

#[component]
pub fn Sidebar(children: Children) -> impl IntoView {
    let (show_sidebar, set_show_sidebar) = create_signal(false);
    // Toggle Sidebar function
    let toggle_sidebar = move || {
        set_show_sidebar.update(|show| *show = !*show);
    };
    let links = vec![
        LinkItem {
            url: String::from("/openorder"),
            label: String::from("Open Orders"),
        },
        LinkItem {
            url: String::from("/closedorder"),
            label: String::from("Closed Orders"),
        },
        LinkItem {
            url: String::from("/returnedorder"),
            label: String::from("Returned Orders"),
        },
    ];
    let reportlinks = vec![
        LinkItem {
            url: String::from("/Audit Trail"),
            label: String::from("Audit Trail"),
        }
       
    ];
    // Borrow `children` to make the closure implement `Fn` instead of `FnOnce`
    let view_fn = move || {
        view! {

 
        <div class="fixed top-0 left-0 z-50 xl:hidden p-4 text-black bg-white w-full border">
                    <button class="mt-1" on:click=move |_| toggle_sidebar()>
                   { svg::hamburger_icon()}
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

                            <SidebarDropdownItem  label="Orders".to_string() icon={svg::shoppingbag_icon()}  linksitem=links/>
                            <SidebarDropdownItem  label="Reports".to_string() icon={svg::report_icon()}  linksitem=reportlinks/>
                        </div>

                        <div class="inset-x-0 bottom-0 h-40 border-t mt-20">
                            <div class="mt-4">
                                  <SidebarItem path="/changepassword".to_string() label="Change Password".to_string() icon=view!{    {svg::key_icon()}} />
                                  <SidebarItem path="/settings".to_string() label="Settings".to_string() icon=view!{    {svg::gear_icon()}} />
                                  <SidebarItem path="/logout".to_string() label="Log out".to_string() icon=view!{    {svg::logout_icon()}} />
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
