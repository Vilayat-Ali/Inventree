use crate::components::{footer::Footer, navbar::website::Navbar};
use yew::prelude::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn WebsiteLayout(props: &Props) -> Html {
    html! {
        <div>
            <Navbar/>
                    {props.children.iter().collect::<Html>()}
            <Footer />
        </div>
    }
}
