#![allow(non_snake_case)]

use crate::app::Route;
use crate::check::{CheckResult, CheckResultError};
use crate::components::{Header, MainSection, Root};
use crate::files::FileList;
use dioxus::prelude::*;
use dioxus_i18n::tid;

macro_rules! filter_err_type {
	($vec: ident, $err_type: ident) => {
		$vec.iter().filter_map(|e| match e {
			CheckResultError::$err_type(p) => Some(p.display().to_string()),
			_ => None,
		})
	};
}

#[component]
pub fn CheckErrors() -> Element {
	let file_list = use_context::<Signal<FileList>>()();

	rsx! {
		Root {
			Header {}
			MainSection {
				close_view: Some(Route::Main {}),
				h1 {
					{ tid!("view_check_errors_title") }
				}
				if let FileList::Hashed(lst) = file_list {
					if let CheckResult::Error(errors) = lst.get_result() {
						if errors.contains(&CheckResultError::ContentFileParseError) {
							p {
								{ tid!("view_check_errors_ctn_file_parse") }
							}
						}
						dl {
							class: "view-check-errors-err",
							for path in filter_err_type!(errors, ContentFileMissingFile) {
								Error {
									path: "{path}",
									message: tid!("view_check_errors_ctn_file_missing"),
								}
							}
							for path in filter_err_type!(errors, ContentFileNonMatchingFile) {
								Error {
									path: "{path}",
									message: tid!("view_check_errors_ctn_file_match"),
								}
							}
							for path in filter_err_type!(errors, ReceiptMissingFile) {
								Error {
									path: "{path}",
									message: tid!("view_check_errors_receipt_missing"),
								}
							}
							for path in filter_err_type!(errors, ReceiptNonMatchingFile) {
								Error {
									path: "{path}",
									message: tid!("view_check_errors_receipt_match"),
								}
							}
						}
					}
				}
			}
		}
	}
}

#[derive(PartialEq, Clone, Props)]
struct ErrorProps {
	path: String,
	message: String,
}

#[component]
fn Error(props: ErrorProps) -> Element {
	rsx! {
		dt {
			"{props.path}"
		}
		dd {
			"{props.message}"
		}
	}
}
