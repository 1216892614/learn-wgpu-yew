use web_sys::HtmlCanvasElement;
use wgpu::{Instance, Surface};
use yew::use_state;
use yew_canvas::WithRander;

use crate::wgpu_state::State;

#[derive(Clone, PartialEq)]
pub(super) struct Rander();

impl WithRander for Rander {
    fn rand(self, canvas: &HtmlCanvasElement) {
        {
            let canvas = canvas.clone();
            let (height, width) = (canvas.height(), canvas.width());

            wasm_bindgen_futures::spawn_local(async move {
                let mut wasm_state = State::new(&canvas).await;
                wasm_state.render(height, width).unwrap();
            })
        }
    }
}
