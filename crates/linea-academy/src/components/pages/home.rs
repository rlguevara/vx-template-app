use yew::prelude::*;

use crate::components::{preloader::Preloader, header::Header, banner::MainBanner, categories::Categories,
about::AboutUs, lastcourses::LastCourses, facts::FunFacts, team::Team, contact::ContactUs, footer::Footer};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            // <Preloader />
            <Header />
            <MainBanner />
            <Categories />
            <AboutUs />
            <LastCourses />
            <FunFacts />
            <Team />
            <ContactUs />
            <Footer />
        </div>
    }
}