use actix_web::client::Client;
use std::process::Command;

#[cfg(feature = "embed_ui")]
use std::thread;

async fn check_server_up() -> bool {
    let client = Client::default()
        .get("http://localhost:8000/check") // <- Create request builder
        .header("User-Agent", "Actix-web")
        .send()
        .await;
    match client {
        Ok(r) => r.status() == actix_web::http::StatusCode::OK,
        Err(_) => false,
    }
}

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

#[cfg(target_os = "windows")]
extern crate winapi;

pub async fn main_ui() -> Result<(), ()> {
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
    let _ = thread::spawn(|| crate::start_rocket());
    let mut ok = false;
    for _ in 0..10 {
        ok = check_server_up().await;
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
