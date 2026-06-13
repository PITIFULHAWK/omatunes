use iced::widget::{button, column, container, row, text, text_input, Space, checkbox};
use iced::{Alignment, Element, Length};

use crate::app::{Message, TagEditorState};
use crate::ui::theme;

pub fn view(state: &TagEditorState) -> Element<'static, Message> {
    let title_input = text_input("Title", &state.title)
        .on_input(Message::UpdateTagFieldTitle)
        .padding(8);

    let artist_input = text_input("Artist", &state.artist)
        .on_input(Message::UpdateTagFieldArtist)
        .padding(8);

    let album_input = text_input("Album", &state.album)
        .on_input(Message::UpdateTagFieldAlbum)
        .padding(8);

    let genre_input = text_input("Genre", &state.genre)
        .on_input(Message::UpdateTagFieldGenre)
        .padding(8);

    let track_num_input = text_input("Track Number", &state.track_number)
        .on_input(Message::UpdateTagFieldTrackNumber)
        .padding(8);

    let disc_num_input = text_input("Disc Number", &state.disc_number)
        .on_input(Message::UpdateTagFieldDiscNumber)
        .padding(8);

    let cover_path_val = state.cover_path.clone().unwrap_or_default();
    let cover_input = text_input("Cover Image Path (jpg/png)", &cover_path_val)
        .on_input(Message::UpdateTagFieldCoverPath)
        .padding(8);

    let apply_to_album_checkbox = checkbox(
        "Apply changes (Album, Genre, Cover) to entire album",
        state.apply_to_album,
    )
    .on_toggle(Message::UpdateTagFieldApplyToAlbum)
    .size(16);

    let content = column![
        text("Edit ID3 Tags")
            .size(18)
            .font(crate::ui::icons::UI_FONT_BOLD)
            .color(theme::accent()),
        Space::with_height(12),
        text("Title").size(12).color(theme::subtext()),
        title_input,
        Space::with_height(8),
        text("Artist").size(12).color(theme::subtext()),
        artist_input,
        Space::with_height(8),
        text("Album").size(12).color(theme::subtext()),
        album_input,
        Space::with_height(8),
        text("Genre").size(12).color(theme::subtext()),
        genre_input,
        Space::with_height(8),
        row![
            column![
                text("Track #").size(12).color(theme::subtext()),
                track_num_input
            ].width(Length::FillPortion(1)),
            Space::with_width(12),
            column![
                text("Disc #").size(12).color(theme::subtext()),
                disc_num_input
            ].width(Length::FillPortion(1)),
            Space::with_width(12),
            column![
                row![
                    text("Cover Path").size(12).color(theme::subtext()),
                    Space::with_width(Length::Fill),
                    button(text("Search Online").size(10))
                        .on_press(Message::SearchCoverOnline)
                        .style(theme::secondary_button)
                        .padding([2, 6])
                ].align_y(Alignment::Center),
                cover_input
            ].width(Length::FillPortion(2))
        ],
        Space::with_height(12),
        apply_to_album_checkbox,
        Space::with_height(16),
        row![
            button(text("Cancel").color(theme::text()))
                .on_press(Message::CloseTagEditor)
                .padding([8, 16])
                .style(theme::secondary_button),
            Space::with_width(12),
            button(text("Save").color(theme::base()))
                .on_press(Message::SaveTags)
                .padding([8, 16])
                .style(theme::primary_button)
        ]
        .align_y(Alignment::Center)
    ]
    .spacing(4)
    .padding(24)
    .width(450);

    // Modal background overlay
    container(
        container(content)
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
