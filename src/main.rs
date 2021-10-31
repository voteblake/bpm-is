use instant::{Duration, Instant};
use yew::prelude::*;

const MINUTE_IN_MICROS: u128 = Duration::from_secs(60).as_micros();

enum Msg {
    Beat,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    beats: Vec<Instant>,
}

impl Model {
    fn bpm(&self) -> Option<u128> {
        // Do not calculate until we have at least two beats
        if self.beats.len() < 2 {
            return None;
        }

        let sum_durations: u128 = self
            .beats
            .iter()
            .zip(self.beats.iter().skip(1))
            .map(|(first, second)| second.duration_since(*first).as_micros())
            .sum();

        let mean = sum_durations / self.beats.len() as u128;
        Some(MINUTE_IN_MICROS / mean)
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            beats: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Beat => {
                self.beats.push(Instant::now());

                // Maximum 16 beat moving average
                while self.beats.len() > 16 {
                    self.beats.remove(0);
                }

                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"The BPM Is"}</h1>
                <button onclick=self.link.callback(|_| Msg::Beat)>{ "Tap the Beat" }</button>
                <p>{ match self.bpm() {Some(n) => n.to_string(), None => String::from("?")} }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
