use crate::components::github_radar::activity::ActivityView;
use crate::components::github_radar::repositories::Repositories;
use crate::components::tabs::{Tab, TabContent};
use yew::prelude::*;

mod activity;
mod repositories;

#[derive(Clone, PartialEq, Properties)]
pub struct State {
    pub current_active_tab: usize,
}

pub struct GitHubRadar {
    pub state: State,
    pub link: ComponentLink<Self>,
}

pub enum Msg {
    SetActive(usize),
}

impl Component for GitHubRadar {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: State {
                current_active_tab: 1,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetActive(next_tab) => {
                if next_tab != self.state.current_active_tab {
                    self.state.current_active_tab = next_tab;
                    return true;
                }

                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let current_active_tab = &self.state.current_active_tab;

        #[allow(clippy::redundant_closure)]
        let set_current_tab_callback = self
            .link
            .callback(|next_tab: usize| Msg::SetActive(next_tab));

        html! {
            <section id="github-radar">
                <div class="nav-tabs">
                    <ol class="nav-tabs-list">
                        <Tab
                            tab_key=1_usize
                            title="Actividad".to_string()
                            current_active_tab=current_active_tab
                            onclick=set_current_tab_callback.clone()
                        />
                        <Tab
                            tab_key=2_usize
                            title="Repositorios".to_string()
                            current_active_tab=current_active_tab
                            onclick=set_current_tab_callback.clone()
                        />
                    </ol>
                </div>
                <TabContent id="activity" current_active_tab=current_active_tab tab_key=1_usize>
                    <ActivityView />
                </TabContent>
                <TabContent id="repositories" current_active_tab=current_active_tab tab_key=2_usize>
                    <Repositories />
                </TabContent>
            </section>
        }
    }
}
