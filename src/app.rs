use crate::lib::{TabState, TitleButton};
use indexmap::IndexMap;
use log::{error, info};
use scraper::{Html, Selector};
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

pub async fn get_timetable() -> anyhow::Result<String> {
    let raw_html = reqwest::get("https://ujbudaiszechenyi.hu/api/lessontimes")
    .await?
    .text()
    .await?;
    
    Ok(
        raw_html    
    )
}

pub async fn get_calendar() -> anyhow::Result<String> {
    let raw_html = reqwest::get("https://ujbudaiszechenyi.hu/api/calendar")
        .await?
        .text()
        .await?;

    Ok(
        raw_html
    )
}

/// Hiányzó tanárok adattípusa xdd
struct AbsentTeacher {
    /// Név
    name: String,

    /// Elmaradt osztályai  (helyettes, helyettesítés típusa, megjegyzés)
    classes: IndexMap<u8, (String, String, String)>
}

/// P100 webscraping mert valakik lusták voltak megcsinalni az apit
pub async fn get_absent() -> anyhow::Result<Vec<AbsentTeacher>> {
    let page_string = reqwest::get("https://ujbudaiszechenyi.hu/helyettesites/kijelzo")
        .await?
        .text()
        .await?;
    
    let parsed_html = Html::parse_document(&page_string);

    let selector = Selector::parse("h3").unwrap();

    for item in parsed_html.select(&selector) {
        dbg!(item.value());
    }

    Ok(
        vec![]
    )
}