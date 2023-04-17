use yew::prelude::{function_component, html, Html};
use yewdux::prelude::*;

use crate::{layout::dashboard::DashboardLayout, pages::auth::AuthProtected, State};

#[function_component]
pub fn Dashboard() -> Html {
    let state = use_store_value::<State>();

    html! {
      <AuthProtected is_authorized={state.is_user_authorized}>
        <DashboardLayout>
        <div class="p-6 bg-gray-100">
        <div class="container grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6 mx-auto">
          <div class="bg-white p-4 rounded-lg shadow-md">
            <div class="flex items-center">
              <div class="icon bg-blue-500 text-white p-4 rounded-full mr-4">
                <i class="fas fa-users"></i>
              </div>
              <div>
                <p class="text-lg text-gray-700 font-medium">{"Users"}</p>
                <p class="text-gray-500">{"50"}</p>
              </div>
            </div>
          </div>
          <div class="bg-white p-4 rounded-lg shadow-md">
            <div class="flex items-center">
              <div class="icon bg-yellow-500 text-white p-4 rounded-full mr-4">
                <i class="fas fa-envelope"></i>
              </div>
              <div>
                <p class="text-lg text-gray-700 font-medium">{"Messages"}</p>
                <p class="text-gray-500">{"50"}</p>
              </div>
            </div>
          </div>
          <div class="bg-white p-4 rounded-lg shadow-md">
            <div class="flex items-center">
              <div class="icon bg-green-500 text-white p-4 rounded-full mr-4">
                <i class="fas fa-chart-line"></i>
              </div>
              <div>
                <p class="text-lg text-gray-700 font-medium">{"Analytics"}</p>
                <p class="text-gray-500">{"50"}</p>
              </div>
            </div>
          </div>
          <div class="bg-white p-4 rounded-lg shadow-md">
            <div class="flex items-center">
              <div class="icon bg-red-500 text-white p-4 rounded-full mr-4">
                <i class="fas fa-money-bill"></i>
              </div>
              <div>
                <p class="text-lg text-gray-700 font-medium">{"Revenue"}</p>
                <p class="text-gray-500">{"$50,000"}</p>
              </div>
            </div>
          </div>
          <div class="bg-white p-4 rounded-lg shadow-md">
            <div class="flex items-center">
              <div class="icon bg-purple-500 text-white p-4 rounded-full mr-4">
                <i class="fas fa-briefcase"></i>
              </div>
              <div>
                <p class="text-lg text-gray-700 font-medium">{"Jobs"}</p>
                <p class="text-gray-500">{"50"}</p>
              </div>
            </div>
          </div>
          <div class="bg-white p-4 rounded-lg shadow-md">
            <div class="flex items-center">
              <div class="icon bg-purple-500 text-white p-4 rounded-full mr-4">
                <i class="fas fa-briefcase"></i>
              </div>
              <div>
                <p class="text-lg text-gray-700 font-medium">{"Jobs"}</p>
                <p class="text-gray-500">{"50"}</p>
              </div>
            </div>
          </div>
        </div>
        </div>

        <div class="p-6 bg-gray-100">
        <div class="container mx-auto">
            <div class="w-full overflow-hidden rounded-lg shadow-xs">
            <div class="w-full overflow-x-auto">
                <table class="w-full whitespace-no-wrap">
                <thead>
                    <tr
                    class="text-xs font-semibold tracking-wide text-left text-gray-500 uppercase border-b dark:border-gray-700 bg-gray-50 dark:text-gray-400 dark:bg-gray-800"
                    >
                    <th class="px-4 py-3">{"Product Name"}</th>
                    <th class="px-4 py-3">{"SKU"}</th>
                    <th class="px-4 py-3">{"Price"}</th>
                    <th class="px-4 py-3">{"Quantity"}</th>
                    </tr>
                </thead>
                <tbody
                    class="bg-white divide-y dark:divide-gray-700 dark:bg-gray-800"
                >
                    <tr class="text-gray-700 dark:text-gray-400">
                    <td class="px-4 py-3">
                        <div class="flex items-center text-sm">
                        <div>
                            <p class="font-semibold">{"Product A"}</p>
                            <p class="text-xs text-gray-600 dark:text-gray-400">
                            {"Description of Product A"}
                            </p>
                        </div>
                        </div>
                    </td>
                    <td class="px-4 py-3">
                        <p class="text-sm">{"ABC123"}</p>
                    </td>
                    <td class="px-4 py-3">
                        <p class="text-sm">{"$100.00"}</p>
                    </td>
                    <td class="px-4 py-3">
                        <p class="text-sm">{"50"}</p>
                    </td>
                    </tr>
                    <tr class="text-gray-700 dark:text-gray-400">
                    <td class="px-4 py-3">
                        <div class="flex items-center text-sm">
                        <div>
                            <p class="font-semibold">{"Product B"}</p>
                            <p class="text-xs text-gray-600 dark:text-gray-400">
                            {"Description of Product B"}
                            </p>
                        </div>
                        </div>
                    </td>
                    <td class="px-4 py-3">
                        <p class="text-sm">{"DEF456"}</p>
                    </td>
                    <td class="px-4 py-3">
                        <p class="text-sm">{"$200.00"}</p>
                    </td>
                    <td class="px-4 py-3">
                        <p class="text-sm">{"25"}</p>
                    </td>
                    </tr>
                </tbody>
                </table>
                </div>
            </div>
            </div>
        </div>
        </DashboardLayout>
      </AuthProtected>
    }
}
