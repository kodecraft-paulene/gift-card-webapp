
    use leptos::*;
    use web_sys::console;
    use wasm_bindgen::JsValue;
    use crate::components::types::LinkItem; 
    use leptos_router::use_location;

    #[component]
    pub fn SidebarDropdownItem(
        label: String,
        icon: impl IntoView,
        linksitem: Vec<LinkItem>, 
    ) -> impl IntoView {
  
        let (is_opened, set_is_opened) = create_signal(false);
        let location = use_location().pathname;
        let location_str = location.get();
        let has_active_link = linksitem.iter().any(|link| link.url == location_str);
    
  
        if has_active_link {  
           // console::log_1(&JsValue::from_str(&location_str));  // Pass the &str to log
            set_is_opened(true);
        } else {
            set_is_opened(false);
        }
    
    
        view! {
            <div class="mb-4">
                <button
                    class="flex cursor-pointer items-center gap-x-2 p-1 rounded-md text-sm mb-2 pe-10 font-semibold hover:bg-slate-100 hover:text-primary text-primary w-full"
                    on:click=move |_| set_is_opened(!is_opened())
                >
                    <div class="flex items-center gap-x-4 w-full justify-between">
                        <div class="flex items-center gap-x-4">
                            {icon}
                            {label}
                        </div>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            class=move || {
                                if is_opened() {
                                    "w-5 h-5 transform rotate-180 duration-150"
                                } else {
                                    "w-5 h-5 duration-150"
                                }
                            }
                        >
                            <path
                                fill-rule="evenodd"
                                d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
                                clip-rule="evenodd"
                            />
                        </svg>
                    </div>
                </button>
    
                <ul
                class=move || {
                    if is_opened() {
                        "pl-4 space-y-2"
                    } else {
                       "pl-4 space-y-2 hidden"
                    }
                }
                >
                    {linksitem.iter().map(|link| {
              
                        let is_active = link.url == location_str;
                        
                        view! {
                            <li>
                                <a
                                    href={link.url.clone()}
                                    class=move || {
                                        if is_active {
                                            "flex items-center gap-x-2  p-2 font-semibold rounded-lg mb-2 text-xs active:bg-gray-100 duration-150 mt-2 bg-primary text-white"
                                        } else {
                                            "flex items-center gap-x-2 text-gray-600 p-2 font-semibold rounded-lg mb-2 text-xs active:bg-gray-100 duration-150 mt-2 hover:bg-slate-100 hover:text-primary"
                                        }
                                    }
                                >
                                    {&link.label}
                                </a>
                            </li>
                        }
                    }).collect::<Vec<_>>() }
                </ul>
            </div>
    
        }
    }