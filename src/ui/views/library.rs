use iced::widget::{button, column, container, mouse_area, row, scrollable, text, Space, checkbox, text_input};
use iced::{Alignment, Element, Length};

use crate::app::{AppState, Message, ViewMode, SortColumn, PlaylistDialogMode};
use crate::ui::theme;

pub fn view(state: &AppState) -> Element<'_, Message> {
    let sidebar = folder_sidebar(state);
    let track_list = track_list_view(state);

    let drag_handle = mouse_area(
        container(
            container(Space::new(Length::Fixed(2.0), Length::Fill))
                .style(move |_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(if state.dragging_sidebar || state.is_hovering_sidebar_resizer { theme::accent() } else { theme::surface0() })),
                    ..Default::default()
                })
        )
        .width(6.0)
        .height(Length::Fill)
        .center_x(Length::Fixed(6.0))
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(theme::base())),
            ..Default::default()
        })
    )
    .on_press(Message::SidebarDragStart)
    .on_enter(Message::HoverSidebarResizer(true))
    .on_exit(Message::HoverSidebarResizer(false))
    .interaction(iced::mouse::Interaction::ResizingHorizontally);

    row![sidebar, drag_handle, track_list]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn folder_sidebar(state: &AppState) -> Element<'_, Message> {
    let tab_btn = |mode: ViewMode, label: &'static str| {
        let is_active = state.view_mode == mode;
        let btn_text = text(label)
            .size(11)
            .font(crate::ui::icons::UI_FONT_BOLD)
            .color(if is_active { theme::accent() } else { theme::subtext() });
        
        button(btn_text)
            .on_press(Message::SelectViewMode(mode))
            .style(if is_active { iced::widget::button::secondary } else { iced::widget::button::text })
            .padding([4, 8])
    };

    let sidebar_search_input = text_input("Search...", &state.sidebar_search)
        .on_input(Message::SidebarSearchChanged)
        .padding(6)
        .size(12)
        .width(Length::Fill);

    let tabs = row![
        tab_btn(ViewMode::Artists, "Artists"),
        tab_btn(ViewMode::Albums, "Albums"),
        tab_btn(ViewMode::Genres, "Genres"),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    let sidebar_items: Element<Message> = match state.view_mode {
        ViewMode::Artists => {
            column(
                state.artists().into_iter().map(|artist| {
                    let is_selected = state.selected_artist.as_ref() == Some(&artist) && state.selected_playlist.is_none();

                    let label = text(artist.clone())
                        .color(if is_selected { theme::accent() } else { theme::text() })
                        .size(13);

                    let context_btn = button(
                        text("\u{f142}") // vertical ellipsis Nerd Font
                            .font(crate::ui::icons::NERD_FONT_MONO)
                            .color(theme::overlay0())
                            .size(13)
                    )
                    .on_press(Message::ToggleContextMenu(Some(crate::app::ContextMenuTarget::Artist(artist.clone()))))
                    .style(iced::widget::button::text);

                    let btn_row = row![
                        button(label)
                            .on_press(Message::SelectArtist(artist.clone()))
                            .style(iced::widget::button::text)
                            .width(Length::Fill)
                            .padding([6, 12]),
                        context_btn,
                        Space::with_width(4),
                    ]
                    .align_y(Alignment::Center);

                    let row_widget = mouse_area(btn_row)
                        .on_right_press(Message::ToggleContextMenu(Some(crate::app::ContextMenuTarget::Artist(artist.clone()))));

                    if is_selected {
                        container(row_widget).style(theme::selected_row).width(Length::Fill).into()
                    } else {
                        container(row_widget).width(Length::Fill).into()
                    }
                })
                .collect::<Vec<_>>(),
            )
            .spacing(2)
            .into()
        }
        ViewMode::Albums => {
            column(
                state.albums().into_iter().map(|album| {
                    let is_selected = state.selected_album.as_ref() == Some(&album) && state.selected_playlist.is_none();

                    let label = text(album.clone())
                        .color(if is_selected { theme::accent() } else { theme::text() })
                        .size(13);

                    let context_btn = button(
                        text("\u{f142}")
                            .font(crate::ui::icons::NERD_FONT_MONO)
                            .color(theme::overlay0())
                            .size(13)
                    )
                    .on_press(Message::ToggleContextMenu(Some(crate::app::ContextMenuTarget::Album(album.clone()))))
                    .style(iced::widget::button::text);

                    let btn_row = row![
                        button(label)
                            .on_press(Message::SelectAlbum(album.clone()))
                            .style(iced::widget::button::text)
                            .width(Length::Fill)
                            .padding([6, 12]),
                        context_btn,
                        Space::with_width(4),
                    ]
                    .align_y(Alignment::Center);

                    let row_widget = mouse_area(btn_row)
                        .on_right_press(Message::ToggleContextMenu(Some(crate::app::ContextMenuTarget::Album(album.clone()))));

                    if is_selected {
                        container(row_widget).style(theme::selected_row).width(Length::Fill).into()
                    } else {
                        container(row_widget).width(Length::Fill).into()
                    }
                })
                .collect::<Vec<_>>(),
            )
            .spacing(2)
            .into()
        }
        ViewMode::Genres => {
            column(
                state.genres().into_iter().map(|genre| {
                    let is_selected = state.selected_genre.as_ref() == Some(&genre) && state.selected_playlist.is_none();

                    let label = text(genre.clone())
                        .color(if is_selected { theme::accent() } else { theme::text() })
                        .size(13);

                    let btn = button(label)
                        .on_press(Message::SelectGenre(genre.clone()))
                        .style(iced::widget::button::text)
                        .width(Length::Fill)
                        .padding([6, 12]);

                    if is_selected {
                        container(btn).style(theme::selected_row).width(Length::Fill).into()
                    } else {
                        container(btn).width(Length::Fill).into()
                    }
                })
                .collect::<Vec<_>>(),
            )
            .spacing(2)
            .into()
        }
    };




    let render_playlist_item = |name: String, is_auto: bool| -> Element<'_, Message> {
        let is_selected = state.selected_playlist.as_ref() == Some(&name);

        let icon_str = if name == "Liked Songs" {
            crate::ui::icons::ICON_HEART
        } else if name == "Most Played" {
            crate::ui::icons::ICON_PODIUM
        } else if name == "Recently Played" {
            "\u{f017}"
        } else {
            crate::ui::icons::ICON_MUSIC
        };

        let is_custom = !is_auto;

        let label_row = row![
            text(icon_str)
                .font(crate::ui::icons::NERD_FONT_MONO)
                .color(if is_selected { theme::accent() } else { theme::overlay0() })
                .size(14),
            Space::with_width(8),
            text(name.clone())
                .color(if is_selected { theme::accent() } else if is_auto { theme::subtext() } else { theme::text() })
                .font(if is_auto { crate::ui::icons::UI_FONT_BOLD } else { crate::ui::icons::UI_FONT })
                .size(14),
        ]
        .align_y(Alignment::Center);

        let is_hovered = state.hovered_playlist.as_ref() == Some(&name);

        let btn = if is_custom {
            let rename_btn = button(
                text("\u{f044}")
                    .font(crate::ui::icons::NERD_FONT_MONO)
                    .color(theme::overlay0())
                    .size(12)
            )
            .on_press(Message::OpenPlaylistDialog(PlaylistDialogMode::Rename(name.clone())))
            .style(iced::widget::button::text);

            let delete_btn = button(
                text("\u{f1f8}")
                    .font(crate::ui::icons::NERD_FONT_MONO)
                    .color(theme::red())
                    .size(12)
            )
            .on_press(Message::DeletePlaylist(name.clone()))
            .style(iced::widget::button::text);

            let mut action_row = row![
                button(label_row)
                    .on_press(Message::SelectPlaylist(name.clone()))
                    .style(iced::widget::button::text)
                    .width(Length::Fill)
                    .padding([6, 12])
            ];

            if is_hovered {
                action_row = action_row.push(rename_btn).push(Space::with_width(4)).push(delete_btn).push(Space::with_width(6));
            }

            action_row.align_y(Alignment::Center).width(Length::Fill)
        } else {
            row![
                button(label_row)
                    .on_press(Message::SelectPlaylist(name.clone()))
                    .style(iced::widget::button::text)
                    .width(Length::Fill)
                    .padding([6, 12])
            ]
            .width(Length::Fill)
        };

        let row_container = if is_selected {
            container(btn).style(theme::selected_row).width(Length::Fill)
        } else {
            container(btn).width(Length::Fill)
        };

        mouse_area(row_container)
            .on_enter(Message::HoverPlaylist(Some(name.clone())))
            .on_exit(Message::HoverPlaylist(None))
            .into()
    };



    let is_liked_selected = state.selected_playlist.as_deref() == Some("Liked Songs");
    let is_recent_selected = state.selected_playlist.as_deref() == Some("Recently Played");
    let is_most_selected = state.selected_playlist.as_deref() == Some("Most Played");

    let liked_btn = button(
        text(crate::ui::icons::ICON_HEART)
            .font(crate::ui::icons::NERD_FONT_MONO)
            .size(18)
            .color(if is_liked_selected { theme::accent() } else { theme::overlay0() })
    )
    .on_press(Message::SelectPlaylist("Liked Songs".to_string()))
    .style(iced::widget::button::text)
    .padding(2);

    let recent_btn = button(
        text("\u{f017}") // Clock
            .font(crate::ui::icons::NERD_FONT_MONO)
            .size(18)
            .color(if is_recent_selected { theme::accent() } else { theme::overlay0() })
    )
    .on_press(Message::SelectPlaylist("Recently Played".to_string()))
    .style(iced::widget::button::text)
    .padding(2);

    let most_btn = button(
        text(crate::ui::icons::ICON_PODIUM)
            .font(crate::ui::icons::NERD_FONT_MONO)
            .size(18)
            .color(if is_most_selected { theme::accent() } else { theme::overlay0() })
    )
    .on_press(Message::SelectPlaylist("Most Played".to_string()))
    .style(iced::widget::button::text)
    .padding(2);

    let mut user_playlists_col = column![].spacing(2);
    let custom_playlists = crate::db::get(|db| db.playlists.keys().cloned().collect::<Vec<String>>());
    for name in custom_playlists {
        user_playlists_col = user_playlists_col.push(render_playlist_item(name, false));
    }

    let compact_new_playlist_btn = button(
        text("\u{f07b}\u{f067}") // folder plus icon Nerd Font
            .font(crate::ui::icons::NERD_FONT_MONO)
            .size(14)
            .color(theme::accent())
    )
    .on_press(Message::OpenPlaylistDialog(PlaylistDialogMode::Create))
    .style(iced::widget::button::text)
    .padding(2);

    let playlists_header = row![
        text("Playlists")
            .color(theme::subtext())
            .size(11)
            .font(crate::ui::icons::UI_FONT_BOLD),
        Space::with_width(Length::Fill),
        row![
            liked_btn,
            Space::with_width(12),
            recent_btn,
            Space::with_width(12),
            most_btn,
        ]
        .align_y(Alignment::Center),
        Space::with_width(Length::Fill),
        compact_new_playlist_btn,
    ]
    .align_y(Alignment::Center)
    .padding([0, 4]);

    let playlist_drag_handle = mouse_area(
        container(
            container(Space::new(Length::Fill, Length::Fixed(2.0)))
                .style(move |_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(if state.dragging_playlist_split || state.is_hovering_playlist_resizer { theme::accent() } else { theme::surface0() })),
                    ..Default::default()
                })
        )
        .width(Length::Fill)
        .height(6.0)
        .center_y(Length::Fixed(6.0))
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(theme::mantle())),
            ..Default::default()
        })
    )
    .on_press(Message::PlaylistDragStart)
    .on_enter(Message::HoverPlaylistResizer(true))
    .on_exit(Message::HoverPlaylistResizer(false))
    .interaction(iced::mouse::Interaction::ResizingVertically);

    let sidebar_items_hover = mouse_area(scrollable(sidebar_items))
        .on_enter(Message::HoverSidebarList(true))
        .on_exit(Message::HoverSidebarList(false));

    let mut sidebar_items_col = column![sidebar_items_hover];
    if !state.hidden_artists_albums.is_empty() {
        let restore_btn = button(
            text("Restore Hidden Items")
                .size(11)
                .color(theme::accent())
        )
        .on_press(Message::RestoreHiddenItems)
        .style(iced::widget::button::text)
        .padding(4);
        sidebar_items_col = sidebar_items_col.push(Space::with_height(4)).push(restore_btn);
    }

    container(
        column![
            tabs,
            Space::with_height(6),
            sidebar_search_input,
            Space::with_height(8),
            container(sidebar_items_col)
                .height(Length::Fill),
            playlist_drag_handle,
            Space::with_height(8),
            container(
                column![
                    playlists_header,
                    Space::with_height(8),
                    container(scrollable(user_playlists_col))
                        .height(Length::Fill),
                ]
            )
            .height(Length::Fixed(state.playlist_height)),
        ]
        .padding(8),
    )
    .style(theme::sidebar)
    .width(state.sidebar_width)
    .height(Length::Fill)
    .into()
}

fn track_list_view(state: &AppState) -> Element<'_, Message> {
    if state.tracks.is_empty() {
        return container(
            text(if state.selected_folder.is_some() || state.selected_playlist.is_some() || !state.search_query.is_empty() {
                state.strings.no_tracks_found
            } else {
                state.strings.select_folder
            })
            .color(theme::overlay0())
            .size(15),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
    }

    let header_btn = |col: SortColumn, label: &str, width: Length| {
        let is_sorted = state.sort_column == Some(col);
        let arrow = if is_sorted {
            if state.sort_ascending { " ▲" } else { " ▼" }
        } else {
            ""
        };
        let txt = text(format!("{label}{arrow}"))
            .size(11)
            .font(crate::ui::icons::UI_FONT_BOLD)
            .color(if is_sorted { theme::accent() } else { theme::subtext() });
            
        button(txt)
            .on_press(Message::SortBy(col))
            .style(iced::widget::button::text)
            .padding(0)
            .width(width)
    };

    let is_recently_played = state.selected_playlist.as_deref() == Some("Recently Played");
    let group_by_album = state.group_by_album && !is_recently_played;

    let table_headers = if is_recently_played {
        container(
            row![
                header_btn(SortColumn::TrackNumber, "#", Length::Fixed(30.0)),
                header_btn(SortColumn::Title, "Title", Length::FillPortion(3)),
                header_btn(SortColumn::Artist, "Artist", Length::FillPortion(2)),
                text("Date Played")
                    .size(11)
                    .font(crate::ui::icons::UI_FONT_BOLD)
                    .color(theme::subtext())
                    .width(Length::FillPortion(2)),
                header_btn(SortColumn::Duration, "Duration", Length::Fixed(60.0)),
                header_btn(SortColumn::Plays, "Plays", Length::Fixed(40.0)),
                Space::with_width(120),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .padding([8, 12])
        )
    } else if group_by_album {
        container(
            row![
                header_btn(SortColumn::TrackNumber, "#", Length::Fixed(30.0)),
                header_btn(SortColumn::Title, "Title", Length::FillPortion(3)),
                header_btn(SortColumn::Artist, "Artist", Length::FillPortion(2)),
                header_btn(SortColumn::Duration, "Duration", Length::Fixed(60.0)),
                header_btn(SortColumn::Plays, "Plays", Length::Fixed(40.0)),
                Space::with_width(120),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .padding([8, 12])
        )
    } else {
        container(
            row![
                header_btn(SortColumn::TrackNumber, "#", Length::Fixed(30.0)),
                header_btn(SortColumn::Title, "Title", Length::FillPortion(3)),
                header_btn(SortColumn::Artist, "Artist", Length::FillPortion(2)),
                header_btn(SortColumn::Album, "Album", Length::FillPortion(2)),
                header_btn(SortColumn::Duration, "Duration", Length::Fixed(60.0)),
                header_btn(SortColumn::Plays, "Plays", Length::Fixed(40.0)),
                Space::with_width(120),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .padding([8, 12])
        )
    }
    .style(|_| iced::widget::container::Style {
        background: Some(iced::Background::Color(theme::mantle())),
        border: iced::Border {
            color: theme::surface0(),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    })
    .width(Length::Fill);

    let current_id = state.current_track.as_ref().map(|t| t.id);

    let mut rows: Vec<Element<Message>> = Vec::new();

    if group_by_album {
        // Group tracks by album keeping insertion order
        let mut groups: Vec<(String, Vec<&crate::library::models::Track>)> = Vec::new();
        for track in &state.tracks {
            if let Some(last) = groups.last_mut() {
                if last.0 == track.album {
                    last.1.push(track);
                    continue;
                }
            }
            groups.push((track.album.clone(), vec![track]));
        }

        for (album_name, tracks) in groups.into_iter() {
            let n = tracks.len();
            let header = container(
                row![
                    text(album_name)
                        .color(theme::accent())
                        .size(13)
                        .font(crate::ui::icons::UI_FONT_BOLD),
                    Space::with_width(Length::Fill),
                    text(state.strings.track_count(n))
                        .color(theme::overlay0())
                        .size(11),
                ]
                .align_y(Alignment::Center)
                .padding([6, 12]),
            )
            .style(theme::album_header)
            .width(Length::Fill);

            rows.push(header.into());

            for track in tracks.into_iter() {
                rows.push(render_track_row(state, track, true, current_id));
            }
            rows.push(Space::with_height(8).into());
        }
    } else {
        for track in &state.tracks {
            rows.push(render_track_row(state, track, false, current_id));
        }
    }

    let tracklist_scroll = mouse_area(scrollable(column(rows).spacing(1)).id(scrollable::Id::new("tracklist_scroll")))
        .on_enter(Message::HoverTracklist(true))
        .on_exit(Message::HoverTracklist(false));

    let shortcuts_btn = button(
        text("\u{f11c}")
            .font(crate::ui::icons::NERD_FONT_MONO)
            .color(theme::subtext())
            .size(13)
    )
    .on_press(Message::OpenShortcuts)
    .style(iced::widget::button::text)
    .padding(4);

    let song_search_input = text_input("Search songs...", &state.search_query)
        .on_input(Message::SearchChanged)
        .padding(6)
        .size(12)
        .width(Length::Fixed(300.0));

    let filter_options: Element<'_, Message> = if !state.search_query.is_empty() {
        container(
            row![
                checkbox("Title", state.filter_title).on_toggle(|_| Message::ToggleFilterTitle).size(14),
                checkbox("Artist", state.filter_artist).on_toggle(|_| Message::ToggleFilterArtist).size(14),
                checkbox("Album", state.filter_album).on_toggle(|_| Message::ToggleFilterAlbum).size(14),
                checkbox("Genre", state.filter_genre).on_toggle(|_| Message::ToggleFilterGenre).size(14),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
        )
        .padding([4, 12])
        .into()
    } else {
        Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into()
    };

    let toolbar = container(
        column![
            row![
                row![
                    checkbox("Group by Album", state.group_by_album)
                        .on_toggle(|_| Message::ToggleGroupByAlbum)
                        .size(16)
                ]
                .width(Length::FillPortion(1))
                .align_y(Alignment::Center),
                Space::with_width(Length::Fill),
                song_search_input,
                Space::with_width(Length::Fill),
                row![
                    Space::with_width(Length::Fill),
                    shortcuts_btn
                ]
                .width(Length::FillPortion(1))
                .align_y(Alignment::Center),
            ]
            .align_y(Alignment::Center),
            filter_options,
        ]
        .spacing(4)
        .padding([8, 12])
    )
    .style(|_| iced::widget::container::Style {
        background: Some(iced::Background::Color(theme::mantle())),
        border: iced::Border {
            color: theme::surface0(),
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    })
    .width(Length::Fill);

    column![
        table_headers,
        container(tracklist_scroll)
            .width(Length::Fill)
            .height(Length::Fill),
        toolbar,
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn render_track_row(
    state: &AppState,
    track: &crate::library::models::Track,
    grouped: bool,
    current_id: Option<i64>,
) -> Element<'static, Message> {
    let is_current = current_id == Some(track.id);
    let is_selected_track = state.selected_tracks.iter().any(|t| t.id == track.id);
    let row_color = if is_current { theme::accent() } else { theme::text() };

    let num = track.track_number
        .map(|n| n.to_string())
        .unwrap_or_else(|| "·".to_string());

    let like_color = if track.liked { theme::red() } else { theme::overlay0() };
    let like_btn = button(
        text(crate::ui::icons::ICON_HEART)
            .font(crate::ui::icons::NERD_FONT_MONO)
            .color(like_color)
            .size(13)
    )
    .on_press(Message::ToggleLikeTrack(track.clone()))
    .style(iced::widget::button::text);

    let edit_btn = button(
        text("\u{f044}")
            .font(crate::ui::icons::NERD_FONT_MONO)
            .color(theme::overlay0())
            .size(13)
    )
    .on_press(Message::OpenTagEditor(track.clone()))
    .style(iced::widget::button::text);

    let mut track_no_cover = track.clone();
    track_no_cover.cover_data = None;

    let add_playlist_btn = button(
        text(crate::ui::icons::ICON_PLUS)
            .font(crate::ui::icons::NERD_FONT_MONO)
            .color(theme::overlay0())
            .size(13)
    )
    .on_press(Message::OpenPlaylistDialog(PlaylistDialogMode::AddTrack(track_no_cover.clone())))
    .style(iced::widget::button::text);

    let mut track_row_widgets = vec![
        text(num)
            .color(theme::overlay0())
            .size(13)
            .width(30)
            .into(),
        text(track.title.clone())
            .color(row_color)
            .size(14)
            .width(Length::FillPortion(3))
            .into(),
        text(track.artist.clone())
            .color(theme::subtext())
            .size(13)
            .width(Length::FillPortion(2))
            .into(),
    ];

    let is_recently_played = state.selected_playlist.as_deref() == Some("Recently Played");
    if is_recently_played {
        let date_str = track.date_played.clone().unwrap_or_default();
        track_row_widgets.push(
            text(date_str)
                .color(theme::subtext())
                .size(13)
                .width(Length::FillPortion(2))
                .into()
        );
    } else if !grouped {
        track_row_widgets.push(
            text(track.album.clone())
                .color(theme::subtext())
                .size(13)
                .width(Length::FillPortion(2))
                .into()
        );
    }

    track_row_widgets.extend(vec![
        text(track.duration_str())
            .color(theme::subtext())
            .size(13)
            .width(60)
            .into(),
        text(track.play_count.to_string())
            .color(theme::subtext())
            .size(13)
            .width(40)
            .into(),
        like_btn.into(),
        edit_btn.into(),
        add_playlist_btn.into(),
    ]);

    let track_row = row(track_row_widgets)
        .spacing(12)
        .align_y(Alignment::Center)
        .padding([5, 12]);

    let styled = if is_selected_track {
        container(track_row).style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(theme::surface0())),
            border: iced::Border {
                color: theme::accent(),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .width(Length::Fill)
    } else if is_current {
        container(track_row).style(theme::selected_row).width(Length::Fill)
    } else {
        container(track_row).width(Length::Fill)
    };

    let select_btn = button(styled)
        .on_press(Message::SelectTrack(track.clone()))
        .style(iced::widget::button::text)
        .width(Length::Fill)
        .padding(0);

    let row_target = if state.selected_tracks.len() > 1 && state.selected_tracks.iter().any(|t| t.id == track.id) {
        crate::app::ContextMenuTarget::MultipleTracks(state.selected_tracks.clone())
    } else {
        crate::app::ContextMenuTarget::Track(track_no_cover)
    };

    mouse_area(select_btn)
        .on_right_press(Message::ToggleContextMenu(Some(row_target)))
        .into()
}
