use web_sys::WebGl2RenderingContext;
use yew::prelude::*;
use yew_canvas::Canvas;

mod rander;
mod wgpu_state;

#[function_component(App)]
fn app() -> Html {
    html!(
        <div
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
                rander={Box::new(rander::Rander())}
            />
        </div>
    )
}

fn main() {
    yew::start_app::<App>();
}
