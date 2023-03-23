use yew::prelude::*;
use yew_router::prelude::*;
// use crate::components::router::{switch, Route};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header-area header-sticky">
            <div class="container">
                <div class="row">
                    <div class="col-12">
                        <nav class="main-nav">
                            //<!-- ***** Logo Start ***** -->
                            <a href="index.html" class="logo">
                                <img src="../../assets/images/logo-academy.png" alt="Linea rosa" width="224px" height="40px"/>
                            </a>
                            //<!-- ***** Logo End ***** -->
                            //<!-- ***** Menu Start ***** -->
                            <ul class="nav">
                                <li class="scroll-to-section"><a href="#top" class="active">{"Home"}</a></li>
                                <li class="scroll-to-section"><a href="#services">{"Services"}</a></li>
                                <li class="scroll-to-section"><a href="#courses">{"Courses"}</a></li>
                                <li class="scroll-to-section"><a href="#team">{"Team"}</a></li>
                                // <li class="scroll-to-section"><a href="#events">{"Events"}</a></li>
                                <li class="scroll-to-section"><a href="#contact">{"Contact us"}</a></li>
                                // <BrowserRouter>
                                //     <Switch<Route> render={switch}/>
                                // </BrowserRouter>
                            </ul>   
                            <a class="menu-trigger">
                                <span>{"Menu"}</span>
                            </a>
                            //<!-- ***** Menu End ***** -->
                        </nav>
                    </div>
                </div>
            </div>
        </header>
    }
}