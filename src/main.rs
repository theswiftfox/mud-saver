#![cfg_attr(
    all(feature = "embed_ui", not(debug_assertions)),
    windows_subsystem = "windows"
)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate dirs;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate uuid;
extern crate zip;

use chrono::{
    offset::{Local, Utc},
    DateTime,
};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{
    handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError},
    Template,
};

#[cfg(feature = "embed_ui")]
use std::thread;

use std::process::Command;
use std::sync::Mutex;

mod appconfig;
mod error;
mod pages;
mod snowrunner;

lazy_static! {
    static ref SETTINGS: Mutex<appconfig::Settings> = Mutex::new(appconfig::try_load());
}

const APP_DATA_NAME: &'static str = "MudSaver";

pub fn get_app_data_dir() -> std::io::Result<std::path::PathBuf> {
    let path = dirs::data_dir();
    if path.is_some() {
        let mut p = path.unwrap();
        p.push(APP_DATA_NAME);
        Ok(p)
    } else {
        std::env::current_dir()
    }
}

fn start_rocket() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                pages::exit,
                pages::index,
                pages::check,
                pages::overview,
                pages::mud_runner,
                pages::snow_runner,
                pages::settings,
                pages::save_settings,
                pages::store_snow_runner_profile,
                pages::get_snowrunner_profile,
                pages::delete_snow_runner_save,
                pages::restore_snow_runner_save,
            ],
        )
        .mount("/images", StaticFiles::from("./images"))
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper(
                "date-time",
                Box::new(
                    |h: &Helper,
                     _: &Handlebars,
                     _: &Context,
                     _: &mut RenderContext,
                     out: &mut dyn Output|
                     -> HelperResult {
                        if let Some(date_val) = h.hash().get("date") {
                            let date_js = date_val.value();
                            if let Ok(date) =
                                serde_json::from_value::<DateTime<Utc>>(date_js.clone())
                            {
                                let local = DateTime::<Local>::from(date);
                                let local_date = local.naive_local();
                                let date_str = format!(
                                    "{} - {}",
                                    local_date.date(),
                                    local_date.time().format("%H:%M:%S")
                                );
                                out.write(&date_str)?;
                                return Ok(());
                            }
                        }
                        Err(RenderError::new("Error parsing date"))
                    },
                ),
            );
        }))
        .launch();
}

#[cfg(feature = "embed_ui")]
extern crate reqwest;

#[cfg(feature = "embed_ui")]
fn check_server_up() -> bool {
    let request = match reqwest::get("http://localhost:8000/check") {
        Ok(r) => r,
        Err(_) => return false,
    };
    request.status() == reqwest::StatusCode::OK
}

#[cfg(feature = "embed_ui")]
fn start_ui() {
    let res = (1000, 600);
    web_view::builder()
        .title("MudSaver")
        .content(web_view::Content::Url("http://localhost:8000"))
        .size(res.0 as i32, res.1 as i32)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}

#[cfg(all(feature = "embed_ui", target_os = "windows"))]
extern crate winapi;

#[cfg(feature = "embed_ui")]
fn main_ui() -> Result<(), ()> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let edge_check = || -> Result<(), Box<dyn std::error::Error>> {
            // check if local permission is set for edge
            let mut cmd = Command::new("cmd");
            cmd.args(&["/C", "CheckNetIsolation.exe", "LoopbackExempt", "-s"]);
            cmd.creation_flags(CREATE_NO_WINDOW);
            let output = cmd.output()?;

            let output_str = std::str::from_utf8(&output.stdout)?;

            #[cfg(debug_assertions)]
            println!("{}", output_str);

            if !(output_str.contains("microsoft.Win32WebViewHost_cw5n1h2txyewy")
                || (output_str.contains("microsoft.win32webviewhost_cw5n1h2txyewy")))
            {
                use winapi::ctypes::c_int;
                use winapi::shared::minwindef::HINSTANCE;
                use winapi::shared::ntdef::LPCWSTR;
                use winapi::shared::windef::HWND;
                extern "system" {
                    pub fn ShellExecuteW(
                        hwnd: HWND,
                        lpOperation: LPCWSTR,
                        lpFile: LPCWSTR,
                        lpParameters: LPCWSTR,
                        lpDirectory: LPCWSTR,
                        nShowCmd: c_int,
                    ) -> HINSTANCE;
                }
                const SW_SHOW: c_int = 1;

                use std::os::windows::prelude::*;
                let path: Vec<u16> = std::ffi::OsStr::new("CheckNetIsolation.exe")
                    .encode_wide()
                    .chain(Some(0).into_iter())
                    .collect();
                let operation: Vec<u16> = std::ffi::OsStr::new("runas")
                    .encode_wide()
                    .chain(Some(0).into_iter())
                    .collect();
                let parameters: Vec<u16> = std::ffi::OsStr::new(
                    "LoopbackExempt -a -n=Microsoft.Win32WebViewHost_cw5n1h2txyewy",
                )
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect();

                let result = unsafe {
                    ShellExecuteW(
                        std::ptr::null_mut(),
                        operation.as_ptr(),
                        path.as_ptr(),
                        parameters.as_ptr(),
                        std::ptr::null(),
                        SW_SHOW,
                    )
                };
                if result != std::ptr::null_mut() {
                    unsafe {
                        winapi::um::synchapi::WaitForSingleObject(
                            result as *mut _,
                            winapi::um::winbase::INFINITE,
                        );
                    }
                } else {
                    panic!("Error on init!");
                }
            }

            Ok(())
        };

        edge_check().expect("Error starting application..");
    }
    let _ = thread::spawn(|| start_rocket());
    let mut ok = false;
    for _ in 0..10 {
        ok = check_server_up();
        if ok {
            break;
        }
        thread::sleep(std::time::Duration::from_secs(1));
    }
    if !ok {
        return Err(());
    }
    start_ui();

    Ok(())
}

#[cfg(feature = "embed_ui")]
fn main() {
    std::process::exit(match main_ui() {
        Ok(_) => 0,
        Err(_) => {
            eprintln!("Error while running application.");
            1
        }
    });
}

#[cfg(not(feature = "embed_ui"))]
fn main() {
    start_rocket();
}
