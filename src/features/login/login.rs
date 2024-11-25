use leptos::*;

use h_modals::{attributes::enums::{Colors, ComponentStatus, Position}, status_modal::components::StatusModal};
use leptos::{html::Input, *};

use crate::{
    features::login::services::directus_login, HasError, Refetcher
};

#[allow(non_snake_case)]
#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="h-full lg:grid lg:grid-cols-3">
            <div class="flex items-center justify-center h-full px-4">
                <div class="flex-shrink-0 w-full max-w-sm shadow-lg card bg-base-100">
                    <div class="card-body">
                        <LoginIsland/>
                    </div>
                </div>
            </div>
            <div class="items-center hidden h-full lg:flex lg:col-span-2">
                <div class="flex flex-col">
                    <div class="flex items-center">
                        <h1 class="text-6xl font-base">
                            "Bespoke Structured Products" <br/> "for your"
                            <span class="font-bold">" digital assets"</span>
                        </h1>
                    </div>
                    <p class="mt-4">A tailored solution for your investment thesis.</p>
                </div>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn LoginIsland() -> impl IntoView {
    let login_has_error = create_rw_signal(false);
    let login_result_message = RwSignal::new("".to_string());
    let login_action = create_action(move |(userid, password): &(String, String)| {
        let userid_clone = userid.clone();
        let password_clone = password.clone();
        async move {
            let result = directus_login(userid_clone, password_clone).await;
            let navigate = leptos_router::use_navigate();

            match result {
                Ok(res) => {
                    if res {
                        use_context::<Refetcher>().unwrap().0.update(|s| *s = !*s);
                        navigate("/quotes/builder", Default::default());
                        true
                    } else {
                        login_has_error.set(true);
                        false
                    }
                }
                Err(e) => {
                    login_has_error.set(true);
                    login_result_message.set(e.get_error_message());
                    false
                },
            }
        }
    });
    // let login_action: Action<DirectusLogin, Result<bool, ServerFnError>> =
    //     create_server_action::<DirectusLogin>();
    let is_pending = login_action.pending();

    create_effect(move |_| {
        log::info!("Is_pending: {:?}", is_pending());

        let value = login_action.value();

        if let Some(data) = value.get() {
            match data {
                true => {
                    use_context::<Refetcher>().unwrap().0.set(true);
                    use_context::<HasError>().unwrap().0.set(false);
                }
                false => {
                    use_context::<Refetcher>().unwrap().0.set(false);
                    use_context::<HasError>().unwrap().0.set(true);
                }
            }
        }
    });

    let email_ref = create_node_ref::<Input>();
    let pass_ref = create_node_ref::<Input>();

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let userid = email_ref.get().expect("input to exist");
            let password = pass_ref.get().expect("input to exist");
            login_action.dispatch((userid.value(), password.value()));
        }>

            <label for="userid" class="label">
                <span class="label-text">Email</span>
            </label>
            <input
                type="text"
                name="userid"
                class="w-full text-black bg-white border-gray-800 rounded shadow-md input input-sm hover:shadow-md"
                autocomplete
                required
                node_ref=email_ref
            />

            <label for="password" class="label">
                <span class="label-text">Password</span>
            </label>
            <input
                type="password"
                name="password"
                class="w-full text-black bg-white border-gray-800 rounded shadow-md input input-sm hover:shadow-md"
                autocomplete
                required
                node_ref=pass_ref
            />
            <label class="label">
                <a href="#" class="label-text-alt link link-hover">
                    Forgot password?
                </a>
            </label>

            {move || match is_pending() {
                true => {
                    view! {
                        <div class="mt-6 form-control">
                            <button type="submit" class="btn btn-block btn-success">
                                <span class="loading loading-spinner loading-sm"></span>
                            </button>
                        </div>
                    }
                        .into_any()
                }
                false => {
                    view! {
                        <div class="mt-6 form-control">
                            <button type="submit" class="rounded btn btn-block btn-success">
                                LOGIN
                            </button>
                        </div>
                    }
                        .into_any()
                }
            }}

        </form>

        <Show when=move || login_has_error.get()>
            <StatusModal
                signal=login_has_error
                title="ERROR!".to_string()
                description=login_result_message.get()
                status=ComponentStatus::Error
                header_text_color=Colors::Black
                desc_text_color=Colors::Black
                position=Position::Center
            />
        </Show>
    }
}
