use std::time::Duration;
use yew::services::{ConsoleService, IntervalService, Task};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    interval: IntervalService,
    callback_tick: Callback<()>,
    state: State,
    job: Option<Box<Task>>,
    console: ConsoleService,
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
    next_instruction: &'static str,
    requires_action: Msg,
}

const INSTRUCTIONS: [Step; 8] = [
    Step {
        display_at_count: 0,
        instruction: "Set your AeroPress inverted and wet double paper filters with hot water.",
        next_instruction: "",
        requires_action: Msg::IncrementStep,
    },
    Step {
        display_at_count: 0,
        instruction: "Set your water temperature at 81 celsius degrees, put the coffee in and fill up with 70 grams of water.",
        next_instruction: "",
        requires_action: Msg::IncrementStep,
    },
    Step {
        display_at_count: 0,
        instruction: "Start timer and swirl your AeroPress around for 15 seconds.",
        next_instruction: "",
        requires_action: Msg::StartTimer,
    },
    Step {
        display_at_count: 15,
        instruction: "Fill up with the remaining amount of water.",
        next_instruction: "Wait until 60 seconds, then put the cap on.",
        requires_action: Msg::IncrementStep,
    },
    Step {
        display_at_count: 60,
        instruction: "Put the cap on",
        next_instruction: "Wait until 80 seconds, then invert.",
        requires_action: Msg::IncrementStep,
    },
    Step {
        display_at_count: 80,
        instruction: "Invert the areopress.",
        next_instruction: "Wait until 90 seconds and start pluging.",
        requires_action: Msg::IncrementStep,
    },
    Step {
        display_at_count: 90,
        instruction: "Start plunging.",
        next_instruction: "Aim to finish with 15 seconds.",
        requires_action: Msg::IncrementStep,
    },
    Step {
        display_at_count: 105,
        instruction: "Done!",
        next_instruction: "",
        requires_action: Msg::CancelTimer,
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

        let mut console = ConsoleService::new();
        console.log("Ready to make coffee");

        Model {
            console,
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
                let step = &INSTRUCTIONS.get(self.state.current_step + 1);
                match step {
                    Some(some_step) => {
                        if some_step.display_at_count == self.state.count {
                            self.state.current_step += 1;
                        };
                    }
                    None => {}
                };
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
                    <h1 class="title",>{ "Aeropress timer" } </h1>
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
        let empty_node: Html<Self> = html! {
            <div></div>
        };

        let timer_html: Html<Self> = match self.job.is_some() {
            true => html! {
                <div class="timer",>{self.state.count}</div>
            },
            false => empty_node,
        };

        if self.state.has_started {
            let step = &INSTRUCTIONS.get(self.state.current_step);
            match step {
                Some(some_step) => {
                    let step_button = match some_step.requires_action {
                        Msg::IncrementStep => {
                            html! { <button onclick=|_| Msg::IncrementStep,>{"Next"}</button>}
                        }
                        Msg::StartTimer => {
                            html! { <button onclick=|_| Msg::StartTimer,>{"Start Timer"}</button>}
                        }
                        _ => html! {</>},
                    };
                    html! {
                        <>
                            <p>{ some_step.instruction }</p>
                            <p>{ some_step.next_instruction }</p>
                            {step_button}
                            {timer_html}
                        </>
                    }
                }
                None => html! {
                    <div></div>
                },
            }
        } else {
            html! {
                <div></div>
            }
        }
    }
}
