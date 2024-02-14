use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/portfolio")]
    Portfolio,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(PortfolioItem)]
fn portfolio_item() -> Html {
    html! {
        <div>
            <h1>{"Hi, I'm Arnav"}</h1>
            <h2>{"498 XR P1:"}</h2>
            <div>
            <iframe width="800" height="310"
            src="https://www.youtube.com/embed/XHo4Qe9vRPY">
            </iframe>
            </div>
            <p>{"This project was made with Unreal Engine 5, the industry standard engine for high performance and high visual quality games. We used their Blueprint programming language to add dozens of interactive features"}</p>
            <p>{"This was made with teammate Andrew Hutchinson, using JIRA to ensure we were always up to date on each others' progress"}</p>
            <p>{"This video was recorded on a Meta Quest Pro"}</p>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Redirect<Route> to={Route::Portfolio} />},
        Route::Portfolio => html! { <PortfolioItem/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
