use crate::config::Config;
use dioxus_i18n::prelude::*;
use unic_langid::langid;

pub fn init(config: &Config) {
	let _ = use_init_i18n(|| {
		I18nConfig::new(config.lang.parse().unwrap_or(langid!("en-US")))
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
