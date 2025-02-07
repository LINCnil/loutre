#![allow(non_snake_case)]

use crate::app::Route;
use crate::components::{Header, MainSection, Root};
use crate::files::FileList;
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn DuplicatedFiles() -> Element {
	let file_list = use_context::<Signal<FileList>>()();

	rsx! {
		Root {
			Header {}
			MainSection {
				close_view: Some(Route::Main {}),
				h1 {
					{ tid!("view_duplicated_files_title") }
				}
				for lst in file_list.duplicated_files() {
					div {
						class: "view-duplicated-files-list",
						ul {
							for f in lst {
								li {
									"{f.get_relative_path().display()}"
								}
							}
						}
					}
				}
			}
		}
	}
}
