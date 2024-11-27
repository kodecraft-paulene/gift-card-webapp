use crate::components::layouts::footer::Footer;
use leptos::*;
#[component]
pub fn Returnedorder() -> impl IntoView {
    view! {
        <div class="min-h-screen">
        <div class="flex flex-col gap-y-6 mx-10 ">
           <div class="mt-10">
              <h3 class="text-2xl font-semibold text-primary">
                 Returned Orders
              </h3>
           </div>

           <div class="space-y-6">

           </div>
        </div>
        </div>
        <Footer/>
    }
}
