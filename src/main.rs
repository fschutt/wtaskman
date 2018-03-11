// Linux: libwebkit2gtk-4.0-dev

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate urlencoding;
extern crate web_view;
extern crate procinfo;
extern crate itertools;

use urlencoding::encode;
use web_view::*;

const APP_TITLE: &str = "Task Manager";

/// The data
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RpcData {
    pub currently_selected_processes: Vec<usize>,
    pub running_processes: Vec<ProcessInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// The name of the process in the UI
    name: String,
    /// The image of the executable, or null for a default icon
    image_path: Option<String>,
    /// Type of process
    process_type: ProcessType,
    /// Publisher, or null if no publisher is active
    publisher: Option<String>,
    /// Process name
    process_name: String,
    /// Command line invokation
    command_line: String,
    /// CPU percentage
    cpu_percentage: f32,
    /// Memory usage in MB (not MiB)!
    memory: f32,
    /// Disk usage of the process
    disk: f32,
    /// Network usage of the process
    network: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProcessType {
    BackgroundProcess,
    App,
    SystemProcess,
}

#[derive(Deserialize)]
#[serde(tag = "cmd")]
pub enum Cmd {
    #[serde(rename = "init")]
    Init,
    #[serde(rename = "log")]
    Log { text: String },
    /// Kill the currently selected process
    #[serde(rename = "kill_selected_process")]
    KillSelectedProcess,
    /// Selects a process to be killed
    #[serde(rename = "select_process")]
    SelectProcess { id: usize },
}

fn webview_cb<'a>(webview: &mut WebView<'a, RpcData>, arg: &str, data: &mut RpcData) {
    
    match serde_json::from_str::<Cmd>(arg) {
        Ok(arg) => {
            match arg {
                Cmd::Init => (),
                Cmd::Log { text } => println!("{}", text),
                Cmd::KillSelectedProcess => { 
                    println!("Killing processes ... {:?}", data.currently_selected_processes); 
                }
                Cmd::SelectProcess { id } => {
                    println!("selecting process for killing: {:?}", id);
                    data.currently_selected_processes.push(id);
                }
            }
        },
        Err(e) => {
            println!("[Rust] Got garbage RPC from JS: {:?}\r\n\r\n{}", e, arg);
        }
    }

    webview.eval(&format!("rpc.update_process_table_view({})", serde_json::to_string(&data.running_processes).unwrap()));
    // webview.eval(&format!("rpc.render({})", serde_json::to_string(tasks).unwrap()));
}

fn main() {
    let app_html = include_str!("dist/app.html");
    let url = "data:text/html,".to_string() + &encode(&app_html);
    let size = (840, 630);
    let resizable = true;
    let debug = true;

    let init_cb = |_webview| {};

    let userdata = RpcData {
        running_processes: get_currently_running_processes().unwrap_or_else(|_e| Vec::new()),
        .. Default::default()
    };

    let (_, launched_successful) = run(APP_TITLE, &url, Some(size), resizable, debug, init_cb, |webview, arg, data: &mut RpcData| {
        webview_cb(webview, arg, data);
    }, userdata);

    if !launched_successful {
        println!("failed to launch {}", env!("CARGO_PKG_NAME"));
    }
}

use std::io;

#[derive(Debug)]
pub enum GetProcessError {
    Io(io::Error),
    ProcDoesNotExist
}

impl From<io::Error> for GetProcessError {
    fn from(e: io::Error) -> Self {
        GetProcessError::Io(e)
    }
}
// i32;

#[cfg(target_os="linux")]
pub fn get_currently_running_processes() -> Result<Vec<ProcessInfo>, GetProcessError>  {

    use std::path::Path;
    use std::fs;
    use itertools::Itertools;

    let proc_dir = Path::new("/proc");
    
    if !proc_dir.exists() || !proc_dir.is_dir() {
        return Err(GetProcessError::ProcDoesNotExist);
    }

    let proc_iter = fs::read_dir(proc_dir)?;

    use procinfo::pid::Status;

    let running_processes: Vec<(String, Vec<Status>)> = proc_iter
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter_map(|e| e.parse::<i32>().ok())
        .filter_map(|pid| procinfo::pid::status(pid).ok())
        .group_by(|status| status.command.clone())
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();

    for (k, _) in running_processes {
        println!("{:?}", k);
    }

    Ok(Vec::new()) // TODO

}