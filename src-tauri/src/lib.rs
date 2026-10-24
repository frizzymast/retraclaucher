use tauri::{Emitter, Manager};
use tauri_plugin_deep_link::DeepLinkExt;

pub mod modules;
use modules::{commands, util};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(
  args: Vec<String>,
) {
  let mut builder = tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_updater::Builder::new().build());

  // Handle single-instance forwarding so deep links go to the already-running launcher window
  builder = builder.plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
    println!("[single-instance] Prevented duplicate process. Forwarding args: {:?}", argv);

    // 1. Focus existing main launcher window
    if let Some(window) = app.get_webview_window("main") {
      let _ = window.show();
      let _ = window.set_focus();
    }

    // 2. Search command line arguments for the retrac:// URL
    if let Some(url) = argv.iter().find(|a| a.starts_with("retrac://")) {
      println!("[single-instance] Emitting deep link URL: {}", url);
      let _ = app.emit("deep-link://url", vec![url.clone()]);
    }
  }));

  builder = builder.plugin(tauri_plugin_deep_link::init());
  builder = builder.plugin(tauri_plugin_opener::init());
  builder = builder.plugin(tauri_plugin_dialog::init());

  if let Some(arg) = args.clone().iter().find(|a| a.starts_with("-action_after=")) {
    if let Some(action_value) = arg.splitn(2, '=').nth(1) {
      println!("Found action_after: {}", action_value);
      util::set_app_action(action_value.to_string());
    }
  }

  builder = builder.setup(|app| {
    let handle = app.handle();
    util::set_app_handle(handle.to_owned());

    let main_window = app.get_webview_window("main").expect("no main window");
    if main_window.set_shadow(true).is_err() {
      eprintln!("Failed to set window shadow");
    }

    #[cfg(desktop)]
    app.deep_link().register("retrac")?;

    Ok(())
  });

  builder = builder.invoke_handler(tauri::generate_handler![
    #[cfg(target_os = "windows")]
    commands::get_windows_version,
    commands::get_fortnite_version,
    commands::launch_retrac,
    commands::download_build,
    commands::delete_build,
    commands::is_fortnite_running,
    commands::close_fortnite,
    commands::add_to_defender,
    commands::add_to_defender_multi,
    commands::get_app_action,
    commands::cancel_download,
  ]);

  builder
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}