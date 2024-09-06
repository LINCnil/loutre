use crate::theme::Theme;
use eframe::egui;

#[derive(Clone, Copy, Debug)]
pub enum Color {
	ButtonBackground,
	ButtonBorder,
	ButtonText,
	ButtonBackgroundHovered,
	ButtonBorderHovered,
	ButtonTextHovered,

	FileSelectionBackground,
	FileSelection,

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

	Title,
}

impl Color {
	pub fn get(&self, theme: Theme) -> egui::Color32 {
		match self {
			Self::ButtonBackground => BaseColor::C_E5EAFF.to_egui_color(),
			Self::ButtonBorder => BaseColor::C_001D96.to_egui_color(),
			Self::ButtonText => BaseColor::C_001D96.to_egui_color(),
			Self::ButtonBackgroundHovered => BaseColor::C_6045FF.to_egui_color(),
			Self::ButtonBorderHovered => BaseColor::C_001D96.to_egui_color(),
			Self::ButtonTextHovered => BaseColor::C_FFFFFF.to_egui_color(),
			Self::FileSelectionBackground => match theme {
				Theme::Dark => BaseColor::C_4F5D76,
				Theme::Light => BaseColor::C_F0F2FB,
			}
			.to_egui_color(),
			Self::FileSelection => match theme {
				Theme::Dark => BaseColor::C_FFF8E6,
				Theme::Light => BaseColor::C_001D96,
			}
			.to_egui_color(),
			Self::InfoBoxErrorBackground => match theme {
				Theme::Dark => BaseColor::C_840000,
				Theme::Light => BaseColor::C_FFF0F0,
			}
			.to_egui_color(),
			Self::InfoBoxErrorBorder => match theme {
				Theme::Dark => BaseColor::C_FF7575,
				Theme::Light => BaseColor::C_AC2121,
			}
			.to_egui_color(),
			Self::InfoBoxErrorIcon => match theme {
				Theme::Dark => BaseColor::C_FF7575,
				Theme::Light => BaseColor::C_AC2121,
			}
			.to_egui_color(),
			Self::InfoBoxErrorText => match theme {
				Theme::Dark => BaseColor::C_FFF0F0,
				Theme::Light => BaseColor::C_AC2121,
			}
			.to_egui_color(),
			Self::InfoBoxInfoBackground => match theme {
				Theme::Dark => BaseColor::C_1D2F59,
				Theme::Light => BaseColor::C_BBE4FF,
			}
			.to_egui_color(),
			Self::InfoBoxInfoBorder => BaseColor::C_348CFF.to_egui_color(),
			Self::InfoBoxInfoIcon => BaseColor::C_348CFF.to_egui_color(),
			Self::InfoBoxInfoText => match theme {
				Theme::Dark => BaseColor::C_E6F1FF,
				Theme::Light => BaseColor::C_414141,
			}
			.to_egui_color(),
			Self::InfoBoxSuccessBackground => match theme {
				Theme::Dark => BaseColor::C_144132,
				Theme::Light => BaseColor::C_E5FFF0,
			}
			.to_egui_color(),
			Self::InfoBoxSuccessBorder => match theme {
				Theme::Dark => BaseColor::C_34FF86,
				Theme::Light => BaseColor::C_21AC59,
			}
			.to_egui_color(),
			Self::InfoBoxSuccessIcon => match theme {
				Theme::Dark => BaseColor::C_34FF86,
				Theme::Light => BaseColor::C_21AC59,
			}
			.to_egui_color(),
			Self::InfoBoxSuccessText => match theme {
				Theme::Dark => BaseColor::C_DEFFEB,
				Theme::Light => BaseColor::C_21AC59,
			}
			.to_egui_color(),
			Self::InfoBoxWarningBackground => match theme {
				Theme::Dark => BaseColor::C_4C3F2B,
				Theme::Light => BaseColor::C_FFF8E5,
			}
			.to_egui_color(),
			Self::InfoBoxWarningBorder => BaseColor::C_FFD151.to_egui_color(),
			Self::InfoBoxWarningIcon => BaseColor::C_FFD151.to_egui_color(),
			Self::InfoBoxWarningText => match theme {
				Theme::Dark => BaseColor::C_FFF8E6,
				Theme::Light => BaseColor::C_414141,
			}
			.to_egui_color(),
			Self::MainFrameBackground => match theme {
				Theme::Dark => BaseColor::C_17172F,
				Theme::Light => BaseColor::C_F8F8F8,
			}
			.to_egui_color(),
			Self::MainText => match theme {
				Theme::Dark => BaseColor::C_E5EAFF,
				Theme::Light => BaseColor::C_414141,
			}
			.to_egui_color(),
			Self::Title => {
				if cfg!(feature = "nightly") {
					BaseColor::C_2242FF.to_egui_color()
				} else {
					match theme {
						Theme::Dark => BaseColor::C_BBE4FF,
						Theme::Light => BaseColor::C_001D96,
					}
					.to_egui_color()
				}
			}
		}
	}
}

// [Red, Green, Blue, Alpha]
struct BaseColor([u8; 4]);

impl BaseColor {
	const C_001D96: Self = Self([0x00, 0x1d, 0x96, 0xff]);
	const C_144132: Self = Self([0x14, 0x41, 0x32, 0xff]);
	const C_17172F: Self = Self([0x17, 0x17, 0x2f, 0xff]);
	const C_1D2F59: Self = Self([0x1d, 0x2f, 0x59, 0xff]);
	const C_21AC59: Self = Self([0x21, 0xac, 0x59, 0xff]);
	const C_2242FF: Self = Self([0x22, 0x42, 0xff, 0xff]);
	const C_348CFF: Self = Self([0x34, 0x8c, 0xff, 0xff]);
	const C_34FF86: Self = Self([0x34, 0xff, 0x86, 0xff]);
	const C_4C3F2B: Self = Self([0x4c, 0x3f, 0x2b, 0xff]);
	const C_4F5D76: Self = Self([0x4f, 0x5d, 0x76, 0xff]);
	const C_6045FF: Self = Self([0x60, 0x45, 0xff, 0xff]);
	const C_840000: Self = Self([0x84, 0x00, 0x00, 0xff]);
	const C_414141: Self = Self([0x41, 0x41, 0x41, 0xff]);
	const C_AC2121: Self = Self([0xac, 0x21, 0x21, 0xff]);
	const C_BBE4FF: Self = Self([0xbb, 0xe4, 0xff, 0xff]);
	const C_DEFFEB: Self = Self([0xde, 0xff, 0xeb, 0xff]);
	const C_E5EAFF: Self = Self([0xe5, 0xea, 0xff, 0xff]);
	const C_E5FFF0: Self = Self([0xe5, 0xff, 0xf0, 0xff]);
	const C_E6F1FF: Self = Self([0xe6, 0xf1, 0xff, 0xff]);
	const C_F8F8F8: Self = Self([0xf8, 0xf8, 0xf8, 0xff]);
	const C_FFD151: Self = Self([0xff, 0xd1, 0x51, 0xff]);
	const C_F0F2FB: Self = Self([0xf0, 0xf2, 0xfb, 0xff]);
	const C_FF7575: Self = Self([0xff, 0x75, 0x75, 0xff]);
	const C_FFF0F0: Self = Self([0xff, 0xf0, 0xf0, 0xff]);
	const C_FFF8E5: Self = Self([0xff, 0xf8, 0xe5, 0xff]);
	const C_FFF8E6: Self = Self([0xff, 0xf8, 0xe6, 0xff]);
	const C_FFFFFF: Self = Self([0xff, 0xff, 0xff, 0xff]);

	fn to_egui_color(&self) -> egui::Color32 {
		egui::Color32::from_rgba_unmultiplied(self.0[0], self.0[1], self.0[2], self.0[3])
	}
}
