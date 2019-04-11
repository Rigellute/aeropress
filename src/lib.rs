use std::time::Duration;
use yew::services::{IntervalService, Task};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    interval: IntervalService,
    callback_tick: Callback<()>,
    state: State,
    job: Option<Box<Task>>,
}

pub struct State {
    count: i32,
    current_step: usize,
    has_started: bool,
}

pub enum Msg {
    StartTimer,
    Tick,
    CancelTimer,
    IncrementStep,
    ToggleStart,
}

pub struct Step {
    display_at_count: i32,
    instruction: &'static str,
    requires_action: bool,
}

const INSTRUCTIONS: [Step; 8] = [
    Step {
        display_at_count: 0,
        instruction: "Set your AeroPress inverted and wet double paper filters with hot water.",
        requires_action: true,
    },
    Step {
        display_at_count: 0,
        instruction: "Set your water temperature at 81 celsius degrees, put the coffee in and fill up with 70 grams of water.",
        requires_action: true,
    },
    Step {
        display_at_count: 0,
        instruction: "Start timer and swirl your AeroPress around for 15 seconds.",
        requires_action: true,
    },
    Step {
        display_at_count: 15,
        instruction: "Fill up with the remaining amount of water",
        requires_action: false,
    },
    Step {
        display_at_count: 60,
        instruction: "Put the cap on",
        requires_action: false,
    },
    Step {
        display_at_count: 80,
        instruction: "Invert the areopress",
        requires_action: false,
    },
    Step {
        display_at_count: 90,
        instruction: "Start plunging",
        requires_action: false,
    },
    Step {
        display_at_count: 105,
        instruction: "Done!",
        requires_action: false,
    }
];

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let state = State {
            has_started: false,
            count: 0,
            current_step: 0,
        };

        Model {
            state,
            callback_tick: link.send_back(|_| Msg::Tick),
            interval: IntervalService::new(),
            job: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleStart => {
                self.state.has_started = !self.state.has_started;
            }
            Msg::IncrementStep => {
                self.state.current_step += 1;
            }
            Msg::StartTimer => {
                let handle = self
                    .interval
                    .spawn(Duration::from_secs(1), self.callback_tick.clone());
                self.job = Some(Box::new(handle));
            }
            Msg::Tick => {
                self.state.count += 1;
            }
            Msg::CancelTimer => {
                if let Some(mut task) = self.job.take() {
                    task.cancel();
                };
                self.state.count = 0;
                self.job = None;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="container",>
                <section class="header",>
                    <h1 class="title",>{ "Aeropress timer" }</h1>
                    <i data-feather="coffee",></i>
                    <p>{ "Brew the perfect cup every time" }</p>
                </section>
                <div class="row-buttons",>
                    <button onclick=|_| Msg::ToggleStart,>{ "Begin" }</button>
                    <button onclick=|_| Msg::CancelTimer ,>{ "Stop" }</button>
                </div>
                {self.view_instruction()}
            </div>
        }
    }
}

impl Model {
    fn view_instruction(&self) -> Html<Model> {
        let has_job = self.job.is_some();
        if has_job {
            html! {
                <div class="timer",>{self.state.count}</div>
            }
        } else if self.state.has_started {
            let step = &INSTRUCTIONS[self.state.current_step];
            html! {
                <>
                    <p>{ step.instruction }</p>
                    <button onclick=|_| Msg::IncrementStep,>{"Next"}</button>
                </>
            }
        } else {
            html! {
                <div></div>
            }
        }
    }
}
