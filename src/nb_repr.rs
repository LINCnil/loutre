use crate::i18n::{Attr, I18n};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NbRepr {
	Letters,
	#[serde(rename = "western arabic numerals")]
	WesternArabicNumerals,
}

impl NbRepr {
	pub fn usize_to_string(&self, nb: usize, i18n: &I18n) -> String {
		match self {
			NbRepr::Letters => format_letters(nb, i18n, true),
			NbRepr::WesternArabicNumerals => nb.to_string(),
		}
	}
}

impl Default for NbRepr {
	fn default() -> Self {
		NbRepr::WesternArabicNumerals
	}
}

fn format_letters(nb: usize, i18n: &I18n, first: bool) -> String {
	let mut parts = vec![];

	// Billions
	let (n, nb) = div_nb(nb, 1_000_000_000, "billion", i18n);
	if let Some(p) = n {
		parts.push(p);
	}

	// Millions
	let (n, nb) = div_nb(nb, 1_000_000, "million", i18n);
	if let Some(p) = n {
		parts.push(p);
	}

	// Thousands
	let (n, nb) = div_nb(nb, 1_000, "thousand", i18n);
	if let Some(p) = n {
		parts.push(p);
	}

	// Hundreds
	let (n, nb) = div_nb(nb, 100, "hundred", i18n);
	if let Some(p) = n {
		parts.push(p);
	}

	let main_sep = i18n.fmt("nb_main_sep", &[("space", Attr::String(String::from(" ")))]);
	let mut s = parts.join(&main_sep);

	if !parts.is_empty() && nb == 0 {
		return s;
	}
	if !parts.is_empty() && first {
		s += &i18n.fmt("nb_last_sep", &[("space", Attr::String(String::from(" ")))]);
	} else if !parts.is_empty() {
		s += &main_sep;
	}

	s += match nb {
		0 => i18n.msg("zero"),
		1 => i18n.msg("one"),
		2 => i18n.msg("two"),
		3 => i18n.msg("three"),
		4 => i18n.msg("four"),
		5 => i18n.msg("five"),
		6 => i18n.msg("six"),
		7 => i18n.msg("seven"),
		8 => i18n.msg("eight"),
		9 => i18n.msg("nine"),
		10 => i18n.msg("ten"),
		11 => i18n.msg("eleven"),
		12 => i18n.msg("twelve"),
		13 => i18n.msg("thirteen"),
		14 => i18n.msg("fourteen"),
		15 => i18n.msg("fifteen"),
		16 => i18n.msg("sixteen"),
		17 => i18n.msg("seventeen"),
		18 => i18n.msg("eighteen"),
		19 => i18n.msg("nineteen"),
		20 => i18n.msg("twenty"),
		21 => i18n.msg("twenty-one"),
		22 => i18n.msg("twenty-two"),
		23 => i18n.msg("twenty-three"),
		24 => i18n.msg("twenty-four"),
		25 => i18n.msg("twenty-five"),
		26 => i18n.msg("twenty-six"),
		27 => i18n.msg("twenty-seven"),
		28 => i18n.msg("twenty-eight"),
		29 => i18n.msg("twenty-nine"),
		30 => i18n.msg("thirty"),
		31 => i18n.msg("thirty-one"),
		32 => i18n.msg("thirty-two"),
		33 => i18n.msg("thirty-three"),
		34 => i18n.msg("thirty-four"),
		35 => i18n.msg("thirty-five"),
		36 => i18n.msg("thirty-six"),
		37 => i18n.msg("thirty-seven"),
		38 => i18n.msg("thirty-eight"),
		39 => i18n.msg("thirty-nine"),
		40 => i18n.msg("forty"),
		41 => i18n.msg("forty-one"),
		42 => i18n.msg("forty-two"),
		43 => i18n.msg("forty-three"),
		44 => i18n.msg("forty-four"),
		45 => i18n.msg("forty-five"),
		46 => i18n.msg("forty-six"),
		47 => i18n.msg("forty-seven"),
		48 => i18n.msg("forty-eight"),
		49 => i18n.msg("forty-nine"),
		50 => i18n.msg("fifty"),
		51 => i18n.msg("fifty-one"),
		52 => i18n.msg("fifty-two"),
		53 => i18n.msg("fifty-three"),
		54 => i18n.msg("fifty-four"),
		55 => i18n.msg("fifty-five"),
		56 => i18n.msg("fifty-six"),
		57 => i18n.msg("fifty-seven"),
		58 => i18n.msg("fifty-eight"),
		59 => i18n.msg("fifty-nine"),
		60 => i18n.msg("sixty"),
		61 => i18n.msg("sixty-one"),
		62 => i18n.msg("sixty-two"),
		63 => i18n.msg("sixty-three"),
		64 => i18n.msg("sixty-four"),
		65 => i18n.msg("sixty-five"),
		66 => i18n.msg("sixty-six"),
		67 => i18n.msg("sixty-seven"),
		68 => i18n.msg("sixty-eight"),
		69 => i18n.msg("sixty-nine"),
		70 => i18n.msg("seventy"),
		71 => i18n.msg("seventy-one"),
		72 => i18n.msg("seventy-two"),
		73 => i18n.msg("seventy-three"),
		74 => i18n.msg("seventy-four"),
		75 => i18n.msg("seventy-five"),
		76 => i18n.msg("seventy-six"),
		77 => i18n.msg("seventy-seven"),
		78 => i18n.msg("seventy-eight"),
		79 => i18n.msg("seventy-nine"),
		80 => i18n.msg("eighty"),
		81 => i18n.msg("eighty-one"),
		82 => i18n.msg("eighty-two"),
		83 => i18n.msg("eighty-three"),
		84 => i18n.msg("eighty-four"),
		85 => i18n.msg("eighty-five"),
		86 => i18n.msg("eighty-six"),
		87 => i18n.msg("eighty-seven"),
		88 => i18n.msg("eighty-eight"),
		89 => i18n.msg("eighty-nine"),
		90 => i18n.msg("ninety"),
		91 => i18n.msg("ninety-one"),
		92 => i18n.msg("ninety-two"),
		93 => i18n.msg("ninety-three"),
		94 => i18n.msg("ninety-four"),
		95 => i18n.msg("ninety-five"),
		96 => i18n.msg("ninety-six"),
		97 => i18n.msg("ninety-seven"),
		98 => i18n.msg("ninety-eight"),
		99 => i18n.msg("ninety-nine"),
		_ => String::new(),
	}
	.as_str();

	s
}

fn div_nb(nb: usize, d: usize, ds: &str, i18n: &I18n) -> (Option<String>, usize) {
	if let Some(n) = nb.checked_div(d) {
		if let Some(r) = nb.checked_rem(d) {
			let s = if n > 0 {
				let nb_str = format_letters(n, i18n, false);
				Some(i18n.fmt(
					ds,
					&[
						("nb", Attr::Usize(n)),
						("nb_str", Attr::String(nb_str)),
						("nb_after", Attr::Usize(r)),
						("space", Attr::String(String::from(" "))),
					],
				))
			} else {
				None
			};
			return (s, r);
		}
	}
	(None, 0)
}

#[cfg(test)]
mod tests {
	use super::NbRepr;
	use crate::i18n::I18n;

	#[test]
	fn test_wab() {
		let i18n = I18n::from_language_tag("fr-FR");
		let nb_repr = NbRepr::WesternArabicNumerals;
		let tests = [
			(0, "0"),
			(1, "1"),
			(5, "5"),
			(10, "10"),
			(21, "21"),
			(42, "42"),
			(70, "70"),
			(85, "85"),
			(92, "92"),
			(100, "100"),
			(101, "101"),
			(139, "139"),
			(400, "400"),
			(501, "501"),
			(673, "673"),
			(1000, "1000"),
			(1001, "1001"),
			(2000, "2000"),
			(2155, "2155"),
			(3200, "3200"),
			(10_000, "10000"),
			(10_008, "10008"),
			(12_108, "12108"),
			(345_678, "345678"),
			(1_000_000, "1000000"),
			(1_234_567, "1234567"),
			(123_456_789, "123456789"),
			(234_567_890, "234567890"),
			(1_000_000_000, "1000000000"),
			(1_234_567_890, "1234567890"),
			(98_123_456_789, "98123456789"),
			(798_123_456_789, "798123456789"),
			(6_798_123_456_789, "6798123456789"),
			(56_798_123_456_789, "56798123456789"),
			(456_798_123_456_789, "456798123456789"),
			(3_456_798_123_456_789, "3456798123456789"),
			(23_456_798_123_456_789, "23456798123456789"),
			(123_456_798_123_456_789, "123456798123456789"),
		];
		for (nb, nb_str) in tests {
			assert_eq!(nb_repr.usize_to_string(nb, &i18n), nb_str);
		}
	}

	#[test]
	fn test_letters_en_us() {
		let i18n = I18n::from_language_tag("en-US");
		let nb_repr = NbRepr::Letters;
		let tests = [
			(0, "zero"),
			(1, "one"),
			(5, "five"),
			(10, "ten"),
			(21, "twenty-one"),
			(42, "forty-two"),
			(70, "seventy"),
			(85, "eighty-five"),
			(92, "ninety-two"),
			(100, "one hundred"),
			(101, "one hundred and one"),
			(139, "one hundred and thirty-nine"),
			(400, "four hundred"),
			(501, "five hundred and one"),
			(673, "six hundred and seventy-three"),
			(1000, "one thousand"),
			(1001, "one thousand and one"),
			(2000, "two thousand"),
			(2155, "two thousand one hundred and fifty-five"),
			(3200, "three thousand two hundred"),
			(10_000, "ten thousand"),
			(10_008, "ten thousand and eight"),
			(12_108, "twelve thousand one hundred and eight"),
			(345_678, "three hundred forty-five thousand six hundred and seventy-eight"),
			(1_000_000, "one million"),
			(1_234_567, "one million two hundred thirty-four thousand five hundred and sixty-seven"),
			(123_456_789, "one hundred twenty-three million four hundred fifty-six thousand seven hundred and eighty-nine"),
			(234_567_890, "two hundred thirty-four million five hundred sixty-seven thousand eight hundred and ninety"),
			(1_000_000_000, "one billion"),
			(1_234_567_890, "one billion two hundred thirty-four million five hundred sixty-seven thousand eight hundred and ninety"),
			(98_123_456_789, "ninety-eight billion one hundred twenty-three million four hundred fifty-six thousand seven hundred and eighty-nine"),
			(798_123_456_789, "seven hundred ninety-eight billion one hundred twenty-three million four hundred fifty-six thousand seven hundred and eighty-nine"),
			(6_798_123_456_789, "six thousand seven hundred ninety-eight billion one hundred twenty-three million four hundred fifty-six thousand seven hundred and eighty-nine"),
			(56_798_123_456_789, "fifty-six thousand seven hundred ninety-eight billion one hundred twenty-three million four hundred fifty-six thousand seven hundred and eighty-nine"),
			(456_798_123_456_789, "four hundred fifty-six thousand seven hundred ninety-eight billion one hundred twenty-three million four hundred fifty-six thousand seven hundred and eighty-nine"),
			(3_456_798_123_456_789, "three million four hundred fifty-six thousand seven hundred ninety-eight billion one hundred twenty-three million four hundred fifty-six thousand seven hundred and eighty-nine"),
			(23_456_798_123_456_789, "twenty-three million four hundred fifty-six thousand seven hundred ninety-eight billion one hundred twenty-three million four hundred fifty-six thousand seven hundred and eighty-nine"),
			(123_456_798_123_456_789, "one hundred twenty-three million four hundred fifty-six thousand seven hundred ninety-eight billion one hundred twenty-three million four hundred fifty-six thousand seven hundred and eighty-nine"),
		];
		for (nb, nb_str) in tests {
			assert_eq!(nb_repr.usize_to_string(nb, &i18n), nb_str);
		}
	}

	#[test]
	fn test_letters_fr_fr() {
		let i18n = I18n::from_language_tag("fr-FR");
		let nb_repr = NbRepr::Letters;
		let tests = [
			(0, "zéro"),
			(1, "un"),
			(5, "cinq"),
			(10, "dix"),
			(21, "vingt-et-un"),
			(42, "quarante-deux"),
			(70, "soixante-dix"),
			(80, "quatre-vingts"),
			(85, "quatre-vingt-cinq"),
			(92, "quatre-vingt-douze"),
			(100, "cent"),
			(101, "cent-un"),
			(139, "cent-trente-neuf"),
			(180, "cent-quatre-vingts"),
			(400, "quatre-cent"),
			(501, "cinq-cent-un"),
			(673, "six-cent-soixante-treize"),
			(1000, "mille"),
			(1001, "mille-un"),
			(2000, "deux-mille"),
			(2155, "deux-mille-cent-cinquante-cinq"),
			(3200, "trois-mille-deux-cent"),
			(10_000, "dix-mille"),
			(10_008, "dix-mille-huit"),
			(12_108, "douze-mille-cent-huit"),
			(345_678, "trois-cent-quarante-cinq-mille-six-cent-soixante-dix-huit"),
			(1_000_000, "un-million"),
			(1_234_567, "un-million-deux-cent-trente-quatre-mille-cinq-cent-soixante-sept"),
			(123_456_789, "cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-neuf"),
			(234_567_890, "deux-cent-trente-quatre-millions-cinq-cent-soixante-sept-mille-huit-cent-quatre-vingt-dix"),
			(1_000_000_000, "un-milliard"),
			(1_234_567_890, "un-milliard-deux-cent-trente-quatre-millions-cinq-cent-soixante-sept-mille-huit-cent-quatre-vingt-dix"),
			(98_123_456_789, "quatre-vingt-dix-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-neuf"),
			(798_123_456_789, "sept-cent-quatre-vingt-dix-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-neuf"),
			(6_798_123_456_789, "six-mille-sept-cent-quatre-vingt-dix-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-neuf"),
			(56_798_123_456_789, "cinquante-six-mille-sept-cent-quatre-vingt-dix-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-neuf"),
			(456_798_123_456_789, "quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-dix-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-neuf"),
			(3_456_798_123_456_789, "trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-dix-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-neuf"),
			(23_456_798_123_456_789, "vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-dix-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-neuf"),
			(123_456_798_123_456_789, "cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-dix-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-quatre-vingt-neuf"),
		];
		for (nb, nb_str) in tests {
			assert_eq!(nb_repr.usize_to_string(nb, &i18n), nb_str);
		}
	}
}
