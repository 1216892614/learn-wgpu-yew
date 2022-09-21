use gloo::console::log;
use web_sys::WebGl2RenderingContext;
use yew::prelude::*;
use yew_canvas::Canvas;

mod rander;
mod wgpu_state;

#[function_component(App)]
fn app() -> Html {
    let is_hold_state = use_state(|| false);
    let cursor_move_state = use_state(|| (0i32, 0i32));
    let cursor_to_state = use_state(|| (0i32, 0i32));

    let onmousedown = {
        let is_hold_state = is_hold_state.clone();
        let cursor_move_state = cursor_move_state.clone();
        Callback::from(move |e: MouseEvent| {
            let cursor = (e.screen_x(), e.screen_y());
            cursor_move_state.set(cursor);

            is_hold_state.set(true);
        })
    };

    let onmouseup = {
        let is_hold_state = is_hold_state.clone();
        Callback::from(move |_| {
            is_hold_state.set(false);
        })
    };

    let onmousemove = {
        let cursor_befor = *cursor_move_state.clone();
        let cursor_to_state = cursor_to_state.clone();

        Callback::from(move |e: MouseEvent| {
            if *is_hold_state {
                let cursor = (e.screen_x(), e.screen_y());
                cursor_to_state.set((cursor_befor.0 - cursor.0, cursor_befor.1 - cursor.1));
            }
        })
    };

    let rander = rander::Rander {
        cursor_to: *cursor_to_state,
    };

    html!(
        <div
            {onmousedown}
            {onmouseup}
            {onmousemove}
            style="
                width: 100%;
                height: 100%;
            "
        >
            <Canvas<WebGl2RenderingContext , rander::Rander>
                //Just use style, canvas can suit automatically.
                style="
                    width: 100%;
                    height: 100%;
                "
                rander={Box::new(rander)}
            />
        </div>
    )
}

fn main() {
    yew::start_app::<App>();
}
