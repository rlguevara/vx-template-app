use yew::prelude::*;

#[function_component(MainBanner)]
pub fn main_banner() -> Html {
    html! {
        <div class="main-banner" id="top">
            <div class="container">
                <div class="row">
                    <div class="col-lg-12">
                    <div class="owl-carousel owl-banner">
                        <div class="item item-1">
                        <div class="header-text">
                            <span class="category">{"Nuestras certificaciones"}</span>
                            <h2>{"Con la Academia Linea Rosa, todo es más fácil"}</h2>
                            <p>{"En Línea Rosa, todas las conductoras son mujeres certificadas con el objetivo de garantizar la comodidad de sus pasajeras, manteniendo altos estándares de seguridad y atención al cliente."}</p>
                            <div class="buttons">
                            <div class="main-button">
                                <a href="#">{"¡Regístrate!"}</a>
                            </div>
                            <div class="icon-button">
                                <a href="https://linearosa.app/quienes-somos"><i class="fa fa-play"></i> {"¿Qué es Linea Rosa?"}</a>
                            </div>
                            </div>
                        </div>
                        </div>
                    </div>
                    </div>
                </div>
            </div>
        </div>
    }
}