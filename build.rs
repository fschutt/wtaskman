fn main() {
    use std::fs::File;
    use std::io::Write;

    let main_html = include_str!("src/views/main_view/main.html");
    let main_html = main_html.replace("{{styles}}", &inline_style(include_str!("src/styles/main_view.css")));
    let main_html = main_html.replace("{{scripts}}", &inline_script(include_str!("src/views/main_view/view.js")));
    
    let mut file = File::create("./src/dist/app.html").unwrap();
    file.write_all(main_html.as_bytes()).unwrap();
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}