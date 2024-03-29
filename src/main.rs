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

struct PortfolioDescriptor {
    project_name: String,
    video_height: u32,
    video_width: u32,
    video_src: String,
    description: String,
}

#[function_component(Portfolio)]
fn portfolio() -> Html {
    let portfolio_items = [
        PortfolioDescriptor {
            project_name: "498 XR P1".to_string(),
            video_height: 310,
            video_width: 800,
            video_src: "https://www.youtube.com/embed/XHo4Qe9vRPY".to_string(),
            description: r#"This project was made with Unreal Engine 5, the industry standard engine for high performance and high visual quality games. We used their Blueprint programming language to add dozens of interactive features.
This was made with teammate Andrew Hutchinson, using JIRA to ensure we were always up to date on each others' progress.
This video was recorded on a Meta Quest Pro."#.to_string(),
        },
        PortfolioDescriptor {
            project_name: "498 XR P2".to_string(),
            video_height: 800,
            video_width: 310,
            video_src: "https://www.youtube.com/embed/5DlQuCnRHuE".to_string(),
            description: r#"This project was made with Unity in C#, the most popular engine for mobile applications.
This was made with teammate Andrew Hutchinson, using JIRA to ensure we were always up to date on each others' progress."#.to_string(),
        },
    ];
    html! {
        <>
            <h1>{"Hi, I'm Arnav"}</h1>
            {portfolio_items.iter().map(portfolio_item).collect::<Html>()}
        </>
    }
}

fn portfolio_item(descriptor: &PortfolioDescriptor) -> Html {
    html! {
        <div>
            <h2>{&descriptor.project_name}</h2>
            <div>
                <iframe width={descriptor.video_width.to_string()} height={descriptor.video_height.to_string()}
                    src={descriptor.video_src.clone()}>
                </iframe>
            </div>
            {descriptor.description.split('\n').map(|s| html! {<p>{s}</p>}).collect::<Html>()}
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Redirect<Route> to={Route::Portfolio} />},
        Route::Portfolio => html! { <Portfolio/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
