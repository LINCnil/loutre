use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use unic_langid::LanguageIdentifier;

pub enum Attr {
	String(String),
	Usize(usize),
	U32(u32),
	U64(u64),
}

pub struct I18n {
	bundle: FluentBundle<FluentResource>,
}

impl I18n {
	pub fn from_language_tag(lang_tag: &str) -> Self {
		let lang_tag: LanguageIdentifier = lang_tag
			.parse()
			.unwrap_or_else(|_| crate::DEFAULT_LANG.parse().unwrap());
		let ressource = I18n::get_ressource(&lang_tag);
		let mut bundle = FluentBundle::new(vec![lang_tag]);
		bundle.add_resource(ressource).unwrap();
		Self { bundle }
	}

	pub fn msg(&self, key: &str) -> String {
		self.fmt(key, &[])
	}

	pub fn fmt(&self, key_name: &str, args: &[(&str, Attr)]) -> String {
		let mut errors = vec![];
		let mut fa = FluentArgs::new();
		let args = if !args.is_empty() {
			for (k, v) in args {
				match v {
					Attr::String(s) => fa.set(*k, FluentValue::from(s.as_str())),
					Attr::Usize(n) => fa.set(*k, FluentValue::from(*n)),
					Attr::U32(n) => fa.set(*k, FluentValue::from(*n)),
					Attr::U64(n) => fa.set(*k, FluentValue::from(*n)),
				}
			}
			Some(&fa)
		} else {
			None
		};
		let key: Vec<&str> = key_name.split('.').collect();
		match self.bundle.get_message(key[0]) {
			Some(msg) => match key.get(1) {
				Some(attr_name) => match msg.get_attribute(attr_name) {
					Some(attr) => self
						.bundle
						.format_pattern(attr.value(), args, &mut errors)
						.to_string(),
					None => format!("<{}>", key_name),
				},
				None => match msg.value() {
					Some(pattern) => self
						.bundle
						.format_pattern(pattern, args, &mut errors)
						.to_string(),
					None => format!("<{}>", key_name),
				},
			},
			None => format!("<{}>", key_name),
		}
	}

	fn get_ressource(lang_tag: &LanguageIdentifier) -> FluentResource {
		let s = match lang_tag.language.as_str() {
			"fr" => I18n::get_ressource_str("fr-FR"),
			_ => I18n::get_ressource_str(crate::DEFAULT_LANG),
		};
		FluentResource::try_new(s.to_string()).unwrap()
	}

	fn get_ressource_str(lang_tag: &str) -> &str {
		match lang_tag {
			"fr-FR" => include_str!("../locale/fr-FR.ftl"),
			_ => panic!("{}: unexpected language identifier", lang_tag),
		}
	}
}

#[cfg(test)]
mod tests {
	use fluent::FluentResource;

	#[test]
	fn test_language_files() {
		let res_lst = [("fr-FR", include_str!("../locale/fr-FR.ftl"))];
		for (tag, res) in res_lst {
			if let Err((_, e)) = FluentResource::try_new(res.to_string()) {
				assert!(false, "{}: unable to parse language file: {:?}", tag, e);
			}
		}
	}
}
