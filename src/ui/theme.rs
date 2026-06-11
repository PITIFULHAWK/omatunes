use iced::widget::container;
use iced::{Border, Color};

// ── Paleta Catppuccin Mocha + Lavanda (Omarchy) ───────────────────────────────

pub fn base() -> Color { hex(0x11, 0x11, 0x1b) }
pub fn mantle() -> Color { hex(0x18, 0x18, 0x25) }
pub fn surface0() -> Color { hex(0x31, 0x32, 0x44) }
pub fn overlay0() -> Color { hex(0x6c, 0x70, 0x86) }
pub fn text() -> Color { hex(0xcd, 0xd6, 0xf4) }
pub fn subtext() -> Color { hex(0xa6, 0xad, 0xc8) }
pub fn accent() -> Color { hex(0xcb, 0xa6, 0xf7) }
pub fn green() -> Color { hex(0xa6, 0xe3, 0xa1) }
pub fn red() -> Color { hex(0xf3, 0x8b, 0xa8) }
pub fn yellow() -> Color { hex(0xf9, 0xe2, 0xaf) }
pub fn blue() -> Color { hex(0x89, 0xb4, 0xfa) }

fn hex(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

pub fn with_alpha(c: Color, a: f32) -> Color {
    Color { a, ..c }
}

// ── Estilos de Container (assinatura Fn(&Theme) -> Style) ────────────────────

pub fn card(_: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(mantle())),
        border: Border {
            color: surface0(),
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}

pub fn header(_: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(mantle())),
        ..Default::default()
    }
}

pub fn sidebar(_: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(mantle())),
        border: Border {
            color: surface0(),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn selected_row(_: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(with_alpha(accent(), 0.15))),
        border: Border {
            color: with_alpha(accent(), 0.4),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

pub fn player_panel(_: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(mantle())),
        border: Border {
            color: surface0(),
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}

pub fn album_header(_: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(with_alpha(surface0(), 0.5))),
        border: Border {
            color: with_alpha(accent(), 0.2),
            width: 0.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    }
}

pub fn spectrum_bar_color(amplitude: f32) -> Color {
    if amplitude < 0.5 {
        lerp_color(green(), accent(), amplitude * 2.0)
    } else {
        lerp_color(accent(), red(), (amplitude - 0.5) * 2.0)
    }
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: 1.0,
    }
}
