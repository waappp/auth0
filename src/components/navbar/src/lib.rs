use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct Navbar {

}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Navbar{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <nav id="navbar" class="navbar bg-white px-3 mb-3 border-bottom">
           <img src="assets/auth0logo.jpeg"/>
           <ul class="nav nav-pills">
            <li class="nav-item">
             <a class="nav-link" href="#">{"Articles"}</a>
            </li>
            <li class="nav-item">
             <a class="nav-link" href="#">{"Quickstarts"}</a>
            </li>
            <li class="nav-item">
             <a class="nav-link" href="#">{"Auth0 APIs"}</a>
            </li>
            <li class="nav-item">
             <a class="nav-link" href="#">{"SDKs"}</a>
            </li>
            <li class="nav-item">
             <a class="nav-link" href="#">{"SDKs"}</a>
            </li>
            <li class="nav-item">
             <a class="nav-link" href="#">{"SDKs"}</a>
            </li>
            <li class="nav-item">
             <a class="nav-link" href="#">{"SDKs"}</a>
            </li>
           </ul>
          </nav>

        }
    }
}
