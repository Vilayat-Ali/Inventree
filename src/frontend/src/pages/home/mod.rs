use yew::prelude::{function_component, html, Html};

use crate::layout::website::WebsiteLayout;

#[function_component]
pub fn Home() -> Html {
    html! {
        <WebsiteLayout>
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content flex-col lg:flex-row-reverse">
            <img src="assets/img/home-hero-img.jpg" class="max-w-sm rounded-lg shadow-2xl" />
            <div>
                <h1 class="text-5xl font-bold">{"Inventree"}</h1>
                <h2 class="text-xl">{"The powerful Inventory Management System for powerful businesses!"}</h2>
                <p class="py-6">{"Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi."}</p>
                <button class="btn btn-primary">{"Get Started"}</button>
            </div>
            </div>
        </div>

        <ul class="steps steps-vertical lg:steps-horizontal">
            <li class="step step-primary">{"Register"}</li>
            <li class="step step-primary">{"Choose plan"}</li>
            <li class="step">{"Purchase"}</li>
            <li class="step">{"Receive Product"}</li>
        </ul>
        </WebsiteLayout>
    }
}
