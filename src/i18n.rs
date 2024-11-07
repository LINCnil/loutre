use crate::config::Config;
use dioxus_i18n::prelude::*;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use unic_langid::{langid, LanguageIdentifier};

#[derive(Clone, Debug)]
pub struct Lang(LanguageIdentifier);

impl Default for Lang {
	fn default() -> Self {
		Self(langid!("en-US"))
	}
}

impl Serialize for Lang {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.0.to_string())
	}
}

impl<'de> Deserialize<'de> for Lang {
	fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
		let s = String::deserialize(d)?;
		let lang_id: LanguageIdentifier = s.parse().map_err(de::Error::custom)?;
		Ok(lang_id.into())
	}
}

impl From<LanguageIdentifier> for Lang {
	fn from(l: LanguageIdentifier) -> Self {
		Self(l)
	}
}

impl From<Lang> for LanguageIdentifier {
	fn from(val: Lang) -> Self {
		val.0
	}
}

pub fn init(config: &Config) {
	let _ = use_init_i18n(|| {
		I18nConfig::new(config.lang.clone().into())
			.with_locale(Locale::new_static(
				langid!("en-US"),
				include_str!("../locale/en-US.ftl"),
			))
			.with_locale(Locale::new_static(
				langid!("fr-BE"),
				include_str!("../locale/fr-BE.ftl"),
			))
			.with_locale(Locale::new_static(
				langid!("fr-FR"),
				include_str!("../locale/fr-FR.ftl"),
			))
	});
}
