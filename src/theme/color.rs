use crate::theme::Theme;
use eframe::egui;

#[derive(Clone, Copy, Debug)]
pub enum Color {
	InfoBoxErrorBackground,
	InfoBoxErrorBorder,
	InfoBoxErrorIcon,
	InfoBoxErrorText,

	InfoBoxInfoBackground,
	InfoBoxInfoBorder,
	InfoBoxInfoIcon,
	InfoBoxInfoText,

	InfoBoxSuccessBackground,
	InfoBoxSuccessBorder,
	InfoBoxSuccessIcon,
	InfoBoxSuccessText,

	InfoBoxWarningBackground,
	InfoBoxWarningBorder,
	InfoBoxWarningIcon,
	InfoBoxWarningText,

	MainFrameBackground,
	MainText,
}

impl Color {
	pub fn get(&self, theme: Theme) -> egui::Color32 {
		match self {
			Self::InfoBoxErrorBackground => BaseColor::RED_1.to_egui_color(),
			Self::InfoBoxErrorBorder => BaseColor::RED_3.to_egui_color(),
			Self::InfoBoxErrorIcon => BaseColor::RED_2.to_egui_color(),
			Self::InfoBoxErrorText => BaseColor::RED_2.to_egui_color(),
			Self::InfoBoxInfoBackground => match theme {
				Theme::Dark => BaseColor::BLUE_3.to_egui_color(),
				Theme::Light => BaseColor::BLUE_1.to_egui_color(),
				#[cfg(feature = "nightly")]
				Theme::NightlyDark | Theme::NightlyLight => BaseColor::BLUE_1.to_egui_color(),
			},
			Self::InfoBoxInfoBorder => BaseColor::BLUE_2.to_egui_color(),
			Self::InfoBoxInfoIcon => BaseColor::BLUE_2.to_egui_color(),
			Self::InfoBoxInfoText => match theme {
				Theme::Dark => BaseColor::BLUE_4.to_egui_color(),
				Theme::Light => BaseColor::GREY_1.to_egui_color(),
				#[cfg(feature = "nightly")]
				Theme::NightlyDark | Theme::NightlyLight => BaseColor::GREY_1.to_egui_color(),
			},
			Self::InfoBoxSuccessBackground => match theme {
				Theme::Dark => BaseColor::GREEN_4.to_egui_color(),
				Theme::Light => BaseColor::GREEN_1.to_egui_color(),
				#[cfg(feature = "nightly")]
				Theme::NightlyDark | Theme::NightlyLight => BaseColor::GREEN_1.to_egui_color(),
			},
			Self::InfoBoxSuccessBorder => match theme {
				Theme::Dark => BaseColor::GREEN_5.to_egui_color(),
				Theme::Light => BaseColor::GREEN_3.to_egui_color(),
				#[cfg(feature = "nightly")]
				Theme::NightlyDark | Theme::NightlyLight => BaseColor::GREEN_3.to_egui_color(),
			},
			Self::InfoBoxSuccessIcon => match theme {
				Theme::Dark => BaseColor::GREEN_5.to_egui_color(),
				Theme::Light => BaseColor::GREEN_2.to_egui_color(),
				#[cfg(feature = "nightly")]
				Theme::NightlyDark | Theme::NightlyLight => BaseColor::GREEN_2.to_egui_color(),
			},
			Self::InfoBoxSuccessText => match theme {
				Theme::Dark => BaseColor::GREEN_6.to_egui_color(),
				Theme::Light => BaseColor::GREEN_2.to_egui_color(),
				#[cfg(feature = "nightly")]
				Theme::NightlyDark | Theme::NightlyLight => BaseColor::GREEN_2.to_egui_color(),
			},
			Self::InfoBoxWarningBackground => match theme {
				Theme::Dark => BaseColor::ORANGE_3.to_egui_color(),
				Theme::Light => BaseColor::ORANGE_1.to_egui_color(),
				#[cfg(feature = "nightly")]
				Theme::NightlyDark | Theme::NightlyLight => BaseColor::ORANGE_1.to_egui_color(),
			},
			Self::InfoBoxWarningBorder => BaseColor::ORANGE_2.to_egui_color(),
			Self::InfoBoxWarningIcon => BaseColor::ORANGE_2.to_egui_color(),
			Self::InfoBoxWarningText => match theme {
				Theme::Dark => BaseColor::ORANGE_4.to_egui_color(),
				Theme::Light => BaseColor::GREY_1.to_egui_color(),
				#[cfg(feature = "nightly")]
				Theme::NightlyDark | Theme::NightlyLight => BaseColor::GREY_1.to_egui_color(),
			},
			Self::MainFrameBackground => match theme {
				Theme::Dark => BaseColor::GREY_3.to_egui_color(),
				Theme::Light => BaseColor::GREY_2.to_egui_color(),
				#[cfg(feature = "nightly")]
				Theme::NightlyDark | Theme::NightlyLight => BaseColor::GREY_2.to_egui_color(),
			},
			Self::MainText => match theme {
				Theme::Dark => BaseColor::GREY_4.to_egui_color(),
				Theme::Light => BaseColor::GREY_1.to_egui_color(),
				#[cfg(feature = "nightly")]
				Theme::NightlyDark | Theme::NightlyLight => BaseColor::GREY_1.to_egui_color(),
			},
		}
	}
}

// [Red, Green, Blue, Alpha]
struct BaseColor([u8; 4]);

impl BaseColor {
	const BLUE_1: Self = Self([0xbb, 0xe4, 0xff, 0xff]); // #BBE4FF
	const BLUE_2: Self = Self([0x34, 0x8c, 0xff, 0xff]); // #348CFF
	const BLUE_3: Self = Self([0x34, 0x8c, 0xff, 0x33]); // #348CFF 20%
	const BLUE_4: Self = Self([0xe6, 0xf1, 0xff, 0xff]); // #E6F1FF
	const GREEN_1: Self = Self([0xe5, 0xff, 0xf0, 0xff]); // #E5FFF0
	const GREEN_2: Self = Self([0x21, 0xac, 0x59, 0xff]); // #21AC59
	const GREEN_3: Self = Self([0x21, 0xac, 0x59, 0x33]); // #21AC59 20%
	const GREEN_4: Self = Self([0x10, 0x64, 0x34, 0xff]); // #106434
	const GREEN_5: Self = Self([0x34, 0xff, 0x86, 0xff]); // #34FF86
	const GREEN_6: Self = Self([0xde, 0xff, 0xeb, 0xff]); // #DEFFEB
	const GREY_1: Self = Self([0x41, 0x41, 0x41, 0xff]); // #414141
	const GREY_2: Self = Self([0xf8, 0xf8, 0xf8, 0xff]); // #f8f8f8
	const GREY_3: Self = Self([0x17, 0x17, 0x2f, 0xff]); // #17172F
	const GREY_4: Self = Self([0xe5, 0xea, 0xff, 0xff]); // #E5EAFF
	const ORANGE_1: Self = Self([0xff, 0xf8, 0xe5, 0xff]); // #FFF8E5
	const ORANGE_2: Self = Self([0xff, 0xd1, 0x51, 0xff]); // #FFD151
	const ORANGE_3: Self = Self([0x9b, 0x7b, 0x23, 0x66]); // #9B7B23 40%
	const ORANGE_4: Self = Self([0xff, 0xf8, 0xe6, 0xff]); // #FFF8E6
	const RED_1: Self = Self([0xff, 0xf0, 0xf0, 0xff]); // #FFF0F0
	const RED_2: Self = Self([0xdf, 0x45, 0x45, 0xff]); // #DF4545
	const RED_3: Self = Self([0xdf, 0x45, 0x45, 0x33]); // #DF4545 20%

	fn to_egui_color(&self) -> egui::Color32 {
		egui::Color32::from_rgba_premultiplied(self.0[0], self.0[1], self.0[2], self.0[3])
	}
}
