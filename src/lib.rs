use std::time::Duration;
use yew::services::{ConsoleService, IntervalService, Task};
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

// While debugging it can be useful to speed through the timer - but this value should be 1000ms
const TIMER_SPEED: u64 = 1000;

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
    Restart,
}

pub struct Step {
    display_at_count: i32,
    instruction: &'static str,
    next_instruction: &'static str,
    on_click: Msg,
}

// TODO: Put these instructions into a json file
const INSTRUCTIONS: [Step; 8] = [
    Step {
        display_at_count: 0,
        instruction: "Set your AeroPress inverted and wet a paper filters with hot water.",
        next_instruction: "",
        on_click: Msg::IncrementStep,
    },
    Step {
        display_at_count: 0,
        instruction: "Set your water temperature at 81 celsius degrees, put in one scoop of the coffee and fill up with 70 grams of water.",
        next_instruction: "",
        on_click: Msg::IncrementStep,
    },
    Step {
        display_at_count: 0,
        instruction: "Start timer and swirl your AeroPress around for 15 seconds.",
        next_instruction: "",
        on_click: Msg::StartTimer,
    },
    Step {
        display_at_count: 15,
        instruction: "Fill up with the remaining amount of water.",
        next_instruction: "Wait until 60 seconds, then put the cap on.",
        on_click: Msg::IncrementStep,
    },
    Step {
        display_at_count: 60,
        instruction: "Put the cap on",
        next_instruction: "Wait until 80 seconds, then invert.",
        on_click: Msg::IncrementStep,
    },
    Step {
        display_at_count: 80,
        instruction: "Invert the areopress.",
        next_instruction: "Wait until 90 seconds and start pluging.",
        on_click: Msg::IncrementStep,
    },
    Step {
        display_at_count: 90,
        instruction: "Start plunging.",
        next_instruction: "Aim to finish within 15 seconds.",
        on_click: Msg::IncrementStep,
    },
    Step {
        display_at_count: 105,
        instruction: "Done! Enjoy your coffee.",
        next_instruction: "",
        on_click: Msg::CancelTimer,
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
            Msg::Restart => {
                self.state.has_started = false;
                self.state.count = 0;
                self.state.current_step = 0;
            }
            Msg::ToggleStart => {
                self.state.has_started = !self.state.has_started;
            }
            Msg::IncrementStep => {
                self.state.current_step += 1;
            }
            Msg::StartTimer => {
                self.console.log("Timer has started");
                let handle = self.interval.spawn(
                    Duration::from_millis(TIMER_SPEED),
                    self.callback_tick.clone(),
                );
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

                if self.state.current_step == INSTRUCTIONS.len() - 1 {
                    self.cancel_timer();
                }
                self.state.count += 1;
            }
            Msg::CancelTimer => {
                self.cancel_timer();
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
                    <h1 class="title",>{ "AeroPress Timer" } </h1>
                    <i data-feather="coffee",></i>
                    <p>{ "Brew the perfect cup every time." }</p>
                </section>
                {self.view_instruction()}
            </div>
        }
    }
}

impl Model {
    fn cancel_timer(&mut self) {
        if let Some(mut task) = self.job.take() {
            task.cancel();
        };
        self.state.count = 0;
        self.job = None;
    }

    fn restart_button(&self) -> Html<Model> {
        let is_done = self.state.current_step == INSTRUCTIONS.len() - 1;
        if !self.job.is_some() && is_done {
            html! {
                <button onclick=|_| Msg::Restart,>{ "Restart" }</button>
            }
        } else {
            empty_node()
        }
    }

    fn view_instruction(&self) -> Html<Model> {
        if !self.state.has_started {
            return html! {
                <button onclick=|_| Msg::ToggleStart,>{ "Begin" }</button>
            };
        }

        let timer_html: Html<Self> = match self.job.is_some() {
            true => html! {
                <div class="timer",>{self.state.count}</div>
            },
            false => empty_node(),
        };

        if self.state.has_started {
            let step = &INSTRUCTIONS.get(self.state.current_step);
            match step {
                Some(some_step) => {
                    let step_button = match some_step.on_click {
                        Msg::IncrementStep => {
                            html! { <button onclick=|_| Msg::IncrementStep,>{"Next"}</button>}
                        }
                        Msg::StartTimer => {
                            html! { <button onclick=|_| Msg::StartTimer,>{"Start Timer"}</button>}
                        }
                        _ => empty_node(),
                    };
                    html! {
                        <>
                            {step_button}
                            {self.restart_button()}
                            <p>{ some_step.instruction } {" "} { some_step.next_instruction }</p>
                            {timer_html}
                        </>
                    }
                }
                None => empty_node(),
            }
        } else {
            empty_node()
        }
    }
}

fn empty_node() -> Html<Model> {
    html! {
        <div></div>
    }
}
