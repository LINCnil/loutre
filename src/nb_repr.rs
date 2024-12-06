use dioxus_i18n::t;

pub fn usize_to_string(nb: usize) -> String {
	format_letters(nb, true)
}

fn format_letters(nb: usize, first: bool) -> String {
	let mut parts = vec![];

	// Billions
	let (n, nb) = div_nb(nb, 1_000_000_000, "cpn_nb_billion");
	if let Some(p) = n {
		parts.push(p);
	}

	// Millions
	let (n, nb) = div_nb(nb, 1_000_000, "cpn_nb_million");
	if let Some(p) = n {
		parts.push(p);
	}

	// Thousands
	let (n, nb) = div_nb(nb, 1_000, "cpn_nb_thousand");
	if let Some(p) = n {
		parts.push(p);
	}

	// Hundreds
	let (n, nb) = div_nb(nb, 100, "cpn_nb_hundred");
	if let Some(p) = n {
		parts.push(p);
	}

	let main_sep = t!("cpn_nb_main_sep", space: " ");
	let mut s = parts.join(&main_sep);

	if !parts.is_empty() && nb == 0 {
		return s;
	}
	if !parts.is_empty() && first {
		s += &t!("cpn_nb_last_sep", space: " ");
	} else if !parts.is_empty() {
		s += &main_sep;
	}

	s += match nb {
		0 => t!("cpn_nb_zero"),
		1 => t!("cpn_nb_one"),
		2 => t!("cpn_nb_two"),
		3 => t!("cpn_nb_three"),
		4 => t!("cpn_nb_four"),
		5 => t!("cpn_nb_five"),
		6 => t!("cpn_nb_six"),
		7 => t!("cpn_nb_seven"),
		8 => t!("cpn_nb_eight"),
		9 => t!("cpn_nb_nine"),
		10 => t!("cpn_nb_ten"),
		11 => t!("cpn_nb_eleven"),
		12 => t!("cpn_nb_twelve"),
		13 => t!("cpn_nb_thirteen"),
		14 => t!("cpn_nb_fourteen"),
		15 => t!("cpn_nb_fifteen"),
		16 => t!("cpn_nb_sixteen"),
		17 => t!("cpn_nb_seventeen"),
		18 => t!("cpn_nb_eighteen"),
		19 => t!("cpn_nb_nineteen"),
		20 => t!("cpn_nb_twenty"),
		21 => t!("cpn_nb_twenty-one"),
		22 => t!("cpn_nb_twenty-two"),
		23 => t!("cpn_nb_twenty-three"),
		24 => t!("cpn_nb_twenty-four"),
		25 => t!("cpn_nb_twenty-five"),
		26 => t!("cpn_nb_twenty-six"),
		27 => t!("cpn_nb_twenty-seven"),
		28 => t!("cpn_nb_twenty-eight"),
		29 => t!("cpn_nb_twenty-nine"),
		30 => t!("cpn_nb_thirty"),
		31 => t!("cpn_nb_thirty-one"),
		32 => t!("cpn_nb_thirty-two"),
		33 => t!("cpn_nb_thirty-three"),
		34 => t!("cpn_nb_thirty-four"),
		35 => t!("cpn_nb_thirty-five"),
		36 => t!("cpn_nb_thirty-six"),
		37 => t!("cpn_nb_thirty-seven"),
		38 => t!("cpn_nb_thirty-eight"),
		39 => t!("cpn_nb_thirty-nine"),
		40 => t!("cpn_nb_forty"),
		41 => t!("cpn_nb_forty-one"),
		42 => t!("cpn_nb_forty-two"),
		43 => t!("cpn_nb_forty-three"),
		44 => t!("cpn_nb_forty-four"),
		45 => t!("cpn_nb_forty-five"),
		46 => t!("cpn_nb_forty-six"),
		47 => t!("cpn_nb_forty-seven"),
		48 => t!("cpn_nb_forty-eight"),
		49 => t!("cpn_nb_forty-nine"),
		50 => t!("cpn_nb_fifty"),
		51 => t!("cpn_nb_fifty-one"),
		52 => t!("cpn_nb_fifty-two"),
		53 => t!("cpn_nb_fifty-three"),
		54 => t!("cpn_nb_fifty-four"),
		55 => t!("cpn_nb_fifty-five"),
		56 => t!("cpn_nb_fifty-six"),
		57 => t!("cpn_nb_fifty-seven"),
		58 => t!("cpn_nb_fifty-eight"),
		59 => t!("cpn_nb_fifty-nine"),
		60 => t!("cpn_nb_sixty"),
		61 => t!("cpn_nb_sixty-one"),
		62 => t!("cpn_nb_sixty-two"),
		63 => t!("cpn_nb_sixty-three"),
		64 => t!("cpn_nb_sixty-four"),
		65 => t!("cpn_nb_sixty-five"),
		66 => t!("cpn_nb_sixty-six"),
		67 => t!("cpn_nb_sixty-seven"),
		68 => t!("cpn_nb_sixty-eight"),
		69 => t!("cpn_nb_sixty-nine"),
		70 => t!("cpn_nb_seventy"),
		71 => t!("cpn_nb_seventy-one"),
		72 => t!("cpn_nb_seventy-two"),
		73 => t!("cpn_nb_seventy-three"),
		74 => t!("cpn_nb_seventy-four"),
		75 => t!("cpn_nb_seventy-five"),
		76 => t!("cpn_nb_seventy-six"),
		77 => t!("cpn_nb_seventy-seven"),
		78 => t!("cpn_nb_seventy-eight"),
		79 => t!("cpn_nb_seventy-nine"),
		80 => t!("cpn_nb_eighty"),
		81 => t!("cpn_nb_eighty-one"),
		82 => t!("cpn_nb_eighty-two"),
		83 => t!("cpn_nb_eighty-three"),
		84 => t!("cpn_nb_eighty-four"),
		85 => t!("cpn_nb_eighty-five"),
		86 => t!("cpn_nb_eighty-six"),
		87 => t!("cpn_nb_eighty-seven"),
		88 => t!("cpn_nb_eighty-eight"),
		89 => t!("cpn_nb_eighty-nine"),
		90 => t!("cpn_nb_ninety"),
		91 => t!("cpn_nb_ninety-one"),
		92 => t!("cpn_nb_ninety-two"),
		93 => t!("cpn_nb_ninety-three"),
		94 => t!("cpn_nb_ninety-four"),
		95 => t!("cpn_nb_ninety-five"),
		96 => t!("cpn_nb_ninety-six"),
		97 => t!("cpn_nb_ninety-seven"),
		98 => t!("cpn_nb_ninety-eight"),
		99 => t!("cpn_nb_ninety-nine"),
		_ => String::new(),
	}
	.as_str();

	s
}

fn div_nb(nb: usize, d: usize, ds: &str) -> (Option<String>, usize) {
	if let Some(n) = nb.checked_div(d) {
		if let Some(r) = nb.checked_rem(d) {
			let s = if n > 0 {
				let nb_str = format_letters(n, false);
				Some(t!(
					ds,
						nb: n,
						nb_str: nb_str,
						nb_after: r,
						space: " "
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
	use unic_langid::langid;

	#[test]
	fn test_wab() {
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
			assert_eq!(nb_repr.usize_to_string(nb), nb_str);
		}
	}

	#[ignore] // FIXME
	#[test]
	fn test_letters_en_us() {
		//crate::i18n::init_raw(langid!("en-US"));

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
			assert_eq!(nb_repr.usize_to_string(nb), nb_str);
		}
	}

	#[ignore] // FIXME
	#[test]
	fn test_letters_fr_fr() {
		//crate::i18n::init_raw(langid!("fr-FR"));

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
			assert_eq!(nb_repr.usize_to_string(nb), nb_str);
		}
	}

	#[ignore] // FIXME
	#[test]
	fn test_letters_fr_be() {
		//crate::i18n::init_raw(langid!("fr-BE"));

		let nb_repr = NbRepr::Letters;
		let tests = [
			(0, "zéro"),
			(1, "un"),
			(5, "cinq"),
			(10, "dix"),
			(21, "vingt-et-un"),
			(42, "quarante-deux"),
			(70, "septante"),
			(80, "octante"),
			(85, "octante-cinq"),
			(92, "nonante-deux"),
			(100, "cent"),
			(101, "cent-un"),
			(139, "cent-trente-neuf"),
			(180, "cent-octante"),
			(400, "quatre-cent"),
			(501, "cinq-cent-un"),
			(673, "six-cent-septante-trois"),
			(1000, "mille"),
			(1001, "mille-un"),
			(2000, "deux-mille"),
			(2155, "deux-mille-cent-cinquante-cinq"),
			(3200, "trois-mille-deux-cent"),
			(10_000, "dix-mille"),
			(10_008, "dix-mille-huit"),
			(12_108, "douze-mille-cent-huit"),
			(345_678, "trois-cent-quarante-cinq-mille-six-cent-septante-huit"),
			(1_000_000, "un-million"),
			(1_234_567, "un-million-deux-cent-trente-quatre-mille-cinq-cent-soixante-sept"),
			(123_456_789, "cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-octante-neuf"),
			(234_567_890, "deux-cent-trente-quatre-millions-cinq-cent-soixante-sept-mille-huit-cent-nonante"),
			(1_000_000_000, "un-milliard"),
			(1_234_567_890, "un-milliard-deux-cent-trente-quatre-millions-cinq-cent-soixante-sept-mille-huit-cent-nonante"),
			(98_123_456_789, "nonante-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-octante-neuf"),
			(798_123_456_789, "sept-cent-nonante-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-octante-neuf"),
			(6_798_123_456_789, "six-mille-sept-cent-nonante-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-octante-neuf"),
			(56_798_123_456_789, "cinquante-six-mille-sept-cent-nonante-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-octante-neuf"),
			(456_798_123_456_789, "quatre-cent-cinquante-six-mille-sept-cent-nonante-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-octante-neuf"),
			(3_456_798_123_456_789, "trois-millions-quatre-cent-cinquante-six-mille-sept-cent-nonante-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-octante-neuf"),
			(23_456_798_123_456_789, "vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-nonante-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-octante-neuf"),
			(123_456_798_123_456_789, "cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-nonante-huit-milliards-cent-vingt-trois-millions-quatre-cent-cinquante-six-mille-sept-cent-octante-neuf"),
		];
		for (nb, nb_str) in tests {
			assert_eq!(nb_repr.usize_to_string(nb), nb_str);
		}
	}
}
