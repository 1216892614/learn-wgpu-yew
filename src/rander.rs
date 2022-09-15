use web_sys::HtmlCanvasElement;
use wgpu::{Instance, Surface};
use yew::use_state;
use yew_canvas::WithRander;

use crate::wgpu_state::State;

#[derive(Clone, PartialEq)]
pub(super) struct Rander();

impl WithRander for Rander {
    fn rand(self, canvas: &HtmlCanvasElement) {
        let wasm_state = use_state(|| -> Option<State> { None });

        {
            let wasm_state = wasm_state.clone();
            let canvas = canvas.clone();

            wasm_bindgen_futures::spawn_local(async move {
                wasm_state.set(Some(State::new(&canvas).await))
            })
        }
    }
}
