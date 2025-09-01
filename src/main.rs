#[macro_use] extern crate rocket;

mod versions;

use crate::versions::v1::{extdata::{v1_delete_extdata, v1_get_extdata, v1_get_extdata_file, v1_put_extdata}, save::{v1_delete_save, v1_get_save_file, v1_get_saves, v1_put_save}, title::{v1_get_title, v1_get_titles}, v1_status};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            v1_status,

            v1_get_saves,
            v1_get_save_file,
            v1_delete_save,
            v1_put_save,

            v1_get_extdata,
            v1_get_extdata_file,
            v1_delete_extdata,
            v1_put_extdata,

            v1_get_title,
            v1_get_titles
        ])
}
