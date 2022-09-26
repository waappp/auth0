use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct Sidebar {

}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Sidebar{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="row">
            <div class="col">
               <aside>
                <nav class="wrapper">
                 <div class="sidebarcontainer">
                 <ul>
                  <li><h1><i class="fa fa-arrow-left" aria-hidden="true"></i>{"Main Menu"}</h1></li>
                  <li><h2><i class="fa fa-bolt" aria-hidden="true"></i>{"Get Started"}</h2></li>
                  <li><h3>{"START BUILDING"}</h3></li>
                  <li>
                   <span class="side-dropdown1">{"QuickStarts"}</span>
                  </li>
                  <li><h3>{"LEARN THE BASICS"}</h3></li>
                  <li>
                   <span class="side-dropdown1">{"Identity Fundamentals"}</span>
                   <span class="side-dropdown1">{"Auth0 Overview"}</span>
                  </li>
                  <li><h3>{"CONFIGURE AUTH0"}</h3></li>
                  <li>
                   <span class="side-dropdown1">{"Tenant Settings"}</span>
                   <span class="side-dropdown1">{"Application in Auth0"}</span>
                    <div class="dropdown-side">
                     <a href="#">{"Aplication Settings"}</a>
                     <a href="#">{"Subdomain URL Placeholders"}</a>
                     <a href="#">{"Confidential and Public Applications"}</a>
                     <a href="#">{"Dynamic Application Registration"}</a>
                     <a href="#">{"Set Up Database Connections"}</a>
                     <a href="#">{"Test Database Connections"}</a>
                     <a href="#">{"Application Grant Types"}</a>
                     <a href="#">{"Revoke Access to APIs Using Application Grants"}</a>
                     <a href="#">{"Signing Algorithms"}</a>
                     <a href="#">{"Change Application Signing Algorithms"}</a>
                     <a href="#">{"Configure Application Metadata"}</a>
                     <a href="#">{"Update Application Connections"}</a>
                     <a href="#">{"Rotate Client Secrets"}</a>
                     <a href="#">{"Enable Android App Links Support"}</a>
                    </div>
                    <span class="side-dropdown1">{"APIs"}</span>
                    <span class="side-dropdown1">{"Manage Dashboard Access"}</span>
                  </li>
                  <li><h3>{"PLAN AND DESIGN"}</h3></li>
                  <li>
                    <span class="side-dropdown1">{"Authentication and Autorization Flows"}</span>
                    <span class="side-dropdown1">{"Architecture Scenarios"}</span>
                    <span class="side-dropdown1">{"Professional Services"}</span>
                  </li>   
                 </ul>
                 </div>
                </nav>  
               </aside>
               </div>
            </div>
        }
    }
}
