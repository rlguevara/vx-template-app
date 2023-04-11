use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::router::Route;
use crate::course::view_add_course::AddNewCourse;
use crate::course::view_delete_course::DelCourse;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub course_id: i64,
    pub course_name: String,
    pub category_id: i64,
}

#[function_component(CourseItem)]
pub fn course_item(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let vcourse_id = props.course_id;
    let onclick = Callback::from(move |_| navigator.push(&Route::Content { id: vcourse_id}));
    html! {
        <>
        <div class="col-lg-4 col-md-6 align-self-center mb-30 col-md-6">
            <div class="events_item">
                <div class="thumb">
                    <a onclick={onclick}><img src="../assets/images/course-01.jpg" alt=""/></a>
                    <span class="price"><h6><em>{"$"}</em>{"160"}</h6></span>
                </div>
                <div class="down-content">
                    <span class="author">{"Stella Blair"}</span>
                    <h4>{&props.course_name}</h4>
                    <p>{format!("{:?}", vcourse_id)}</p>
                </div>
                <a href="#" class="btn btn-primary m-1" data-bs-toggle="modal" data-bs-target="#exampleModal1"><i class="fa-solid fa-plus"></i></a>
                <a href="#" class="btn btn-primary m-1" data-bs-toggle="modal" data-bs-target="#exampleModal2"><i class="fa-regular fa-pen-to-square"></i></a>
                <DelCourse course_id={vcourse_id}/>
            </div>
        </div>
        <div class="modal fade" id="exampleModal1" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
            <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header">
                <h5 class="modal-title" id="exampleModalLabel">{"Agregar nuevo curso"}</h5>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <AddNewCourse c_id={&props.category_id}/>
                // <div class="modal-body">
                //     <div class="input-group mb-3">
                //         <span class="input-group-text" id="basic-addon1">{"Nombre"}</span>
                //         <input type="text" class="form-control" placeholder="Username" aria-label="Username" aria-describedby="basic-addon1"/>
                //     </div>
                //     <div class="input-group mb-3">
                //         <span class="input-group-text" id="basic-addon1">{"Descripción"}</span>
                //         <input type="text" class="form-control" placeholder="Username" aria-label="Username" aria-describedby="basic-addon1"/>
                //     </div>
                // </div>
                // <div class="modal-footer">
                //     <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancelar"}</button>
                //     <button type="button" class="btn btn-primary">{"Crear curso"}</button>
                // </div>
            </div>
            </div>
        </div>
        <div class="modal fade" id="exampleModal2" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
            <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header">
                <h5 class="modal-title" id="exampleModalLabel">{"Editar curso"}</h5>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <div class="modal-body">
                    <div class="input-group mb-3">
                        <span class="input-group-text" id="basic-addon1">{"Nombre"}</span>
                        <input type="text" class="form-control" placeholder="Username" aria-label="Username" aria-describedby="basic-addon1"/>
                    </div>
                    <div class="input-group mb-3">
                        <span class="input-group-text" id="basic-addon1">{"Descripción"}</span>
                        <input type="text" class="form-control" placeholder="Username" aria-label="Username" aria-describedby="basic-addon1"/>
                    </div>
                </div>
                <div class="modal-footer">
                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancelar"}</button>
                <button type="button" class="btn btn-primary">{"Guardar cambios"}</button>
                </div>
            </div>
            </div>
        </div>  
        // <div class="modal fade" id="exampleModal3" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
        //     <div class="modal-dialog">
        //     <div class="modal-content">
        //         <div class="modal-header">
        //         <h5 class="modal-title" id="exampleModalLabel">{"Eliminar curso"}</h5>
        //         <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        //         </div>
        //         <div class="modal-body">
        //             <p>{"¿Estás seguro que quieres eliminar este curso?"}</p>
        //         </div>
        //         // <div class="modal-footer">
        //         // <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancelar"}</button>
        //         // <button type="button" class="btn btn-danger">{"Eliminar"}</button>
        //         // </div>
        //         <DelCourse course_id={vcourse_id}/>
        //     </div>
        //     </div>
        // </div>    
        </>
    }
}