#![allow(non_snake_case)]

use crate::app::Route;
use crate::components::{Header, MainSection, Root};
use crate::files::FileList;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn EmptyFiles() -> Element {
	let file_list = use_context::<Signal<FileList>>()();
	let nb_empty_files = file_list.nb_empty_files();

	rsx! {
		Root {
			Header {}
			MainSection {
				close_view: Some(Route::Main {}),
				h1 {
					{ t!("view_empty_files_title", nb: nb_empty_files) }
				}
				ul {
					for f in file_list.empty_files() {
						li {
							"{f.get_relative_path().display()}"
						}
					}
				}
			}
		}
	}
}
