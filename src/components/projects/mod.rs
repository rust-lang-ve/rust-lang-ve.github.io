mod repositories;

use crate::components::title::Title;
use crate::components::projects::repositories::Repositories;

use yew::prelude::*;

pub struct Projects {}

pub enum Msg {}

impl Component for Projects {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    html! {
      <section id="projects">
        <div id="content">
          <Title href="projects" text="Proyectos" />
          <p class="contents">
            {
              r#"Sientete libre de contribuir a cualquiera de nuestros proyectos en cualquier
              nivel, sea abriendo un"#
            }
            <i>{" issue "}</i>
            {
              r#", discutiendo en nuestro grupo de Telegram
              o bien sea abriendo un "#
            }
            <i>{ "pull request" }</i>
            { "!" }
          </p>
          <p class="content">
            {
              r#"Estamos aca para crecer juntos and ayudar a otros futuros Rustaceans a escribir
              sus aplicaciones y conocer lo increible que es Rust!"#
            }
          </p>
          <Title href="github-projects" text="Repositorios en GitHub" />
          <Repositories />
        </div>
      </section>
    }
  }
}
