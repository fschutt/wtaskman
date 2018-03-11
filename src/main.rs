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
use std::io;

const APP_TITLE: &str = "Task Manager";
const GTK_OVERLAY_SCROLLING: &str = "GTK_OVERLAY_SCROLLING";

/// The data
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RpcData {
    pub currently_selected_processes: Vec<usize>,
    pub running_processes: Vec<ProcessInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// The name of the process in the UI
    pub name: String,
    /// The image of the executable, or null for a default icon
    pub image_path: Option<String>,
    /// Type of process
    pub process_type: ProcessType,
    /// Publisher, or null if no publisher is active
    pub publisher: Option<String>,
    /// Process name
    pub process_name: String,
    /// Command line invokation
    pub command_line: String,
    /// CPU percentage
    pub cpu_percentage: f32,
    /// Memory usage in MB (not MiB)!
    pub memory: f32,
    /// Disk usage of the process
    pub disk: f32,
    /// Network usage of the process
    pub network: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProcessType {
    BackgroundProcess,
    App,
    SystemProcess,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Html(pub String);

impl ProcessType {
    pub fn to_str(&self) -> &'static str {
        use ProcessType::*;
        match *self {
            BackgroundProcess => "Background process",
            App => "App",
            SystemProcess => "System process",
        }
    }
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
    #[serde(rename = "update_process_table")]
    Update,
}

fn webview_cb<'a>(webview: &mut WebView<'a, RpcData>, arg: &str, data: &mut RpcData) {
    
    match serde_json::from_str::<Cmd>(arg) {
        Ok(arg) => {
            match arg {
                Cmd::Init => (),
                Cmd::Log { text } => println!("{}", text),
                Cmd::KillSelectedProcess => { 
                    println!("Killing processes ... {:?}", data.currently_selected_processes); 
                },
                Cmd::SelectProcess { id } => {
                    println!("selecting process for killing: {:?}", id);
                    data.currently_selected_processes.push(id);
                },
                Cmd::Update => {
                    if let Ok(processes) = get_currently_running_processes() {
                        data.running_processes = processes;
                        let html = Html(build_process_info_string(&data.running_processes));
                        webview.eval(&format!("rpc.update_process_table_view({})", serde_json::to_string(&html).unwrap()));
                    } else {
                        eprintln!("Update command failed");
                    }
                }
            }
        },
        Err(e) => {
            println!("[Rust] Got garbage RPC from JS: {:?}\r\n\r\n{}", e, arg);
        }
    }

    // webview.eval(&format!("rpc.render({})", serde_json::to_string(tasks).unwrap()));
}

fn main() {

    use std::env;
    let original_value = env::var(GTK_OVERLAY_SCROLLING);
    env::set_var(GTK_OVERLAY_SCROLLING, "0"); // disable overlaid scrollbars
    
    let app_html = include_str!("dist/app.html");
    let url = "data:text/html,".to_string() + &encode(&app_html);
    let size = (840, 630);
    let resizable = true;
    let debug = true;

    let init_cb = |_webview| { };

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

    if let Ok(original_value) = original_value {
        env::set_var(GTK_OVERLAY_SCROLLING, original_value);
    }
}

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

#[cfg(target_os="linux")]
pub fn get_currently_running_processes() -> Result<Vec<ProcessInfo>, GetProcessError>  {

    use std::path::{Path, PathBuf};
    use std::fs;
    use itertools::Itertools;
    use procinfo::pid::{status, cwd, Status};

    let proc_dir = Path::new("/proc");

    if !proc_dir.exists() || !proc_dir.is_dir() {
        return Err(GetProcessError::ProcDoesNotExist);
    }

    let proc_iter = fs::read_dir(proc_dir)?;

    let mut running_processes = proc_iter
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter_map(|e| e.parse::<i32>().ok())
        .filter_map(|pid| {
            if let (Ok(status), Ok(cwd)) = (status(pid), cwd(pid)) {
                Some((status, cwd))
            } else {
               None 
            }
        })
        .collect::<Vec<(Status, PathBuf)>>();

    running_processes.sort_unstable_by_key(|&(ref a, _)| a.command.clone());

    let grouped_processes: Vec<(String, Vec<(Status, PathBuf)>)> = running_processes
        .into_iter()
        .group_by(|&(ref status, _)| status.command.clone())
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();

    Ok(grouped_processes.into_iter().map(|(process_name, v)| {
        // TODO: We can't just use status.vm_size because that includes memory that is 
        // shared with other processes
        let total_mem_kilobyte = v.iter().map(|&(ref status, _)| status.vm_size).sum::<usize>() as f32 / 8.0;
        let total_mem_megabyte = total_mem_kilobyte / 1000.0;

        ProcessInfo {
            name: process_name.clone(),
            image_path: None,
            process_type: ProcessType::App,
            publisher: None,
            process_name: process_name,
            command_line: (v[0].1).to_str().unwrap().to_string(),
            cpu_percentage: 0.0,
            memory: total_mem_megabyte,
            disk: 0.0,
            network: 0.0,
        }
    }).collect())
}

const PROCESS_TABLE_HEADER: &str = "                                \
<tr>                                                                \
  <th><p>Name</p></th>                                              \
  <th class='seperator_v movable'></th>                             \
  <th><p>Type</p></th>                                              \
  <th class='seperator_v movable'></th>                             \
  <th><p>Process name</p></th>                                      \
  <th class='seperator_v movable'></th>                             \
  <th><p>Command line</p></th>                                      \
  <th class='seperator_v movable'></th>                             \
  <th class='align_right width_fixed_100 selected_column_blue'>     \
    <div>27%</div>                                                  \
    <p>CPU</p>                                                      \
  </th>                                                             \
  <th class='seperator_v'></th>                                     \
  <th class='align_right width_fixed_100'>                          \
    <div>27%</div>                                                  \
    <p>Memory</p>                                                   \
  </th>                                                             \
  <th class='seperator_v'></th>                                     \
  <th class='align_right width_fixed_100'>                          \
    <div>2%</div>                                                   \
    <p>Disk</p>                                                     \
  </th>                                                             \
  <th class='seperator_v'></th>                                     \
  <th class='align_right width_fixed_100'>                          \
    <div>0%</div>                                                   \
    <p>Network</p>                                                  \
  </th>                                                             \
</tr>                                                               \
";

pub fn build_process_info_string(process_infos: &[ProcessInfo]) -> String {
    let rows = process_infos.iter().map(|info| info.into_html_row()).collect::<Vec<_>>().join("");
    format!("{}{}", PROCESS_TABLE_HEADER, rows)
}

impl ProcessInfo {
    /// Returns the HTML String for one row
    pub fn into_html_row(&self) -> String {
        format!("
<tr>
  <td class='app_name'>{app_name}</td>
  <td class='seperator_v movable'></td>
  <td>{process_type}</td>
  <td class='seperator_v movable'></td>
  <td>{process_name}</td>
  <td class='seperator_v movable'></td>
  <td>{command_line}</td>
  <td class='seperator_v cpu'></td>
  <td class='align_right width_fixed_100 very_dark_yellow cpu'>{cpu_percentage}%</td>
  <td class='seperator_v ram'></td>
  <td class='align_right width_fixed_100 dark_yellow ram'>{ram} MB</td>
  <td class='seperator_v disk'></td>
  <td class='align_right width_fixed_100 middle_yellow disk'>{disk} MB/s</td>
  <td class='seperator_v network'></td>
  <td class='align_right width_fixed_100 light_yellow network'>{network} Mbps</td>
</tr>", 
        app_name = self.name,
        process_type = self.process_type.to_str(),
        process_name = self.process_name,
        command_line = self.command_line,
        cpu_percentage = format!("{:.1}", self.cpu_percentage),
        ram = format!("{:.1}", self.memory),
        disk = format!("{:.1}", self.disk),
        network = format!("{:.1}", self.network),
)
    }
}