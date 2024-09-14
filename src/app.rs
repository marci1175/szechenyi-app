use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug)]
enum ButtonMessage {
    LeftClicked,
    MiddleClicked,
    RightClicked,

    Forward,
    Backward,
}

#[derive(Debug, Default, Properties, PartialEq)]
struct Button {
    counter: i32,
}

fn get_button_click_type(event: MouseEvent) -> ButtonMessage {
    match event.button() {
        0 => ButtonMessage::LeftClicked,
        1 => ButtonMessage::MiddleClicked,
        2 => ButtonMessage::RightClicked,
        3 => ButtonMessage::Backward,
        4 => ButtonMessage::Forward,
        
        _ => unreachable!(),
    }
}

impl Component for Button {
    fn create(_ctx: &Context<Self>) -> Self {
        Self { counter: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let displayed_string = format!("Click: {}", self.counter);

        let button_callback = ctx.link().callback(|event| {get_button_click_type(event)});

        html!(
            <button onContextmenu = "return false;" onmousedown={button_callback}>
                { displayed_string }
            </button>
        )
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ButtonMessage::LeftClicked => {
                self.counter += 1;

                true
            },
            ButtonMessage::MiddleClicked => false,
            ButtonMessage::RightClicked => {
                self.counter -= 1;

                true
            },
            ButtonMessage::Forward => false,
            ButtonMessage::Backward => false,
        }
    }
    
    type Message = ButtonMessage;
    
    type Properties = ();
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <div>
            <center>
                <Button />
            </center>
        </div>
    )
}