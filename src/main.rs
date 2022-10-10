use yew::{prelude::*, virtual_dom::AttrValue};

mod components;
use components::{sociallinks as sl, descelevator as de, techtitle as tt};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div class = "title-card">
                <sl::SocialLinks />
                <span class="name-title">{"CAOLAN"}</span>
            </div>
            <de::DescElevator/>

            <div class = "projects">
                <tt::TechTitle title={tt::Tech::JavaScript}/>
                <tt::TechTitle title={tt::Tech::Java}/>
                <tt::TechTitle title={tt::Tech::Unity}/>
                <tt::TechTitle title={tt::Tech::CSharp}/>
                <tt::TechTitle title={tt::Tech::Bevy}/>
                <tt::TechTitle title={tt::Tech::Rust}/>
            </div>


        </>
    }
}


fn main() {
    yew::start_app::<App>();
}