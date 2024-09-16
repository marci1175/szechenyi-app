use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Debug)]
pub enum TitleButtonMessage {
    LeftClicked,
    MiddleClicked,
    RightClicked,

    Forward,
    Backward,
}

#[derive(Debug, Default, Properties, PartialEq)]
pub struct TitleButton;

#[derive(Properties, PartialEq)]
pub struct TitleButtonProperties {
    /// The label of the title button
    pub label: String,

    /// The common state this title button modifies
    pub common_state: UseStateHandle<TabState>,

    /// The value this button has, this is what itll overwrite ```common_state``` with
    pub value: TabState,
}

fn get_title_button_click_type(event: MouseEvent) -> TitleButtonMessage {
    match event.button() {
        0 => TitleButtonMessage::LeftClicked,
        1 => TitleButtonMessage::MiddleClicked,
        2 => TitleButtonMessage::RightClicked,
        3 => TitleButtonMessage::Backward,
        4 => TitleButtonMessage::Forward,

        _ => unreachable!(),
    }
}

impl Component for TitleButton {
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title_button_callback = ctx
            .link()
            .callback(|event| get_title_button_click_type(event));

        let button = html!(
            <button onContextmenu = "return false;" onmousedown={title_button_callback}>
                { ctx.props().label.clone() }
            </button>
        );

        let is_active = *ctx.props().common_state == ctx.props().value;

        if is_active {
            html!(
                <div class="activebutton">
                    {button}
                </div>
            )
        } else {
            button
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TitleButtonMessage::LeftClicked => {
                ctx.props().common_state.set(ctx.props().value.clone());

                true
            }
            _ => false,
        }
    }

    type Message = TitleButtonMessage;

    type Properties = TitleButtonProperties;
}

pub struct TextArea {
    last_key_code: u32,
}

#[derive(Debug, Properties, PartialEq)]
pub struct TextAreaProperties {
    /// The name of the ```TextArea```
    pub name: String,

    /// The id of the ```TextArea```
    pub id: String,

    /// The size is in rows and columns. ```(Rows, Columns)```
    pub size: (i32, i32),

    /// The handle to the inner buffer of this ```TextArea``` instance
    pub text_buffer: UseStateHandle<String>,
}

pub enum TextAreaMessage {
    /// The keycode of the Titlebutton pressed while the ```TextArea``` is in focus
    KeyPressed(u32),

    /// This enum's inner value holds the text from the ```textarea``` instance and is sent at every text edit
    ValueUpdate(String),
}

impl Component for TextArea {
    type Message = TextAreaMessage;

    type Properties = TextAreaProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { last_key_code: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text_edit_button_callback = ctx
            .link()
            .callback(|event: KeyboardEvent| TextAreaMessage::KeyPressed(event.key_code()));

        let text_edit_value_callback: Callback<InputEvent> =
            ctx.link().callback(|event: InputEvent| {
                let html_elem: HtmlTextAreaElement = event.target_unchecked_into();
                TextAreaMessage::ValueUpdate(html_elem.value())
            });

        html!(
            <textarea name={ctx.props().name.clone()} id={ctx.props().id.clone()} rows={ctx.props().size.0.to_string()} columns={ctx.props().size.1.to_string()} onkeypress={text_edit_button_callback} oninput={text_edit_value_callback}></textarea>
        )
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextAreaMessage::KeyPressed(code) => {
                self.last_key_code = code;
            }
            TextAreaMessage::ValueUpdate(current_inner) => {
                _ctx.props().text_buffer.set(current_inner);
            }
        }

        true
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TabState {
    #[default]
    Helyettesitesek,
    Csongetes,
    TanevRendje,
}
