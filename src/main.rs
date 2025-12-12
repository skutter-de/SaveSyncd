use std::{collections::HashMap, sync::{Arc, Mutex}, time::Duration};

use rocket::{data::{Limits, ToByteUnit}, tokio};
use tray_icon::{TrayIconEvent, menu::MenuEvent};
use winit::event_loop::EventLoop;
use crate::{config::Config, tray_app::{Application, QuitEvent, UserEvent}, versions::v1};

#[macro_use] extern crate rocket;

pub mod config;
pub mod versions;
pub mod tray_app;


fn main() {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    let tray_icon_proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| { let _ = tray_icon_proxy.send_event(UserEvent::TrayIconEvent(event)); }));
    
    let menu_proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| { let _ = menu_proxy.send_event(UserEvent::MenuEvent(event)); }));

    let quit_proxy = event_loop.create_proxy();
    QuitEvent::set_event_handler(Some(move |event| { let _ = quit_proxy.send_event(UserEvent::QuitEvent(event)); }));

    let mut app = Application::new();
    
    let _menu_channel = MenuEvent::receiver();
    let _tray_channel = TrayIconEvent::receiver();
    
    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        gtk::init().unwrap();
        let _tray_icon = Application::new_tray_icon();
        gtk::main();
    });

    // cleanup previous if exists
    v1::ticket::clear_tickets_path().expect("Failed to clear old tickets path");

    let tickets: v1::ticket::Tickets = Arc::new(Mutex::new(HashMap::new()));
    let config = Config::load();
        
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", config.port()))
        .merge(("limits", Limits::new()
            .limit("bytes", 100.mebibytes())
            .limit("json", 8.mebibytes())
        ));

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to create Tokio runtime");

    rt.spawn(async move {
        rocket::custom(figment)
            .manage(tickets)
            .manage(config)
            .mount("/", routes![
                v1::status_get,
                v1::status_head,
                v1::titles::titles,

                v1::upload::begin::upload_begin,
                v1::upload::file::upload_file,
                v1::upload::end::upload_end,
                v1::upload::cancel::upload_cancel,

                v1::download::begin::download_begin,
                v1::download::file::download_file,
                v1::download::end::download_end
            ])
            .launch()
            .await
    });

    rt.spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        QuitEvent::send(QuitEvent{});
    });
    
    if let Err(err) = event_loop.run_app(&mut app) {
        println!("Error: {:?}", err);
    }

    rt.shutdown_timeout(Duration::from_millis(1000));
    v1::ticket::clear_tickets_path().expect("Failed to cleanup tickets path");
}