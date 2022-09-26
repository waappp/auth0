use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct Footer {

}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Footer{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container text-center">
            <div class="row row-cols-4"
            style="border-top: .5px solid rgba(0,0,0,.2); font-size: 16px; font-weight:500; color: grey; text-align:left;">
            <div class="col">
             <ul>
              <li style="list-style-type: none; margin-bottom: 10px; margin-top: 10px;">
              {"PLATFORM"}
              </li>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Access Management"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Extensibility"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Login Security"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"User Management"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Authentication"}</a><br/>
             </ul>
            </div>
            <div class="col">
             <ul>
              <li style="list-style-type: none; margin-bottom: 10px; margin-top: 10px;">
              {"USE CASES"}
              </li>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"CIAM"}{" | For your Customers"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"B2B"}{" | For your Bussiness Partners"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"B2E"}{" | For your Employees"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Return on Investment"}</a>
             </ul>
            </div>
            <div class="col">
             <ul>
              <li style="list-style-type: none; margin-bottom: 10px; margin-top: 10px;">
              {"DEVELOPERS"}
              </li>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Documentation"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"APIs"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Tutorials"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Quickstarts"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Comunity"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Support Center"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Auth0 Developer Hub"}</a><br/>
             </ul>
            </div>
            <div class="col">
             <ul>
              <li style="list-style-type: none; margin-bottom: 10px; margin-top: 10px;">
              {"COMPANY"}
              </li>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"About Us"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Our Customers"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Partners"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Careers"} <button class="btn" style="">{"We're hiring!"}</button></a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Press"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Compliance"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Social Impact"}</a><br/>
             </ul>
            </div>
            </div>
            //bottom footer
            <div class="row row-cols-4"
            style="font-size: 16px; font-weight:500; color: grey; text-align:left;">
            <div class="col">
             <ul>
              <li style="list-style-type: none; margin-bottom: 10px; margin-top: 10px;">
              {"FEATURES"}
              </li>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Universal Login"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Single Sign On"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Multifactor Authentication"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Breached Passwords"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Actions"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Machine to Machine"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Passwordless"}</a><br/>
             </ul>
            </div>
            <div class="col">
             <ul>
              <li style="list-style-type: none; margin-bottom: 10px; margin-top: 10px;">
              {"INDUSTRIES"}
              </li>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Financial Services"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Healthcare"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Retail"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"B2B SaaS"}</a>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Public Sector"}</a>
             </ul>
            </div>
            <div class="col">
             <ul>
              <li style="list-style-type: none; margin-bottom: 10px; margin-top: 10px;">
              {"RESOURCES"}
              </li>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Blog"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Reports"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Videos"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Webinars"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Case Studies"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Podcasts"}</a><br/>
             </ul>
            </div>
            <div class="col">
             <ul>
              <li style="list-style-type: none; margin-bottom: 10px; margin-top: 10px;">
              {"GET STARTED"}
              </li>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Pricing"}</a><br/>
              <a href="#" style="text-decoration:none; color: black; font-size:14px; margin-bottom: 5px;">
              {"Contact Sales"}</a><br/>
             </ul>
            </div>
            </div>
            </div>
        }
    }
}
