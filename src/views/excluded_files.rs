#![allow(non_snake_case)]

use crate::app::Route;
use crate::components::{Header, MainSection, Root};
use crate::files::FileList;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn ExcludedFiles() -> Element {
	let file_list = use_context::<Signal<FileList>>()();
	let nb_excluded_files = file_list.nb_excluded_files();

	rsx! {
		Root {
			Header {}
			MainSection {
				close_view: Some(Route::Main {}),
				h1 {
					{ t!("view_excluded_files_title", nb: nb_excluded_files) }
				}
				ul {
					for f in file_list.excluded_files() {
						li {
							if f.is_hidden() {
								i {
									class: "ri-spy-line",
								}
							}
							if f.is_system() {
								i {
									class: "ri-tools-line",
								}
							}
							"{f.get_relative_path().display()}"
						}
					}
				}
			}
		}
	}
}
