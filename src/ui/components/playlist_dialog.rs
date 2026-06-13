use iced::widget::{button, column, container, row, text, text_input, Space, checkbox, pick_list};
use iced::{Alignment, Element, Length};

use crate::app::{Message, PlaylistDialogState, PlaylistDialogMode};
use crate::ui::theme;

pub fn view(state: &PlaylistDialogState) -> Element<'static, Message> {
    let custom_playlists = crate::db::get(|db| db.playlists.keys().cloned().collect::<Vec<String>>());

    let title_text = match state.mode {
        PlaylistDialogMode::Create => "Create New Playlist",
        PlaylistDialogMode::AddTrack(_) => "Add to Playlist",
        PlaylistDialogMode::Rename(_) => "Rename Playlist",
    };

    let mut content = column![
        text(title_text)
            .size(18)
            .font(crate::ui::icons::UI_FONT_BOLD)
            .color(theme::accent()),
        Space::with_height(12),
    ];

    match &state.mode {
        PlaylistDialogMode::Create => {
            let name_input = text_input("Playlist Name", &state.name_input)
                .on_input(Message::PlaylistInputChanged)
                .padding(8);

            content = content.push(text("Name").size(12).color(theme::subtext()))
                .push(name_input)
                .push(Space::with_height(16));
        }
        PlaylistDialogMode::AddTrack(track) => {
            if custom_playlists.is_empty() {
                content = content.push(text("No custom playlists found. Create one first!").size(14).color(theme::red()))
                    .push(Space::with_height(12));
            } else {
                let current_selection = state.selected_playlist.clone().unwrap_or_else(|| custom_playlists[0].clone());
                let select_dropdown = pick_list(
                    custom_playlists.clone(),
                    Some(current_selection),
                    Message::PlaylistDialogSelect,
                )
                .padding(8);

                content = content.push(text("Select Playlist").size(12).color(theme::subtext()))
                    .push(select_dropdown)
                    .push(Space::with_height(12));
            }

            let add_album_chk = checkbox(format!("Add full album ({}) instead of just the track", track.album), state.add_album)
                .on_toggle(Message::PlaylistDialogToggleAddAlbum)
                .size(16);

            content = content.push(add_album_chk).push(Space::with_height(16));
        }
        PlaylistDialogMode::Rename(_) => {
            let name_input = text_input("New Playlist Name", &state.name_input)
                .on_input(Message::PlaylistInputChanged)
                .padding(8);

            content = content.push(text("New Name").size(12).color(theme::subtext()))
                .push(name_input)
                .push(Space::with_height(16));
        }
    }

    let submit_enabled = match &state.mode {
        PlaylistDialogMode::Create => !state.name_input.trim().is_empty(),
        PlaylistDialogMode::AddTrack(_) => state.selected_playlist.is_some() && !custom_playlists.is_empty(),
        PlaylistDialogMode::Rename(_) => !state.name_input.trim().is_empty(),
    };

    let submit_btn = if submit_enabled {
        button(text("Submit").color(theme::base()))
            .on_press(Message::PlaylistDialogSubmit)
            .padding([8, 16])
            .style(theme::primary_button)
    } else {
        button(text("Submit").color(theme::overlay0()))
            .padding([8, 16])
            .style(|_, _| iced::widget::button::Style {
                background: Some(iced::Background::Color(theme::surface0())),
                text_color: theme::overlay0(),
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
    };

    let buttons_row = row![
        button(text("Cancel").color(theme::text()))
            .on_press(Message::ClosePlaylistDialog)
            .padding([8, 16])
            .style(theme::secondary_button),
        Space::with_width(12),
        submit_btn,
    ]
    .align_y(Alignment::Center);

    let main_col = content.push(buttons_row)
        .spacing(4)
        .padding(24)
        .width(450);

    container(
        container(main_col)
            .style(|_| iced::widget::container::Style {
                background: Some(iced::Background::Color(theme::mantle())),
                border: iced::Border {
                    color: theme::surface0(),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            })
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(|_| iced::widget::container::Style {
        background: Some(iced::Background::Color(theme::with_alpha(theme::base(), 0.8))),
        ..Default::default()
    })
    .into()
}
