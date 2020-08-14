use crate::components::title::Title;

use yew::prelude::*;

pub struct About;

impl Component for About {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    html! {
      <section id="about">
        <div id="content">
          <article id="about-us">
            <Title href="about" text="Acerca de Nosotros" />
            <p>
              {
                r#"Somos un grupo de programadores con base en Venezuela e interesados en desarrollar
                con el lenguaje Rust."#
              }
            </p>
          </article>
          <article id="code">
            <Title href="code" text="Código de Conducta" />
            <p>
              {
                r#"Esperamos que cualquier participante de esta organización y los medios acompañados
                esten enterados del siguiente código de conducta y ayuden a cumplir el mismo.
                Llámese participante de esta organización a cualquier colaborador de la organización en GitHub,
                miembro del chat en Telegram, contribuidor que no sea miembro de la organización, asi como cualquier
                medio de comunicación o participación que es representado por esta comunidad."#
              }
              <hr />
              { "Se requiere que todo participante de la organización " }
              <b>{ "rust-lang-ve" }</b>
              { " cumpla con el código de conducta que se redacta a continuación." }
              <br />
              <br />
              {
                r#"Todos los participantes de la organización tanto contribuidores como no contribuidores deben ser
                tratados con respeto y ser libres de cualquier discriminación, así como intimidación, acoso o victimización.
                No será tolerado ningún tipo de contenido o comportamiento obseno e inadecuado de cualquier naturaleza,
                sean imágenes inadecuadas, o lenguaje inapropiado."#
              }
              {
                r#"Si algún participante se ve envuelto en alguno de los comportamientos antes mencionados, los administradores
                de la comunidad en cuestión tendrán derecho a tomar acciones que crean pertinentes hasta la expulsión de todo
                medio relacionado a la misma."#
              }
            </p>
          </article>
        </div>
      </section>
    }
  }
}
