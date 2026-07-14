use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::{App, Screen, Focus, api::UserRole};

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(size);

    draw_header(f, app, main_layout[0]);

    match app.screen {
        Screen::Login => draw_login_screen(f, app, main_layout[1]),
        Screen::Signup => draw_signup_screen(f, app, main_layout[1]),
        Screen::ResetPassword => draw_reset_screen(f, app, main_layout[1]),
        Screen::Dashboard => draw_dashboard(f, app, main_layout[1]),
        Screen::AddRestaurant | Screen::EditRestaurant => draw_form_screen(f, app, main_layout[1]),
        Screen::MyRestaurants => draw_my_restaurants_screen(f, app, main_layout[1]),
        Screen::FavoritesView => draw_favorites_screen(f, app, main_layout[1]),
    }

    draw_footer(f, app, main_layout[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let user_info = match (&app.api_client.username, &app.api_client.role) {
        (Some(u), Some(r)) => format!("Logged in as: {} ({:?})", u, r),
        _ => "Not logged in".to_string(),
    };

    let title = Paragraph::new(Line::from(vec![
        Span::styled(" CART_OGRAPHER ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled("Restaurant Tracker & Router", Style::default().fg(Color::Gray)),
    ]))
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)))
    .alignment(ratatui::layout::Alignment::Left);

    let info = Paragraph::new(Line::from(vec![
        Span::styled(user_info, Style::default().fg(Color::LightBlue)),
        Span::raw(" "),
    ]))
    .alignment(ratatui::layout::Alignment::Right);

    f.render_widget(title, area);

    let info_area = Rect {
        x: area.x + area.width.saturating_sub(45),
        y: area.y + 1,
        width: 43.min(area.width),
        height: 1,
    };
    f.render_widget(info, info_area);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let shortcuts = match app.screen {
        Screen::Login => "[Tab] Switch Field  [Enter] Login  [Ctrl+S] Signup  [Ctrl+R] Reset Password  [Esc] Exit",
        Screen::Signup => "[Tab] Switch Field  [Space] Cycle Role  [Enter] Register  [Esc] Back to Login",
        Screen::ResetPassword => "[Tab] Switch Field  [Enter] Reset Password  [Esc] Back to Login",
        Screen::Dashboard => {
            if app.api_client.role == Some(UserRole::Admin) {
                "[Tab] Switch Focus  [Up/Down] Navigate  [S] Search  [A] Add  [E] Edit  [T] Toggle  [D] Delete  [R] Approve  [L] Approve Loc  [Shift+L] Reject Loc  [Ctrl+L] Logout"
            } else if app.api_client.role == Some(UserRole::Customer) {
                "[Tab] Switch Focus  [Up/Down] Navigate  [S] Search  [M] My Restaurants  [Ctrl+L] Logout"
            } else {
                "[Tab] Switch Focus  [Up/Down] Navigate  [S] Search  [F] Add Favorite  [V] View Favorites  [Ctrl+L] Logout"
            }
        }
        Screen::MyRestaurants => {
            "[Tab] Switch Focus  [Up/Down] Navigate  [A] Submit New  [E] Edit  [T] Toggle Status  [D] Delete  [Ctrl+L] Logout  [Esc] Back"
        }
        Screen::FavoritesView => {
            "[Tab] Switch Focus  [Up/Down] Navigate  [D] Remove Favorite  [Ctrl+L] Logout  [Esc] Back"
        }
        Screen::AddRestaurant | Screen::EditRestaurant => {
            "[Tab] Switch Field  [Space] Cycle Type/Status  [Enter] Save  [Esc] Cancel"
        }
    };

    let text = if let Some(ref msg) = app.status_message {
        let color = if msg.to_lowercase().contains("fail") || msg.to_lowercase().contains("error") || msg.to_lowercase().contains("denied") {
            Color::Red
        } else {
            Color::Green
        };
        Line::from(vec![
            Span::styled(" STATUS: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(msg, Style::default().fg(color)),
        ])
    } else {
        Line::from(vec![
            Span::styled(" HOTKEYS: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(shortcuts, Style::default().fg(Color::Yellow)),
        ])
    };

    let footer = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));

    f.render_widget(footer, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn draw_login_screen(f: &mut Frame, app: &App, area: Rect) {
    let block_area = centered_rect(50, 45, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" SIGN IN ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    f.render_widget(Clear, block_area);
    f.render_widget(block, block_area);

    let inner_area = block_area.inner(ratatui::layout::Margin { horizontal: 2, vertical: 2 });
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(inner_area);

    let user_style = if let Focus::LoginUsername = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };
    let user_block = Block::default().borders(Borders::ALL).title(" Username ").border_style(user_style);
    let user_para = Paragraph::new(app.input_username.clone()).block(user_block);
    f.render_widget(user_para, chunks[0]);

    let pass_style = if let Focus::LoginPassword = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };
    let pass_block = Block::default().borders(Borders::ALL).title(" Password ").border_style(pass_style);
    let masked_pass: String = "*".repeat(app.input_password.len());
    let pass_para = Paragraph::new(masked_pass).block(pass_block);
    f.render_widget(pass_para, chunks[1]);

    let action_para = Paragraph::new("Press [Enter] to submit credentials")
        .alignment(ratatui::layout::Alignment::Center)
        .fg(Color::DarkGray);
    f.render_widget(action_para, chunks[2]);
}

fn draw_signup_screen(f: &mut Frame, app: &App, area: Rect) {
    let block_area = centered_rect(50, 55, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" CREATE ACCOUNT ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    f.render_widget(Clear, block_area);
    f.render_widget(block, block_area);

    let inner_area = block_area.inner(ratatui::layout::Margin { horizontal: 2, vertical: 2 });
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(inner_area);

    let user_style = if let Focus::SignupUsername = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };
    let user_block = Block::default().borders(Borders::ALL).title(" Username ").border_style(user_style);
    let user_para = Paragraph::new(app.input_username.clone()).block(user_block);
    f.render_widget(user_para, chunks[0]);

    let pass_style = if let Focus::SignupPassword = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };
    let pass_block = Block::default().borders(Borders::ALL).title(" Password ").border_style(pass_style);
    let masked_pass: String = "*".repeat(app.input_password.len());
    let pass_para = Paragraph::new(masked_pass).block(pass_block);
    f.render_widget(pass_para, chunks[1]);

    let role_style = if let Focus::SignupRole = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };
    let role_block = Block::default().borders(Borders::ALL).title(" Role (Press [Space] to toggle) ").border_style(role_style);
    let role_para = Paragraph::new(format!("{:?}", app.input_role)).block(role_block);
    f.render_widget(role_para, chunks[2]);

    let action_para = Paragraph::new("Press [Enter] to register new account")
        .alignment(ratatui::layout::Alignment::Center)
        .fg(Color::DarkGray);
    f.render_widget(action_para, chunks[3]);
}

fn draw_reset_screen(f: &mut Frame, app: &App, area: Rect) {
    let block_area = centered_rect(50, 45, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" PASSWORD RESET ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    f.render_widget(Clear, block_area);
    f.render_widget(block, block_area);

    let inner_area = block_area.inner(ratatui::layout::Margin { horizontal: 2, vertical: 2 });
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(inner_area);

    let user_style = if let Focus::ResetUsername = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };
    let user_block = Block::default().borders(Borders::ALL).title(" Target Username ").border_style(user_style);
    let user_para = Paragraph::new(app.input_username.clone()).block(user_block);
    f.render_widget(user_para, chunks[0]);

    let pass_style = if let Focus::ResetPassword = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };
    let pass_block = Block::default().borders(Borders::ALL).title(" New Password ").border_style(pass_style);
    let masked_pass: String = "*".repeat(app.input_password.len());
    let pass_para = Paragraph::new(masked_pass).block(pass_block);
    f.render_widget(pass_para, chunks[1]);

    let action_para = Paragraph::new("Press [Enter] to apply password change")
        .alignment(ratatui::layout::Alignment::Center)
        .fg(Color::DarkGray);
    f.render_widget(action_para, chunks[2]);
}

fn draw_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
        ])
        .split(area);

    let search_style = if let Focus::SearchInput = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let search_block = Block::default()
        .borders(Borders::ALL)
        .title(" Search Filter (Type to filter restaurant list) ")
        .border_style(search_style);

    let search_text = if app.search_query.is_empty() && !matches!(app.focus, Focus::SearchInput) {
        "Type matching name...".to_string()
    } else {
        app.search_query.clone()
    };
    let search_para = Paragraph::new(search_text).block(search_block);
    f.render_widget(search_para, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(chunks[1]);

    let list_style = if let Focus::RestaurantList = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let items: Vec<ListItem> = app.restaurants
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let status_indicator = if r.open_status {
                Span::styled(" [OPEN] ", Style::default().fg(Color::Green))
            } else {
                Span::styled(" [CLOSED]", Style::default().fg(Color::Red))
            };

            let approval_indicator = if r.is_approved {
                Span::styled(" [*]", Style::default().fg(Color::Green))
            } else {
                Span::styled(" [~]", Style::default().fg(Color::Yellow))
            };

            let type_indicator = format!(" ({})", r.restaurant_type.as_str());

            let mut style = Style::default().fg(Color::White);
            if Some(i) == app.selected_restaurant_index {
                style = style.fg(Color::Black).bg(Color::Cyan);
            }

            ListItem::new(Line::from(vec![
                Span::styled(format!(" {:<20}", r.name), Style::default()),
                status_indicator,
                approval_indicator,
                Span::styled(type_indicator, Style::default().fg(Color::Gray)),
            ]))
            .style(style)
        })
        .collect();

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(" Restaurants ")
        .border_style(list_style);

    let list_widget = List::new(items).block(list_block);
    f.render_widget(list_widget, body_chunks[0]);

    let details_style = if let Focus::DetailPane = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let details_block = Block::default()
        .borders(Borders::ALL)
        .title(" Selected Restaurant Details ")
        .border_style(details_style);

    if let Some(idx) = app.selected_restaurant_index {
        if let Some(r) = app.restaurants.get(idx) {
            let status_text = if r.open_status {
                Span::styled("Active (Open for business)", Style::default().fg(Color::Green))
            } else {
                Span::styled("Inactive (Closed)", Style::default().fg(Color::Red))
            };

            let approval_text = if r.is_approved {
                Span::styled("Approved", Style::default().fg(Color::Green))
            } else {
                Span::styled("Pending Approval", Style::default().fg(Color::Yellow))
            };

            let desc = r.description.clone().unwrap_or_else(|| "No description provided.".to_string());
            let menu = r.menu_items.clone().unwrap_or_else(|| "None listed".to_string());

            let mut detail_text = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(" Name:        ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    Span::styled(r.name.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled(" Type:        ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    Span::styled(r.restaurant_type.as_str(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled(" Cuisine:     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    Span::styled(r.cuisine_type.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled(" Location:    ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    Span::styled(r.location.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled(" Hours:       ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    Span::styled(format!("{} - {}", r.open_time, r.close_time), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled(" Status:      ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    status_text,
                ]),
                Line::from(vec![
                    Span::styled(" Approved:    ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    approval_text,
                ]),
                Line::from(""),
                Line::from(Span::styled(" Description:", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan))),
                Line::from(Span::styled(desc, Style::default().fg(Color::Gray))),
                Line::from(""),
                Line::from(Span::styled(" Menu Items:", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan))),
                Line::from(Span::styled(menu, Style::default().fg(Color::Gray))),
            ];

            if r.location_change_pending {
                let pending_loc = r.pending_location.clone().unwrap_or_default();
                detail_text.push(Line::from(""));
                detail_text.push(Line::from(vec![
                    Span::styled(" Location Change Pending: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
                    Span::styled(pending_loc, Style::default().fg(Color::White)),
                ]));
            }

            let details_para = Paragraph::new(detail_text)
                .block(details_block)
                .wrap(Wrap { trim: true });
            f.render_widget(details_para, body_chunks[1]);
        }
    } else {
        let details_para = Paragraph::new("\n No restaurant selected.\n Search and select an entry to view details.")
            .alignment(ratatui::layout::Alignment::Center)
            .fg(Color::DarkGray)
            .block(details_block);
        f.render_widget(details_para, body_chunks[1]);
    }
}

fn draw_my_restaurants_screen(f: &mut Frame, app: &App, area: Rect) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(area);

    let list_style = if let Focus::MyRestaurantsList = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let items: Vec<ListItem> = app.my_restaurants
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let status_indicator = if r.open_status {
                Span::styled(" [OPEN] ", Style::default().fg(Color::Green))
            } else {
                Span::styled(" [CLOSED]", Style::default().fg(Color::Red))
            };

            let approval_indicator = if r.is_approved {
                Span::styled(" [*]", Style::default().fg(Color::Green))
            } else {
                Span::styled(" [~]", Style::default().fg(Color::Yellow))
            };

            let type_indicator = format!(" ({})", r.restaurant_type.as_str());

            let mut style = Style::default().fg(Color::White);
            if Some(i) == app.selected_my_restaurant_index {
                style = style.fg(Color::Black).bg(Color::Cyan);
            }

            ListItem::new(Line::from(vec![
                Span::styled(format!(" {:<20}", r.name), Style::default()),
                status_indicator,
                approval_indicator,
                Span::styled(type_indicator, Style::default().fg(Color::Gray)),
            ]))
            .style(style)
        })
        .collect();

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(" My Restaurants ")
        .border_style(list_style);

    let list_widget = List::new(items).block(list_block);
    f.render_widget(list_widget, body_chunks[0]);

    let details_style = if let Focus::MyRestaurantsDetail = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let details_block = Block::default()
        .borders(Borders::ALL)
        .title(" Restaurant Details ")
        .border_style(details_style);

    if let Some(idx) = app.selected_my_restaurant_index {
        if let Some(r) = app.my_restaurants.get(idx) {
            let status_text = if r.open_status {
                Span::styled("Active (Open for business)", Style::default().fg(Color::Green))
            } else {
                Span::styled("Inactive (Closed)", Style::default().fg(Color::Red))
            };

            let approval_text = if r.is_approved {
                Span::styled("Approved", Style::default().fg(Color::Green))
            } else {
                Span::styled("Pending Approval", Style::default().fg(Color::Yellow))
            };

            let desc = r.description.clone().unwrap_or_else(|| "No description provided.".to_string());
            let menu = r.menu_items.clone().unwrap_or_else(|| "None listed".to_string());

            let mut detail_text = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(" Name:        ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    Span::styled(r.name.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled(" Type:        ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    Span::styled(r.restaurant_type.as_str(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled(" Cuisine:     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    Span::styled(r.cuisine_type.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled(" Location:    ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    Span::styled(r.location.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled(" Hours:       ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    Span::styled(format!("{} - {}", r.open_time, r.close_time), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled(" Status:      ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    status_text,
                ]),
                Line::from(vec![
                    Span::styled(" Approved:    ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                    approval_text,
                ]),
                Line::from(""),
                Line::from(Span::styled(" Description:", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan))),
                Line::from(Span::styled(desc, Style::default().fg(Color::Gray))),
                Line::from(""),
                Line::from(Span::styled(" Menu Items:", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan))),
                Line::from(Span::styled(menu, Style::default().fg(Color::Gray))),
            ];

            if r.location_change_pending {
                let pending_loc = r.pending_location.clone().unwrap_or_default();
                detail_text.push(Line::from(""));
                detail_text.push(Line::from(vec![
                    Span::styled(" Location Change Pending: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
                    Span::styled(pending_loc, Style::default().fg(Color::White)),
                ]));
            }

            let details_para = Paragraph::new(detail_text)
                .block(details_block)
                .wrap(Wrap { trim: true });
            f.render_widget(details_para, body_chunks[1]);
        }
    } else {
        let details_para = Paragraph::new("\n No restaurant selected.\n Select an entry to view details.")
            .alignment(ratatui::layout::Alignment::Center)
            .fg(Color::DarkGray)
            .block(details_block);
        f.render_widget(details_para, body_chunks[1]);
    }
}

fn draw_favorites_screen(f: &mut Frame, app: &App, area: Rect) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .split(area);

    let list_style = if let Focus::FavoritesList = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let items: Vec<ListItem> = app.favorites
        .iter()
        .enumerate()
        .map(|(i, fave)| {
            let restaurant_name = fave.restaurant.as_ref()
                .map(|r| r.name.clone())
                .unwrap_or_else(|| format!("Restaurant #{}", fave.restaurant_id));

            let restaurant_type_str = fave.restaurant.as_ref()
                .map(|r| format!(" ({})", r.restaurant_type.as_str()))
                .unwrap_or_default();

            let mut style = Style::default().fg(Color::White);
            if Some(i) == app.selected_favorite_index {
                style = style.fg(Color::Black).bg(Color::Cyan);
            }

            ListItem::new(Line::from(vec![
                Span::styled(format!(" {:<20}", restaurant_name), Style::default()),
                Span::styled(restaurant_type_str, Style::default().fg(Color::Gray)),
            ]))
            .style(style)
        })
        .collect();

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(" My Favorites ")
        .border_style(list_style);

    let list_widget = List::new(items).block(list_block);
    f.render_widget(list_widget, body_chunks[0]);

    let details_style = if let Focus::FavoritesDetail = app.focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let details_block = Block::default()
        .borders(Borders::ALL)
        .title(" Favorite Details ")
        .border_style(details_style);

    if let Some(idx) = app.selected_favorite_index {
        if let Some(fave) = app.favorites.get(idx) {
            if let Some(r) = &fave.restaurant {
                let status_text = if r.open_status {
                    Span::styled("Active (Open for business)", Style::default().fg(Color::Green))
                } else {
                    Span::styled("Inactive (Closed)", Style::default().fg(Color::Red))
                };

                let desc = r.description.clone().unwrap_or_else(|| "No description provided.".to_string());
                let menu = r.menu_items.clone().unwrap_or_else(|| "None listed".to_string());

                let detail_text = vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled(" Name:        ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                        Span::styled(r.name.clone(), Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                        Span::styled(" Type:        ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                        Span::styled(r.restaurant_type.as_str(), Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                        Span::styled(" Cuisine:     ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                        Span::styled(r.cuisine_type.clone(), Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                        Span::styled(" Location:    ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                        Span::styled(r.location.clone(), Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                        Span::styled(" Hours:       ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                        Span::styled(format!("{} - {}", r.open_time, r.close_time), Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                        Span::styled(" Status:      ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
                        status_text,
                    ]),
                    Line::from(""),
                    Line::from(Span::styled(" Description:", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan))),
                    Line::from(Span::styled(desc, Style::default().fg(Color::Gray))),
                    Line::from(""),
                    Line::from(Span::styled(" Menu Items:", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan))),
                    Line::from(Span::styled(menu, Style::default().fg(Color::Gray))),
                ];

                let details_para = Paragraph::new(detail_text)
                    .block(details_block)
                    .wrap(Wrap { trim: true });
                f.render_widget(details_para, body_chunks[1]);
            } else {
                let details_para = Paragraph::new("\n Restaurant details not available.")
                    .alignment(ratatui::layout::Alignment::Center)
                    .fg(Color::DarkGray)
                    .block(details_block);
                f.render_widget(details_para, body_chunks[1]);
            }
        }
    } else {
        let details_para = Paragraph::new("\n No favorites selected.\n Press [V] on dashboard to add favorites.")
            .alignment(ratatui::layout::Alignment::Center)
            .fg(Color::DarkGray)
            .block(details_block);
        f.render_widget(details_para, body_chunks[1]);
    }
}

fn draw_form_screen(f: &mut Frame, app: &App, area: Rect) {
    let block_area = centered_rect(65, 90, area);
    let title = match app.screen {
        Screen::AddRestaurant => " ADD RESTAURANT ",
        Screen::EditRestaurant => " EDIT RESTAURANT ",
        _ => " RESTAURANT FORM ",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_alignment(ratatui::layout::Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    f.render_widget(Clear, block_area);
    f.render_widget(block, block_area);

    let inner_area = block_area.inner(ratatui::layout::Margin { horizontal: 2, vertical: 2 });
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(inner_area);

    let name_style = if let Focus::FormName = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let name_block = Block::default().borders(Borders::ALL).title(" Name ").border_style(name_style);
    let name_para = Paragraph::new(app.form_name.clone()).block(name_block);
    f.render_widget(name_para, chunks[0]);

    let type_style = if let Focus::FormType = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let type_block = Block::default().borders(Borders::ALL).title(" Type (Press [Space] to toggle) ").border_style(type_style);
    let type_para = Paragraph::new(app.form_type.as_str()).block(type_block);
    f.render_widget(type_para, chunks[1]);

    let cuisine_style = if let Focus::FormCuisineType = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let cuisine_block = Block::default().borders(Borders::ALL).title(" Cuisine Type ").border_style(cuisine_style);
    let cuisine_para = Paragraph::new(app.form_cuisine_type.clone()).block(cuisine_block);
    f.render_widget(cuisine_para, chunks[2]);

    let loc_style = if let Focus::FormLocation = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_block = Block::default().borders(Borders::ALL).title(" Location ").border_style(loc_style);
    let loc_para = Paragraph::new(app.form_location.clone()).block(loc_block);
    f.render_widget(loc_para, chunks[3]);

    let time_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[4]);

    let open_style = if let Focus::FormOpenTime = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let open_block = Block::default().borders(Borders::ALL).title(" Open Time (HH:MM:SS) ").border_style(open_style);
    let open_para = Paragraph::new(app.form_open_time.clone()).block(open_block);
    f.render_widget(open_para, time_chunks[0]);

    let close_style = if let Focus::FormCloseTime = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let close_block = Block::default().borders(Borders::ALL).title(" Close Time (HH:MM:SS) ").border_style(close_style);
    let close_para = Paragraph::new(app.form_close_time.clone()).block(close_block);
    f.render_widget(close_para, time_chunks[1]);

    let status_style = if let Focus::FormOpenStatus = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let status_block = Block::default().borders(Borders::ALL).title(" Status (Press [Space] to toggle) ").border_style(status_style);
    let status_text = if app.form_open_status { "Open" } else { "Closed" };
    let status_para = Paragraph::new(status_text).block(status_block);
    f.render_widget(status_para, chunks[5]);

    let desc_style = if let Focus::FormDescription = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let desc_block = Block::default().borders(Borders::ALL).title(" Description ").border_style(desc_style);
    let desc_para = Paragraph::new(app.form_description.clone()).block(desc_block);
    f.render_widget(desc_para, chunks[6]);

    let menu_style = if let Focus::FormMenuItems = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let menu_block = Block::default().borders(Borders::ALL).title(" Menu Items ").border_style(menu_style);
    let menu_para = Paragraph::new(app.form_menu_items.clone()).block(menu_block);
    f.render_widget(menu_para, chunks[7]);

    let action_para = Paragraph::new("Press [Enter] to submit  -  [Esc] to discard changes")
        .alignment(ratatui::layout::Alignment::Center)
        .fg(Color::DarkGray);
    f.render_widget(action_para, chunks[8]);
}
