#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use pulldown_cmark::{
    html, CodeBlockKind, Event, Options, Parser,
    Tag::CodeBlock,
};
use serde::Deserialize;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{
    as_24_bit_terminal_escaped, LinesWithEndings,
};
use syntect::{
    easy::HighlightLines, html::highlighted_html_for_string,
};

use once_cell::sync::OnceCell;

static PS: OnceCell<SyntaxSet> = OnceCell::new();
static TS: OnceCell<ThemeSet> = OnceCell::new();

fn main() {
    PS.set(SyntaxSet::load_defaults_newlines());
    TS.set(ThemeSet::load_defaults());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            render_markdown
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Deserialize)]
pub struct JSOptions {
    pub tables: Option<bool>,
    pub footnotes: Option<bool>,
    pub strikethrough: Option<bool>,
    pub tasklists: Option<bool>,
    pub smart_punctuation: Option<bool>,
    pub heading_attributes: Option<bool>,
}

#[tauri::command]
fn render_markdown(
    input: String,
    opts: Option<JSOptions>,
) -> String {
    let mut options = Options::empty();
    if let Some(js_options) = opts {
        if let Some(true) = js_options.tables {
            options.insert(Options::ENABLE_TABLES);
        }
        if let Some(true) = js_options.footnotes {
            options.insert(Options::ENABLE_FOOTNOTES);
        }
        if let Some(true) = js_options.strikethrough {
            options.insert(Options::ENABLE_STRIKETHROUGH);
        }
        if let Some(true) = js_options.tasklists {
            options.insert(Options::ENABLE_TASKLISTS);
        }
        if let Some(true) = js_options.smart_punctuation {
            options
                .insert(Options::ENABLE_SMART_PUNCTUATION);
        }
        if let Some(true) = js_options.heading_attributes {
            options
                .insert(Options::ENABLE_HEADING_ATTRIBUTES);
        }
    }

    let parser = Parser::new_ext(&input, options);
    // Write to String buffer.
    let mut html_output = String::new();
    let mut current_code = None;
    html::push_html(
        &mut html_output,
        parser
            .map(|event| match &event {
                e @ pulldown_cmark::Event::Start(
                    CodeBlock(CodeBlockKind::Fenced(lang)),
                ) => {
                    current_code = Some(lang.to_string());
                    event
                }
                e @ pulldown_cmark::Event::Text(code) => {
                    if let Some(lang) = &current_code {
                        let syntax = PS
                            .get()
                            .unwrap()
                            .find_syntax_by_extension("rs")
                            .unwrap();

                        let result =
                            highlighted_html_for_string(
                                &code,
                                &PS.get().unwrap(),
                                syntax,
                                &TS.get().unwrap().themes
                                    ["Solarized (light)"],
                            );
                        dbg!(&result);
                        current_code = None;
                        Event::Html(result.into())
                    } else {
                        event
                    }
                }

                e => {
                    println!("in event {:?}", &e);
                    event
                }
            })
            .filter(|event| match event {
                Event::Start(CodeBlock(_))
                | Event::End(CodeBlock(_)) => false,
                _ => true,
            }),
    );
    html_output
}
