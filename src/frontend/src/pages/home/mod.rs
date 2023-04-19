use yew::prelude::{function_component, html, Html, Callback};
use yew_router::prelude::*;

use crate::{layout::website::WebsiteLayout, app::Route};

#[function_component]
pub fn Home() -> Html {
    let navigator = use_navigator().unwrap();
    let go_to_dashboard = Callback::from(move |_| navigator.push(&Route::Dashboard));

    html! {
        <WebsiteLayout>
            <section class="relative py-20">
                <div class="absolute inset-0 w-full h-full bg-gray-900 opacity-75"></div>
                    <div class="container mx-auto px-4 py-12">
                        <div class="relative flex flex-col items-center justify-center h-full">
                        <h1 class="text-5xl font-bold text-center text-white mb-6">{"Introducing Inventree"}</h1>
                        <p class="text-2xl text-center text-gray-300 mb-8">{"The ultimate inventory management system for your business"}</p>
                        <a onclick={go_to_dashboard} class="inline-block bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-4 px-8 rounded-full">{"Go to Dashboard"}</a>
                        </div>
                    </div>
                <img src="assets/img/home-hero.jpg" alt="Inventree Hero Image" class="absolute inset-0 object-cover w-full h-full" loading="lazy" />
            </section>

            <section class="bg-white py-10">
                <div class="container mx-auto">
                    <h2 class="text-4xl font-bold mb-10 text-center">{"Features"}</h2>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-10">
                    <div class="p-6 bg-gray-100 rounded-lg">
                        <i class="mdi mdi-lock text-4xl text-gray-500 mb-5"></i>
                        <h3 class="text-2xl font-bold mb-5">{"Secure"}</h3>
                        <p class="text-gray-500">{"Our platform is designed with security in mind. We use the latest encryption and security technologies to keep your data safe and secure."}</p>
                    </div>
                    <div class="p-6 bg-gray-100 rounded-lg">
                        <i class="mdi mdi-chart-bar text-4xl text-gray-500 mb-5"></i>
                        <h3 class="text-2xl font-bold mb-5">{"Analytics"}</h3>
                        <p class="text-gray-500">{"Get insights into your data with our powerful analytics tools. Analyze your inventory trends and make data-driven decisions."}</p>
                    </div>
                    <div class="p-6 bg-gray-100 rounded-lg">
                        <i class="mdi mdi-sync text-4xl text-gray-500 mb-5"></i>
                        <h3 class="text-2xl font-bold mb-5">{"Integration"}</h3>
                        <p class="text-gray-500">{"Integrate Inventree with your favorite e-commerce platforms and marketplaces. Sync inventory levels and streamline your operations."}</p>
                    </div>
                    <div class="p-6 bg-gray-100 rounded-lg">
                        <i class="mdi mdi-shield-check text-4xl text-gray-500 mb-5"></i>
                        <h3 class="text-2xl font-bold mb-5">{"Compliance"}</h3>
                        <p class="text-gray-500">{"Stay compliant with industry regulations and standards. Our platform is designed to help you comply with all necessary regulations."}</p>
                    </div>
                    <div class="p-6 bg-gray-100 rounded-lg">
                        <i class="mdi mdi-account-multiple text-4xl text-gray-500 mb-5"></i>
                        <h3 class="text-2xl font-bold mb-5">{"Collaboration"}</h3>
                        <p class="text-gray-500">{"Collaborate with your team in real-time. Our platform allows you to share inventory data with your team members and work together seamlessly."}</p>
                    </div>
                    <div class="p-6 bg-gray-100 rounded-lg">
                        <i class="mdi mdi-cogs text-4xl text-gray-500 mb-5"></i>
                        <h3 class="text-2xl font-bold mb-5">{"Customization"}</h3>
                        <p class="text-gray-500">{"Customize Inventree to fit your business needs. Our platform offers a wide range of customization options to help you tailor the platform to your requirements."}</p>
                    </div>
                    </div>
                </div>
            </section>

            <section class="py-20 bg-gray-100">
                <div class="container mx-auto px-4">
                <h2 class="text-4xl font-bold text-center text-gray-800 mb-8">{"Pricing"}</h2>
                    <div class="flex flex-col md:flex-row justify-center">
                        <div class="bg-white rounded-lg shadow-md mx-4 mb-8 md:mb-0 max-w-sm">
                        <div class="py-8 px-6">
                            <h3 class="text-2xl font-bold text-center text-gray-800 mb-4">{"Starter"}</h3>
                            <div class="flex items-center justify-center text-5xl font-bold text-gray-800 mb-6">
                            <span class="mr-2">{"$"}</span>{"9"}<span class="text-xl">{"/mo"}</span>
                            </div>
                            <ul class="text-lg text-gray-600 mb-6">
                            <li>{"10 Products"}</li>
                            <li>{"100 Orders/month"}</li>
                            <li>{"Basic Analytics"}</li>
                            </ul>
                            <a href="#" class="block bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-4 px-8 rounded-full text-center">{"Sign Up"}</a>
                        </div>
                        </div>
                        <div class="bg-white rounded-lg shadow-md mx-4 mb-8 md:mb-0 max-w-sm">
                        <div class="py-8 px-6">
                            <h3 class="text-2xl font-bold text-center text-gray-800 mb-4">{"Professional"}</h3>
                            <div class="flex items-center justify-center text-5xl font-bold text-gray-800 mb-6">
                            <span class="mr-2">{"$"}</span>{"29"}<span class="text-xl">{"/mo"}</span>
                            </div>
                            <ul class="text-lg text-gray-600 mb-6">
                            <li>{"100 Products"}</li>
                            <li>{"1000 Orders/month"}</li>
                            <li>{"Advanced Analytics"}</li>
                            <li>{"Premium Support"}</li>
                            </ul>
                            <a href="#" class="block bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-4 px-8 rounded-full text-center">{"Sign Up"}</a>
                        </div>
                        </div>
                        <div class="bg-white rounded-lg shadow-md mx-4">
                        <div class="py-8 px-6">
                            <h3 class="text-2xl font-bold text-center text-gray-800 mb-4">{"Enterprise"}</h3>
                            <div class="flex items-center justify-center text-5xl font-bold text-gray-800 mb-6">
                            <span class="mr-2">{"$"}</span>{"99"}<span class="text-xl">{"/mo"}</span>
                            </div>
                            <ul class="text-lg text-gray-600 mb-6">
                            <li>{"Unlimited Products"}</li>
                            <li>{"Unlimited Orders/month"}</li>
                            <li>{"Advanced Analytics"}</li>
                            <li>{"Premium Support"}</li>
                            <li>{"Dedicated Account Manager"}</li>
                            </ul>
                            <a href="#" class="block bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-4 px-8 rounded-full text-center">{"Sign Up"}</a>
                        </div>
                    </div>
                </div>
                </div>
            </section>

            <section class="py-20 bg-gray-100">
                <div class="container mx-auto px-4">
                    <h2 class="text-4xl font-bold text-center text-gray-800 mb-8">{"Testimonials"}</h2>
                    <div class="flex flex-wrap justify-center">

                    <div class="w-full md:w-1/2 lg:w-1/3 p-4">
                        <div class="bg-white rounded-lg shadow-md overflow-hidden">
                        <div class="p-4">
                            <div class="flex items-center mb-4">
                            <div class="w-12 h-12 mr-4">
                                <img class="w-full h-full object-cover rounded-full" src="https://source.unsplash.com/100x100/?person" alt="Person" />
                            </div>
                            <div>
                                <h3 class="text-lg font-bold text-gray-800">{"John Doe"}</h3>
                                <p class="text-gray-600">{"CEO, ABC Company"}</p>
                            </div>
                            </div>
                            <p class="text-gray-600">{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis vitae bibendum ante, at hendrerit ipsum. Morbi vestibulum quam a elit venenatis, a sagittis justo molestie."}</p>
                        </div>
                        </div>
                    </div>

                    <div class="w-full md:w-1/2 lg:w-1/3 p-4">
                        <div class="bg-white rounded-lg shadow-md overflow-hidden">
                        <div class="p-4">
                            <div class="flex items-center mb-4">
                            <div class="w-12 h-12 mr-4">
                                <img class="w-full h-full object-cover rounded-full" src="https://source.unsplash.com/100x100/?person" alt="Person" />
                            </div>
                            <div>
                                <h3 class="text-lg font-bold text-gray-800">{"Jane Smith"}</h3>
                                <p class="text-gray-600">{"COO, XYZ Company"}</p>
                            </div>
                            </div>
                            <p class="text-gray-600">{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis vitae bibendum ante, at hendrerit ipsum. Morbi vestibulum quam a elit venenatis, a sagittis justo molestie."}</p>
                            </div>
                            </div>
                        </div>
                        </div>
                    </div>
            </section>
        </WebsiteLayout>
    }
}
