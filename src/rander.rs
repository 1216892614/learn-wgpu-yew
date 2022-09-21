pub(super) mod camera;
pub(super) mod texture;
pub(super) mod vertex;

use gloo::console::log;
use once_cell::sync::OnceCell;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlCanvasElement;
use yew_canvas::WithRander;

use crate::wgpu_state::State;

#[derive(Clone, PartialEq)]
pub(super) struct Rander {
    pub(crate) cursor_to: (i32, i32),
}

static mut WGPU_STATE: OnceCell<State> = OnceCell::new();

impl WithRander for Rander {
    fn rand(self, canvas: &HtmlCanvasElement) {
        let canvas = canvas.clone();
        let (height, width) = (canvas.height(), canvas.width());

        unsafe {
            if WGPU_STATE.get().is_none() {
                spawn_local(async move {
                    let state = State::new(&canvas).await;
                    WGPU_STATE.get_or_init(|| state.unwrap());

                    //render pass per 17 ms
                    gloo::timers::callback::Interval::new(17, || {
                        WGPU_STATE.get().unwrap().render().unwrap();
                    })
                    .forget();
                });
            }

            if !WGPU_STATE.get().is_none() {
                WGPU_STATE
                    .get_mut()
                    .unwrap()
                    .update(width, height, self.cursor_to);
                WGPU_STATE.get().unwrap().render().unwrap();
            }
        }
    }
}
