// Linux: libwebkit2gtk-4.0-dev

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate urlencoding;
extern crate web_view;
extern crate procinfo;

use urlencoding::encode;
use web_view::*;

const APP_TITLE: &str = "Task Manager";

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    done: bool,
}

#[derive(Deserialize)]
#[serde(tag = "cmd")]
pub enum Cmd {
    #[serde(rename = "init")]
    Init,
    #[serde(rename = "log")]
    Log { text: String },
    #[serde(rename = "update_stuff")]
    UpdateStuff,
}

fn webview_cb<'a>(webview: &mut WebView<'a, Vec<Task>>, arg: &str, tasks: &mut Vec<Task>) {
    
    match serde_json::from_str::<Cmd>(arg) {
        Ok(arg) => {
            match arg {
                Cmd::Init => (),
                Cmd::Log { text } => println!("{}", text),
                Cmd::UpdateStuff => { println!("Updating stuff"); }
            }
        },
        Err(e) => {
            println!("[Rust] Got garbage RPC from JS: {:?}\r\n\r\n{:?}", e, arg);
        }
    }

    webview.eval(&format!("rpc.render({})", serde_json::to_string(tasks).unwrap()));
}

fn main() {
    let app_html = include_str!("dist/app.html");
    let url = "data:text/html,".to_string() + &encode(&app_html);
    let size = (840, 630);
    let resizable = true;
    let debug = true;

    let init_cb = |_webview| {};

    let userdata = vec![];
    let (_, launched_successful) = run(APP_TITLE, &url, Some(size), resizable, debug, init_cb, |webview, arg, tasks: &mut Vec<Task>| {
        webview_cb(webview, arg, tasks);
    }, userdata);

    if !launched_successful {
        println!("failed to launch {}", env!("CARGO_PKG_NAME"));
    }
}