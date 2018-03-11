// Linux: libwebkit2gtk-4.0-dev

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate urlencoding;
extern crate web_view;

use urlencoding::encode;
use web_view::*;

fn webview_cb<'a>(webview: &mut WebView<'a, Vec<Task>>, arg: &str, tasks: &mut Vec<Task>) {
    use Cmd::*;
    match serde_json::from_str(arg).unwrap() {
        init => (),
        log { text } => println!("{}", text),
        addTask { name } => tasks.push(Task { name, done: false }),
        markTask { index, done } => tasks[index].done = done,
        clearDoneTasks => tasks.retain(|t| !t.done),
    }
    render(webview, tasks);
}

fn main() {
    let app_html = include_str!("dist/app.html");
    let url = "data:text/html,".to_string() + &encode(&app_html);
    let size = (840, 630);
    let resizable = true;
    let debug = true;
    let init_cb = |_webview| {};
    let userdata = vec![];
    let (_, launched_successful) = run("Rust Todo App", &url, Some(size), resizable, debug, init_cb, |webview, arg, tasks: &mut Vec<Task>| {
        webview_cb(webview, arg, tasks);
    }, userdata);

    if !launched_successful {
        println!("failed to launch {}", env!("CARGO_PKG_NAME"));
    }
}

fn render<'a, T>(webview: &mut WebView<'a, T>, tasks: &[Task]) {
    // println!("{:#?}", tasks);
    webview.eval(&format!("rpc.render({})", serde_json::to_string(tasks).unwrap()));
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    done: bool,
}

#[derive(Deserialize)]
#[serde(tag = "cmd")]
pub enum Cmd {
    init,
    log { text: String },
    addTask { name: String },
    markTask { index: usize, done: bool },
    clearDoneTasks,
}