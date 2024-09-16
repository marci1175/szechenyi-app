use crate::lib::{TabState, TitleButton};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let tab_state = use_state(|| TabState::default());

    html!(
        <div>
            {
                match *tab_state {
                    TabState::Helyettesitesek => {
                        html!(

                        )
                    },
                    TabState::Csongetes => {
                        html!(

                        )
                    },
                    TabState::TanevRendje => {
                        html!(

                        )
                    },
                }
            }

            <div class="menubar">
                <TitleButton label="Helyettesítések" common_state={tab_state.clone()} value={TabState::Helyettesitesek}/>
                <TitleButton label="Csöngetés" common_state={tab_state.clone()} value={TabState::Csongetes}/>
                <TitleButton label="Tanév rendje" common_state={tab_state.clone()} value={TabState::TanevRendje}/>
            </div>
        </div>
    )
}
