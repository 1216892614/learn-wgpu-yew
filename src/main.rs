use web_sys::WebGl2RenderingContext;
use yew::prelude::*;
use yew_canvas::Canvas;

mod rander;
mod wgpu_state;

#[function_component(App)]
fn app() -> Html {
    html!(
        <Canvas<WebGl2RenderingContext , rander::Rander>
            //Just use style, canvas can suit automatically.
            style="
                width: 100%;
                height: 100%;
            "
            rander={Box::new(rander::Rander())}
        />
    )
}

fn main() {
    yew::start_app::<App>();
}
