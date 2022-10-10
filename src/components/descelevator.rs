use std::vec;
use yew::{prelude::*};

const TITLES1: &'static [&'static str] = &["Driven", "Creative", "Problem solver", "Team worker"];
const TITLES2: &'static [&'static str] = &["Software designer","Games designer","Website designer","Frontend designer", "Backend designer"];


pub enum Msg {
    RandomT1,
    RandomT2,
}

pub struct DescElevator {
    t1: String,
    t2: String,

    t1_iter: usize,
    t2_iter: usize,
}



impl Component for DescElevator {
    type Message = Msg;
    type Properties = ();

    

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            t1: TITLES1[0].to_string(),
            t2: TITLES2[0].to_string(),

            t1_iter: 0,
            t2_iter: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RandomT1 => {
                self.t1_iter = (self.t1_iter + 1) % TITLES1.len();
                self.t1 = TITLES1[self.t1_iter].to_string();
                true
            }

            Msg::RandomT2 => {
                self.t2_iter = (self.t2_iter + 1) % TITLES2.len();
                self.t2 = TITLES2[self.t2_iter].to_string();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div id = "elev-container">
                <span id = "elev1" onanimationiteration={link.callback(|_| Msg::RandomT1)}>{&self.t1}</span>
                <span id = "elev2" onanimationiteration={link.callback(|_| Msg::RandomT2)}>{&self.t2}</span>
            </div>
        }
    }
}