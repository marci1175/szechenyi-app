use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;
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

struct TextArea {
    inner: String,

    last_key_code: u32,
}

#[derive(Debug, Default, Properties, PartialEq)]
struct TextAreaProperties {
    /// The name of the ```TextArea```
    name: String,

    /// The id of the ```TextArea```
    id: String,

    /// The size is in rows and columns. ```(Rows, Columns)```
    size: (i32, i32),
}

enum TextAreaMessage {
    /// The keycode of the button pressed while the ```TextArea``` is in focus
    KeyPressed(u32),

    ValueUpdate(String),
}

impl TextArea {
    pub fn get_inner(&self) -> String {
        self.inner.clone()
    }

    pub fn get_last_key_code(&self) -> u32 {
        self.last_key_code
    }
}

impl Component for TextArea {
    type Message = TextAreaMessage;

    type Properties = TextAreaProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            inner: String::new(),
            last_key_code: 0,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text_edit_button_callback = ctx.link().callback(|event: KeyboardEvent| {
            TextAreaMessage::KeyPressed(event.key_code())
        });

        let text_edit_value_callback: Callback<InputEvent> = ctx.link().callback(|event: InputEvent| {
            let html_elem: HtmlTextAreaElement = event.target_unchecked_into();
            TextAreaMessage::ValueUpdate(html_elem.value())
        });

        html!(
            <>
                <textarea name={ctx.props().name.clone()} id={ctx.props().id.clone()} rows={ctx.props().size.0.to_string()} columns={ctx.props().size.1.to_string()} onkeypress={text_edit_button_callback} oninput={text_edit_value_callback}></textarea>
            </>
        )
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextAreaMessage::KeyPressed(code) => {
                self.last_key_code = code;
            },
            TextAreaMessage::ValueUpdate(current_inner) => {
                self.inner = current_inner;
            },
        }

        true
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <div>
            <center>
            <Button />
            <TextArea name="text_area" id="text_area" size={(10, 20)}/>
            </center>
        </div>
    )
}