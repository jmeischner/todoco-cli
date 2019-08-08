use crate::helper;
use crate::terminal::{SearchTerm, TermDialog};
use crate::search::listterm::{
    KeywordControlTerm, KeywordSearchTerm, MainTerm
};
use console::{style, Term};
use std::io::Result as IOResult;
use std::path::PathBuf;
use types::{config::helper as types_helper, FilterMatch};

pub mod listterm;
mod printer;

pub use printer::tagprinter::TagPrinter;
pub use printer::textprinter::TextPrinter;
pub use printer::todoprinter::TodoPrinter;


/// todoco search opening dialog
pub fn start() -> IOResult<()> {
    let welcome_text: Vec<String> = include_str!("../../welcome_search.txt")
        .split("\n")
        .map(|line| line.to_string())
        .collect();
    let dialog = init_welcome_dialog(welcome_text);
    dialog.start()
}

fn init_welcome_dialog(lines: Vec<String>) -> TermDialog<String, TextPrinter, MainTerm> {
    let main_term = MainTerm::new(lines, Term::stdout());
    TermDialog::new(Term::stdout(), main_term)
}

/// todoco list dialog
pub fn list(keyword: Option<&str>, matches: FilterMatch, dir: PathBuf) -> IOResult<()> {
    let (is_project, _config) = types_helper::get_config_and_project_info_from(&dir);
    // todo: handle @error
    let project = todofilter::get_project(is_project, &dir).unwrap();
    let mut keyword_search_term = KeywordSearchTerm::new_from_filter_match(matches, Term::stdout())
        .set_project(project)
        .set_keyword(keyword.unwrap_or("").to_string());

    keyword_search_term.set_on_quit(|me, by_escape| {
        if by_escape {
            start_keyword_control_term(me)
        } else {
            Ok(())
        }
    });

    let dialog = TermDialog::new(Term::stdout(), keyword_search_term);
    dialog.start()?;

    let term = Term::stdout();
    term.clear_screen()?;
    let goodbye_line = helper::get_goodbye_message();
    term.write_line(&format!("{}", style(goodbye_line).bold()))
}

fn start_keyword_control_term(searchterm: KeywordSearchTerm) -> IOResult<()> {
    let term = KeywordControlTerm::new(
        searchterm.get_items().clone(),
        searchterm.get_term().clone(),
    )
    .set_project(searchterm.get_project())
    .set_keyword(searchterm.get_keyword())
    .set_headline(searchterm.headline());

    let dialog = TermDialog::new(searchterm.get_term().clone(), term);
    dialog.start()
}