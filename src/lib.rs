use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod commons;
mod components;
mod features;
mod utilities;

// Top-Level pages
use crate::features::landing::landing::Landing;
use crate::features::not_found::NotFound;
use components::layouts::footer::Footer;
use components::layouts::navigation_bar::NavBar;
use components::layouts::sidebar::Sidebar;

#[derive(Copy, Clone)]
pub struct Refetcher(pub RwSignal<bool>);

#[derive(Copy, Clone)]
pub struct HasError(pub RwSignal<bool>);

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="gctheme" />

        // sets the document title
        <Title text="Gift Card Web App" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <main class="font-poppins">

                <div class="min-h-screen">
                    // You can conditionally render the Sidebar based on the route
                    <Routes>
                        <Route path="/"  view=|| {
                            view! {
                                <NavBar />

                                    <Landing />

                            }
                        } />
                        <Route path="/*" view=NotFound />
                        <Route path="/home" view=|| {
                            view! {
                                <Sidebar>
                                    <Landing />
                                </Sidebar>
                            }
                        } />
                    </Routes>
                </div>

                //<Footer />
            </main>
        </Router>
    }
}
