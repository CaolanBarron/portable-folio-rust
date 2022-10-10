use std::fmt::Display;

use yew::prelude::*;

#[derive(PartialEq)]
pub enum Tech {
    JavaScript,
    Unity,
    Java,
    CSharp,
    Rust,
    React,
    Bevy,
    Python
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: Tech,
}

impl Display for Tech {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Tech::JavaScript => write!(f, "JavaScript"),
            Tech::Unity => write!(f, "Unity"),
            Tech::Java => write!(f, "Java"),
            Tech::CSharp => write!(f, "C#"),
            Tech::Rust => write!(f,"Rust"),
            Tech::React => write!(f, "React"),
            Tech::Bevy => write!(f, "Bevy"),
            Tech::Python => write!(f, "Python")
        }
    }
}

pub struct TechTitle;

impl Component for TechTitle {
    type Message=();
    type Properties= Props;

    fn create(ctx: &Context<Self>, ) -> Self {
        TechTitle
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div class = "tech-title">
                <span>{&ctx.props().title} </span>
                <span class="tech-ball"/>
            </div>
        }
    }
}