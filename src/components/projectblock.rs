use yew::prelude::*;



enum Msg {
    HighLight,
    UnHighLight,
}
struct ProjectBlock;

impl Component for ProjectBlock {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
       ProjectBlock
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <>
                <img href="" /> 
                <span class="project-title">{"Lorem Ipsum"}</span>
                <span class="project-desc">{"Lorem Ipsum dolor sit amet consecteur"}</span>
            </>
        }
    }
}