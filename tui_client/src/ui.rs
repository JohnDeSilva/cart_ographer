use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::{App, Screen, Focus, api};
use crate::api::UserRole;

fn location_detail_lines(loc: &api::Location) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    lines.push(Line::from(vec![
        Span::styled(" Location:    ", Style::default().add_modifier(Modifier::BOLD).fg(Color::LightCyan)),
        Span::styled(loc.formatted.clone(), Style::default().fg(Color::White)),
    ]));
    if let Some(ref d) = loc.description {
        lines.push(Line::from(vec![
            Span::styled("   Description:", Style::default().fg(Color::Gray)),
            Span::styled(format!(" {}", d), Style::default().fg(Color::DarkGray)),
        ]));
    }
    if let (Some(lat), Some(lng)) = (loc.lat, loc.lng) {
        lines.push(Line::from(vec![
            Span::styled("   GPS:        ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:.4}, {:.4}", lat, lng), Style::default().fg(Color::DarkGray)),
        ]));
    }
    if let Some(ref a) = loc.address {
        let mut addr = a.clone();
        if let Some(ref c) = loc.city { addr = format!("{}, {}", addr, c); }
        if let Some(ref s) = loc.state { addr = format!("{}, {}", addr, s); }
        if let Some(ref z) = loc.zip_code { addr = format!("{} {}", addr, z); }
        lines.push(Line::from(vec![
            Span::styled("   Address:    ", Style::default().fg(Color::Gray)),
            Span::styled(addr, Style::default().fg(Color::DarkGray)),
        ]));
    }
    if let Some(ref r1) = loc.road_1 {
        let r2 = loc.road_2.as_deref().unwrap_or("");
        lines.push(Line::from(vec![
            Span::styled("   Intersection:", Style::default().fg(Color::Gray)),
            Span::styled(format!(" {} & {}", r1, r2), Style::default().fg(Color::DarkGray)),
        ]));
    }
    if let Some(ref v) = loc.venue_name {
        let mut venue = v.clone();
        if let Some(ref s) = loc.stall_number {
            venue = format!("{}, Stall {}", venue, s);
        }
        lines.push(Line::from(vec![
            Span::styled("   Venue:      ", Style::default().fg(Color::Gray)),
            Span::styled(venue, Style::default().fg(Color::DarkGray)),
        ]));
    }
    if let Some(ref l) = loc.lot_name {
        lines.push(Line::from(vec![
            Span::styled("   Lot:        ", Style::default().fg(Color::Gray)),
            Span::styled(format!(" {}", l), Style::default().fg(Color::DarkGray)),
        ]));
    }
    lines
}

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
            "[Tab] Switch Field  [Space] Cycle/Insert Space  [Enter] Save  [Esc] Cancel"
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

            let loc_lines = location_detail_lines(&r.location);
            let insert_at = detail_text.len() - 6;
            for (i, l) in loc_lines.into_iter().enumerate() {
                detail_text.insert(insert_at + i, l);
            }

            if r.location_change_pending {
                if let Some(ref ploc) = r.pending_location {
                    detail_text.push(Line::from(""));
                    detail_text.push(Line::from(vec![
                        Span::styled(" Location Change Pending: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
                        Span::styled(ploc.formatted.clone(), Style::default().fg(Color::White)),
                    ]));
                    let mut pend_lines = location_detail_lines(ploc);
                    for l in pend_lines.drain(..) {
                        detail_text.push(l);
                    }
                }
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

            let loc_lines = location_detail_lines(&r.location);
            let insert_at = 4;
            for (i, l) in loc_lines.into_iter().enumerate() {
                detail_text.insert(insert_at + i, l);
            }

            if r.location_change_pending {
                if let Some(ref ploc) = r.pending_location {
                    detail_text.push(Line::from(""));
                    detail_text.push(Line::from(vec![
                        Span::styled(" Location Change Pending: ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
                        Span::styled(ploc.formatted.clone(), Style::default().fg(Color::White)),
                    ]));
                    let mut pend_lines = location_detail_lines(ploc);
                    for l in pend_lines.drain(..) {
                        detail_text.push(l);
                    }
                }
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
                let loc_lines = location_detail_lines(&r.location);
                let insert_at = 4;
                for (i, l) in loc_lines.into_iter().enumerate() {
                    detail_text.insert(insert_at + i, l);
                }

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
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
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
    let loc_block = Block::default().borders(Borders::ALL).title(" Location (free-form) ").border_style(loc_style);
    let loc_para = Paragraph::new(app.form_location.clone()).block(loc_block);
    f.render_widget(loc_para, chunks[3]);

    let loc_desc_style = if let Focus::FormLocationDesc = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_desc_block = Block::default().borders(Borders::ALL).title(" Location Description ").border_style(loc_desc_style);
    let loc_desc_para = Paragraph::new(app.form_location_desc.clone()).block(loc_desc_block);
    f.render_widget(loc_desc_para, chunks[4]);

    let loc_lat_style = if let Focus::FormLocationLat = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_lat_block = Block::default().borders(Borders::ALL).title(" Latitude ").border_style(loc_lat_style);
    let loc_lat_para = Paragraph::new(app.form_location_lat.clone()).block(loc_lat_block);
    f.render_widget(loc_lat_para, chunks[5]);

    let loc_lng_style = if let Focus::FormLocationLng = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_lng_block = Block::default().borders(Borders::ALL).title(" Longitude ").border_style(loc_lng_style);
    let loc_lng_para = Paragraph::new(app.form_location_lng.clone()).block(loc_lng_block);
    f.render_widget(loc_lng_para, chunks[6]);

    let loc_addr_style = if let Focus::FormLocationAddress = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_addr_block = Block::default().borders(Borders::ALL).title(" Street Address ").border_style(loc_addr_style);
    let loc_addr_para = Paragraph::new(app.form_location_address.clone()).block(loc_addr_block);
    f.render_widget(loc_addr_para, chunks[7]);

    let loc_city_style = if let Focus::FormLocationCity = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_city_block = Block::default().borders(Borders::ALL).title(" City ").border_style(loc_city_style);
    let loc_city_para = Paragraph::new(app.form_location_city.clone()).block(loc_city_block);
    f.render_widget(loc_city_para, chunks[8]);

    let loc_state_style = if let Focus::FormLocationState = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_state_block = Block::default().borders(Borders::ALL).title(" State ").border_style(loc_state_style);
    let loc_state_para = Paragraph::new(app.form_location_state.clone()).block(loc_state_block);
    f.render_widget(loc_state_para, chunks[9]);

    let loc_zip_style = if let Focus::FormLocationZip = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_zip_block = Block::default().borders(Borders::ALL).title(" ZIP Code ").border_style(loc_zip_style);
    let loc_zip_para = Paragraph::new(app.form_location_zip.clone()).block(loc_zip_block);
    f.render_widget(loc_zip_para, chunks[10]);

    let loc_r1_style = if let Focus::FormLocationRoad1 = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_r1_block = Block::default().borders(Borders::ALL).title(" Intersection Road 1 ").border_style(loc_r1_style);
    let loc_r1_para = Paragraph::new(app.form_location_road1.clone()).block(loc_r1_block);
    f.render_widget(loc_r1_para, chunks[11]);

    let loc_r2_style = if let Focus::FormLocationRoad2 = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_r2_block = Block::default().borders(Borders::ALL).title(" Intersection Road 2 ").border_style(loc_r2_style);
    let loc_r2_para = Paragraph::new(app.form_location_road2.clone()).block(loc_r2_block);
    f.render_widget(loc_r2_para, chunks[12]);

    let loc_ven_style = if let Focus::FormLocationVenue = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_ven_block = Block::default().borders(Borders::ALL).title(" Venue / Mall Name ").border_style(loc_ven_style);
    let loc_ven_para = Paragraph::new(app.form_location_venue.clone()).block(loc_ven_block);
    f.render_widget(loc_ven_para, chunks[13]);

    let loc_stall_style = if let Focus::FormLocationStall = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_stall_block = Block::default().borders(Borders::ALL).title(" Stall / Cart Number ").border_style(loc_stall_style);
    let loc_stall_para = Paragraph::new(app.form_location_stall.clone()).block(loc_stall_block);
    f.render_widget(loc_stall_para, chunks[14]);

    let loc_lot_style = if let Focus::FormLocationLot = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let loc_lot_block = Block::default().borders(Borders::ALL).title(" Parking Lot Name ").border_style(loc_lot_style);
    let loc_lot_para = Paragraph::new(app.form_location_lot.clone()).block(loc_lot_block);
    f.render_widget(loc_lot_para, chunks[15]);

    let time_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[16]);

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
    f.render_widget(status_para, chunks[17]);

    let desc_style = if let Focus::FormDescription = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let desc_block = Block::default().borders(Borders::ALL).title(" Description ").border_style(desc_style);
    let desc_para = Paragraph::new(app.form_description.clone()).block(desc_block);
    f.render_widget(desc_para, chunks[18]);

    let menu_style = if let Focus::FormMenuItems = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let menu_block = Block::default().borders(Borders::ALL).title(" Menu Items ").border_style(menu_style);
    let menu_para = Paragraph::new(app.form_menu_items.clone()).block(menu_block);
    f.render_widget(menu_para, chunks[19]);

    let action_para = Paragraph::new("Press [Enter] to submit  -  [Esc] to discard changes")
        .alignment(ratatui::layout::Alignment::Center)
        .fg(Color::DarkGray);
    f.render_widget(action_para, chunks[20]);
}
