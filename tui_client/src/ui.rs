use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::{App, Screen, Focus, api};
use crate::api::UserRole;

fn menu_item_lines(items: &[api::MenuItem]) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    if items.is_empty() {
        lines.push(Line::from(Span::styled("  (No menu items)", Style::default().fg(Color::DarkGray))));
    } else {
        for item in items {
            let price_str = match item.price {
                Some(p) => format!("${:.2}", p),
                None => "".to_string(),
            };
            let status_str = if item.is_sold_out {
                Span::styled(" [SOLD OUT]", Style::default().fg(Color::Red))
            } else {
                Span::styled(" [Available]", Style::default().fg(Color::Green))
            };
            let desc_str = match &item.description {
                Some(d) if !d.is_empty() => format!(" - {}", d),
                _ => String::new(),
            };
            lines.push(Line::from(vec![
                Span::styled(format!("  {}  {}", item.name, price_str), Style::default().fg(Color::White)),
                status_str,
                Span::styled(desc_str, Style::default().fg(Color::Gray)),
            ]));
        }
    }
    lines
}

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
                "[Tab] Switch Focus  [Up/Down] Navigate  [S] Search  [A] Add  [E] Edit  [T] Toggle  [D] Delete  [R] Approve  [Ctrl+L] Logout"
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
            "[Tab] Switch Field  [Space] Cycle  [Enter] Save/Add Item  [A] Add Item  [T] Toggle Sold-out  [D] Delete Item  [Esc] Cancel"
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
            ];
            detail_text.extend(menu_item_lines(&r.menu_items));

            let loc_lines = location_detail_lines(&r.location);
            let insert_at = detail_text.len() - 6;
            for (i, l) in loc_lines.into_iter().enumerate() {
                detail_text.insert(insert_at + i, l);
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
            ];
            detail_text.extend(menu_item_lines(&r.menu_items));

            let loc_lines = location_detail_lines(&r.location);
            let insert_at = 4;
            for (i, l) in loc_lines.into_iter().enumerate() {
                detail_text.insert(insert_at + i, l);
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
                ];
                detail_text.extend(menu_item_lines(&r.menu_items));
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

fn input_field(label: &str, value: &str, focus: bool, area: Rect, f: &mut Frame) {
    let style = if focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let block = Block::default().borders(Borders::ALL).title(label).border_style(style);
    let para = Paragraph::new(value).block(block);
    f.render_widget(para, area);
}

fn draw_form_screen(f: &mut Frame, app: &App, area: Rect) {
    let block_area = centered_rect(85, 90, area);
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

    let inner_area = block_area.inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 });

    let vert = Layout::default()
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
            Constraint::Min(4),
            Constraint::Length(2),
        ])
        .split(inner_area);

    input_field(" Name ", &app.form_name, app.focus == Focus::FormName, vert[0], f);

    // Row: Type + Cuisine
    let r1 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(30), Constraint::Percentage(70)]).split(vert[1]);
    input_field(" Type [Space] ", app.form_type.as_str(), app.focus == Focus::FormType, r1[0], f);
    input_field(" Cuisine Type ", &app.form_cuisine_type, app.focus == Focus::FormCuisineType, r1[1], f);

    // Row: Location free-form + Location Description
    let r2 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(vert[2]);
    input_field(" Location (free-form) ", &app.form_location, app.focus == Focus::FormLocation, r2[0], f);
    input_field(" Location Description ", &app.form_location_desc, app.focus == Focus::FormLocationDesc, r2[1], f);

    // Row: Lat + Lng + Address
    let r3 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(15), Constraint::Percentage(15), Constraint::Percentage(70)]).split(vert[3]);
    input_field(" Lat ", &app.form_location_lat, app.focus == Focus::FormLocationLat, r3[0], f);
    input_field(" Lng ", &app.form_location_lng, app.focus == Focus::FormLocationLng, r3[1], f);
    input_field(" Street Address ", &app.form_location_address, app.focus == Focus::FormLocationAddress, r3[2], f);

    // Row: City + State + ZIP + Road1 + Road2
    let r4 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(25), Constraint::Percentage(12), Constraint::Percentage(15), Constraint::Percentage(24), Constraint::Percentage(24)]).split(vert[4]);
    input_field(" City ", &app.form_location_city, app.focus == Focus::FormLocationCity, r4[0], f);
    input_field(" State ", &app.form_location_state, app.focus == Focus::FormLocationState, r4[1], f);
    input_field(" ZIP ", &app.form_location_zip, app.focus == Focus::FormLocationZip, r4[2], f);
    input_field(" Road 1 ", &app.form_location_road1, app.focus == Focus::FormLocationRoad1, r4[3], f);
    input_field(" Road 2 ", &app.form_location_road2, app.focus == Focus::FormLocationRoad2, r4[4], f);

    // Row: Venue + Stall + Lot + Open + Close
    let r5 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(22), Constraint::Percentage(18), Constraint::Percentage(18), Constraint::Percentage(21), Constraint::Percentage(21)]).split(vert[5]);
    input_field(" Venue ", &app.form_location_venue, app.focus == Focus::FormLocationVenue, r5[0], f);
    input_field(" Stall ", &app.form_location_stall, app.focus == Focus::FormLocationStall, r5[1], f);
    input_field(" Lot ", &app.form_location_lot, app.focus == Focus::FormLocationLot, r5[2], f);
    input_field(" Open (HH:MM:SS) ", &app.form_open_time, app.focus == Focus::FormOpenTime, r5[3], f);
    input_field(" Close (HH:MM:SS) ", &app.form_close_time, app.focus == Focus::FormCloseTime, r5[4], f);

    // Row: Status + Description
    let r6 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Length(14), Constraint::Min(20)]).split(vert[6]);
    input_field(" Status [Space] ", if app.form_open_status { "Open" } else { "Closed" }, app.focus == Focus::FormOpenStatus, r6[0], f);
    input_field(" Description ", &app.form_description, app.focus == Focus::FormDescription, r6[1], f);

    // Row: Menu item name + price + description
    let r7 = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(35), Constraint::Percentage(15), Constraint::Percentage(50)]).split(vert[7]);
    input_field(" Menu Item Name (Enter/[A] add) ", &app.form_menu_item_name, app.focus == Focus::FormMenuItemName, r7[0], f);
    input_field(" Price ", &app.form_menu_item_price, app.focus == Focus::FormMenuItemPrice, r7[1], f);
    input_field(" Menu Item Description ", &app.form_menu_item_description, app.focus == Focus::FormMenuItemDescription, r7[2], f);

    // Menu Items list
    let menu_list_style = if let Focus::FormMenuList = app.focus { Style::default().fg(Color::Yellow) } else { Style::default().fg(Color::Gray) };
    let mut mi_list_lines = vec![
        Line::from(Span::styled("  [A]dd  [T]oggle Sold-out  [D]elete  [Up/Down] Select", Style::default().fg(Color::Gray))),
    ];
    if app.form_menu_items.is_empty() {
        mi_list_lines.push(Line::from(Span::styled("  No menu items added yet.", Style::default().fg(Color::DarkGray))));
    } else {
        for (i, item) in app.form_menu_items.iter().enumerate() {
            let selected = app.form_menu_item_selected == Some(i);
            let prefix = if selected { " >" } else { "  " };
            let price_str = match item.price {
                Some(p) => format!(" ${:.2}", p),
                None => "".to_string(),
            };
            let status_str = if item.is_sold_out { " [SOLD OUT]" } else { "" };
            let style = if selected {
                Style::default().fg(Color::Black).bg(Color::Cyan)
            } else {
                Style::default().fg(Color::White)
            };
            mi_list_lines.push(Line::from(Span::styled(
                format!("{}{}{}{}", prefix, item.name, price_str, status_str),
                style,
            )));
        }
    }
    let mi_list_block = Block::default().borders(Borders::ALL).title(" Menu Items ").border_style(menu_list_style);
    let mi_list_para = Paragraph::new(mi_list_lines).block(mi_list_block);
    f.render_widget(mi_list_para, vert[8]);

    let action_para = Paragraph::new("Press [Enter] to submit  -  [Esc] to discard changes")
        .alignment(ratatui::layout::Alignment::Center)
        .fg(Color::DarkGray);
    f.render_widget(action_para, vert[9]);
}
