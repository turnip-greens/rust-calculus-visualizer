use web_sys::HtmlInputElement;
use yew::prelude::*;

fn clamp(value: String, min: i32, max: i32) -> i32 {
    let value: i32 = value.parse().unwrap_or(min);
    value.max(min).min(max)
}


#[component]
fn App() -> Html {
    let samples = use_state(|| 1);
    let area_under_curve = use_state(|| 0.0f32);
    let on_slider_input = {
        let value = samples.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            value.set(clamp(input.value(), 1, 128));
        })
    };

    let on_number_input = {
        let value = samples.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            value.set(clamp(input.value(), 1, 128));
        })
    };

    let run_compute = {
        let area = area_under_curve.clone();
        Callback::from(move |_: MouseEvent| {
            // Compute the area under the curve here
            area.set(0.0); // Replace with actual computation
        })
    };

    html! {
        <div>
            <p>{ format!("Samples: {} x 2048 = {}", *samples, *samples * 2048) }</p>
            <input
                type="number"
                min="1"
                max="128"
                step="1"
                value={(*samples).to_string()}
                oninput={on_number_input}
            />
            <input
                type="range"
                min="1"
                max="128"
                step="1"
                value={(*samples).to_string()}
                oninput={on_slider_input}
            />
            <p>{ format!("Area: {}", *area_under_curve) }</p>
            <button onclick={run_compute}>{"Compute Area"}</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}