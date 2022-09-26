use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct Article {

}

impl Component for Article {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Article{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="col-6">
            <article class="article">
             <h1 style="font-weight">{"Application in Auth0"}</h1>
             <h2>{"The term application or app in Auth0 does not imply any particular implementation characteristics. For example, it could be a native app that executes on a mobile device, a single-page application that executes on a browser, or a regular web application that executes on a server."}</h2>
             <h2>{"Auth0 categorizes apps based on these characteristics:"}</h2>
             <li>{"Application type: To add authentication to your application, you must register it in the Auth0 Dashboard and select from one of the following application types: "}</li>
             <ul>
              <li>{"Regular web application: Traditional web apps that perform most of their application logic on the server (such as Express.js or ASP.NET). To learn how to set up a regular web application, read Register Regular Web Applications."}</li>
              <li>{"Single page web application (SPA): JavaScript apps that perform most of their user interface logic in a web browser, communicating with a web server primarily using APIs (such as AngularJS + Node.js or React). To learn how to set up a Single-page web application, read Register Single-Page Web Applications."}</li>
              <li>{"Native application: Mobile or Desktop applications that run natively on a device (such as iOS or Android). To learn how to set up a native application, read Register Native Applications."}</li>
              <li>{"Machine to machine (M2M) application: Non-interactive applications, such as command-line tools, daemons, IoT devices, or services running on your backend. Typically, you use this option if you have a service that requires access to an API. To learn how to set up a native application, read Register Machine-to-Machine Applications."}</li>
             </ul>
             <li>{"Credential security: According to the OAuth 2.0 spec, apps can be classified as either public or confidential; confidential apps can hold credentials securely, while public apps cannot. To learn more, read Confidential and Public Applications."}</li>
             <li>{"Ownership: Whether an app is classified as first- or third-party depends on app ownership and control. First-party apps are controlled by the same organization or person that owns the Auth0 domain. Third-party apps enable external parties or partners to securely access protected resources behind your API. To learn more, read First-Party and Third-Party Applications."}</li>
             <h1>{"Manage Applications Settings "}</h1>
             <h2>{"You register applications in Dashboard > Applications > Applications. In addition to setting up applications in the Dashboard, you can also set up applications programmatically as described in the OpenID Connect (OIDC) Dynamic Client Registration 1.0 specification. "}</h2>
             <h2>{"You can set up a more complex configuration that allows users to log in differently for different apps. To learn more, read Multi-Tenant Application Best Practices and Create Multiple Tenants."}</h2>
             <h2>{"By default, Auth0 enables all connections associated with your tenant when you create a new application. To change this, update application connections in the Application Settings in the Dashboard."}</h2>
            </article>
           </div>
        }
    }
}