use yew::prelude::*;
use crate::category::view_show_category::ShowCategoryList;
use crate::category::view_add_category::AddNewCategory;

#[function_component(Categories)]
pub fn categories() -> Html {
    html! {
      <>
      <div class="services section" id="services">
        <div class="container">
          <div class="section-heading text-center">
            <h6>{"Comienza ya"}</h6>
            <h2>{"Nuestras certificaciones"}</h2>
          </div>
          <a href="#" class="btn btn-primary m-1" data-bs-toggle="modal" data-bs-target="#exampleModal1"><i class="fa-solid fa-plus"></i> {" Agregar categoria"}</a>
          <div class="row">
                <ShowCategoryList />
          </div>
        </div>
      </div>
      <div class="modal fade" id="exampleModal1" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
        <div class="modal-dialog">
        <div class="modal-content">
            <div class="modal-header">
            <h5 class="modal-title" id="exampleModalLabel">{"Agregar nueva categoria"}</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <AddNewCategory />
        </div>
        </div>
      </div>
      </>
    }
}