use yew::prelude::*;

#[function_component(SocialLinks)]
pub fn social_links() -> Html {

    html! {
        <>
            <a href="https://www.twitter.com" class="social-bubble">
                <img src="resources/images/twitter.svg" alt="Twitter Link"  />
            </a>

            <a href="https://www.github.com/CaolanBarron" class="social-bubble">
                <img src="resources/images/github-icon.svg" alt="Github Link" />
            </a>

            <a href="https://www.linkedin.com/in/caolan-barron-965a66206" class="social-bubble">
                <img src="resources/images/linkedin-icon.svg" alt="LinkedIn List" />
            </a>

            <a href="blog" class="social-bubble">
                <img src="resources/images/notebook-svgrepo-com.svg" />
            </a>
        </>
    }

}