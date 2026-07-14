use std::{io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod api;
mod ui;

use api::{ApiClient, Restaurant, RestaurantCreate, RestaurantUpdate, RestaurantType, UserRole};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Login,
    Signup,
    ResetPassword,
    Dashboard,
    AddRestaurant,
    EditRestaurant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    // Login
    LoginUsername,
    LoginPassword,
    // Signup
    SignupUsername,
    SignupPassword,
    SignupRole,
    // Reset
    ResetUsername,
    ResetPassword,
    // Dashboard
    SearchInput,
    RestaurantList,
    DetailPane,
    // Form
    FormName,
    FormType,
    FormLocation,
    FormOpenTime,
    FormCloseTime,
    FormOpenStatus,
    FormDescription,
}

pub struct App {
    pub screen: Screen,
    pub focus: Focus,
    pub api_client: ApiClient,
    pub restaurants: Vec<Restaurant>,
    pub selected_restaurant_index: Option<usize>,
    pub status_message: Option<String>,

    // Text Input buffers
    pub input_username: String,
    pub input_password: String,
    pub input_role: UserRole,

    // Search query
    pub search_query: String,

    // Form buffers
    pub form_name: String,
    pub form_type: RestaurantType,
    pub form_location: String,
    pub form_open_time: String,
    pub form_close_time: String,
    pub form_open_status: bool,
    pub form_description: String,
}

impl App {
    pub fn new(base_url: &str) -> Self {
        Self {
            screen: Screen::Login,
            focus: Focus::LoginUsername,
            api_client: ApiClient::new(base_url),
            restaurants: Vec::new(),
            selected_restaurant_index: None,
            status_message: None,
            input_username: String::new(),
            input_password: String::new(),
            input_role: UserRole::Customer,
            search_query: String::new(),
            form_name: String::new(),
            form_type: RestaurantType::FoodStall,
            form_location: String::new(),
            form_open_time: "08:00:00".to_string(),
            form_close_time: "22:00:00".to_string(),
            form_open_status: true,
            form_description: String::new(),
        }
    }

    pub fn set_status(&mut self, msg: &str) {
        self.status_message = Some(msg.to_string());
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    pub async fn fetch_restaurants(&mut self) {
        self.set_status("Loading restaurants...");
        match self.api_client.get_restaurants(Some(&self.search_query)).await {
            Ok(list) => {
                self.restaurants = list;
                if self.restaurants.is_empty() {
                    self.selected_restaurant_index = None;
                } else if self.selected_restaurant_index.is_none() {
                    self.selected_restaurant_index = Some(0);
                } else {
                    let max_idx = self.restaurants.len() - 1;
                    if let Some(ref mut idx) = self.selected_restaurant_index {
                        if *idx > max_idx {
                            *idx = max_idx;
                        }
                    }
                }
                self.clear_status();
            }
            Err(e) => {
                self.set_status(&format!("Fetch error: {}", e));
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let base_url = "http://127.0.0.1:8000";
    let mut app = App::new(base_url);

    let run_res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = run_res {
        println!("App error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    // Global Exit Check
                    if app.screen == Screen::Login && key.code == KeyCode::Esc {
                        return Ok(());
                    }

                    // Global Logout Check
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('l') {
                        app.api_client.logout();
                        app.screen = Screen::Login;
                        app.focus = Focus::LoginUsername;
                        app.input_username.clear();
                        app.input_password.clear();
                        app.set_status("Logged out successfully");
                        continue;
                    }

                    match app.screen {
                        Screen::Login => handle_login_keys(app, key).await,
                        Screen::Signup => handle_signup_keys(app, key).await,
                        Screen::ResetPassword => handle_reset_keys(app, key).await,
                        Screen::Dashboard => handle_dashboard_keys(app, key).await,
                        Screen::AddRestaurant | Screen::EditRestaurant => {
                            handle_form_keys(app, key).await
                        }
                    }
                }
            }
        }
    }
}

async fn handle_login_keys(app: &mut App, key: event::KeyEvent) {
    // Ctrl+S to go to signup
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('s') {
        app.screen = Screen::Signup;
        app.focus = Focus::SignupUsername;
        app.input_username.clear();
        app.input_password.clear();
        app.clear_status();
        return;
    }
    // Ctrl+R to go to reset password
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('r') {
        app.screen = Screen::ResetPassword;
        app.focus = Focus::ResetUsername;
        app.input_username.clear();
        app.input_password.clear();
        app.clear_status();
        return;
    }

    match key.code {
        KeyCode::Tab => {
            app.focus = match app.focus {
                Focus::LoginUsername => Focus::LoginPassword,
                _ => Focus::LoginUsername,
            };
        }
        KeyCode::Char(c) => {
            app.clear_status();
            match app.focus {
                Focus::LoginUsername => app.input_username.push(c),
                Focus::LoginPassword => app.input_password.push(c),
                _ => {}
            }
        }
        KeyCode::Backspace => {
            match app.focus {
                Focus::LoginUsername => {
                    app.input_username.pop();
                }
                Focus::LoginPassword => {
                    app.input_password.pop();
                }
                _ => {}
            }
        }
        KeyCode::Enter => {
            app.set_status("Signing in...");
            match app.api_client.login(&app.input_username, &app.input_password).await {
                Ok(_) => {
                    app.screen = Screen::Dashboard;
                    app.focus = Focus::RestaurantList;
                    app.fetch_restaurants().await;
                }
                Err(e) => {
                    app.set_status(&format!("Login failed: {}", e));
                }
            }
        }
        _ => {}
    }
}

async fn handle_signup_keys(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Tab => {
            app.focus = match app.focus {
                Focus::SignupUsername => Focus::SignupPassword,
                Focus::SignupPassword => Focus::SignupRole,
                _ => Focus::SignupUsername,
            };
        }
        KeyCode::Char(' ') if app.focus == Focus::SignupRole => {
            app.input_role = match app.input_role {
                UserRole::Customer => UserRole::Admin,
                UserRole::Admin => UserRole::Customer,
            };
        }
        KeyCode::Esc => {
            app.screen = Screen::Login;
            app.focus = Focus::LoginUsername;
            app.input_username.clear();
            app.input_password.clear();
            app.clear_status();
        }
        KeyCode::Char(c) => {
            app.clear_status();
            match app.focus {
                Focus::SignupUsername => app.input_username.push(c),
                Focus::SignupPassword => app.input_password.push(c),
                _ => {}
            }
        }
        KeyCode::Backspace => {
            match app.focus {
                Focus::SignupUsername => {
                    app.input_username.pop();
                }
                Focus::SignupPassword => {
                    app.input_password.pop();
                }
                _ => {}
            }
        }
        KeyCode::Enter => {
            app.set_status("Registering...");
            match app.api_client.signup(&app.input_username, &app.input_password, app.input_role.clone()).await {
                Ok(_) => {
                    app.set_status("Registration successful! Please login.");
                    app.screen = Screen::Login;
                    app.focus = Focus::LoginUsername;
                }
                Err(e) => {
                    app.set_status(&format!("Signup failed: {}", e));
                }
            }
        }
        _ => {}
    }
}

async fn handle_reset_keys(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Tab => {
            app.focus = match app.focus {
                Focus::ResetUsername => Focus::ResetPassword,
                _ => Focus::ResetUsername,
            };
        }
        KeyCode::Esc => {
            app.screen = Screen::Login;
            app.focus = Focus::LoginUsername;
            app.input_username.clear();
            app.input_password.clear();
            app.clear_status();
        }
        KeyCode::Char(c) => {
            app.clear_status();
            match app.focus {
                Focus::ResetUsername => app.input_username.push(c),
                Focus::ResetPassword => app.input_password.push(c),
                _ => {}
            }
        }
        KeyCode::Backspace => {
            match app.focus {
                Focus::ResetUsername => {
                    app.input_username.pop();
                }
                Focus::ResetPassword => {
                    app.input_password.pop();
                }
                _ => {}
            }
        }
        KeyCode::Enter => {
            app.set_status("Updating password...");
            match app.api_client.reset_password(&app.input_username, &app.input_password).await {
                Ok(_) => {
                    app.set_status("Password reset successful! Please login.");
                    app.screen = Screen::Login;
                    app.focus = Focus::LoginUsername;
                }
                Err(e) => {
                    app.set_status(&format!("Reset failed: {}", e));
                }
            }
        }
        _ => {}
    }
}

async fn handle_dashboard_keys(app: &mut App, key: event::KeyEvent) {
    if app.focus == Focus::SearchInput {
        match key.code {
            KeyCode::Char(c) => {
                app.search_query.push(c);
                app.fetch_restaurants().await;
                return;
            }
            KeyCode::Backspace => {
                app.search_query.pop();
                app.fetch_restaurants().await;
                return;
            }
            KeyCode::Tab => {
                app.focus = Focus::RestaurantList;
                return;
            }
            KeyCode::Enter | KeyCode::Esc => {
                app.focus = Focus::RestaurantList;
                return;
            }
            _ => {}
        }
    }

    match key.code {
        KeyCode::Tab => {
            app.focus = match app.focus {
                Focus::SearchInput => Focus::RestaurantList,
                Focus::RestaurantList => Focus::DetailPane,
                Focus::DetailPane => Focus::SearchInput,
                _ => Focus::RestaurantList,
            };
        }
        KeyCode::Char('s') => {
            app.focus = Focus::SearchInput;
            app.clear_status();
        }
        KeyCode::Up => {
            if !app.restaurants.is_empty() {
                if let Some(ref mut idx) = app.selected_restaurant_index {
                    if *idx > 0 {
                        *idx -= 1;
                    }
                } else {
                    app.selected_restaurant_index = Some(0);
                }
            }
        }
        KeyCode::Down => {
            if !app.restaurants.is_empty() {
                if let Some(ref mut idx) = app.selected_restaurant_index {
                    if *idx < app.restaurants.len() - 1 {
                        *idx += 1;
                    }
                } else {
                    app.selected_restaurant_index = Some(0);
                }
            }
        }
        // Admin commands
        KeyCode::Char('a') if app.api_client.role == Some(UserRole::Admin) => {
            app.screen = Screen::AddRestaurant;
            app.focus = Focus::FormName;
            app.form_name.clear();
            app.form_type = RestaurantType::FoodStall;
            app.form_location.clear();
            app.form_open_time = "08:00:00".to_string();
            app.form_close_time = "22:00:00".to_string();
            app.form_open_status = true;
            app.form_description.clear();
            app.clear_status();
        }
        KeyCode::Char('e') if app.api_client.role == Some(UserRole::Admin) => {
            if let Some(idx) = app.selected_restaurant_index {
                let restaurant_info = app.restaurants.get(idx).cloned();
                if let Some(r) = restaurant_info {
                    app.screen = Screen::EditRestaurant;
                    app.focus = Focus::FormName;
                    app.form_name = r.name;
                    app.form_type = r.restaurant_type;
                    app.form_location = r.location;
                    app.form_open_time = r.open_time;
                    app.form_close_time = r.close_time;
                    app.form_open_status = r.open_status;
                    app.form_description = r.description.unwrap_or_default();
                    app.clear_status();
                }
            } else {
                app.set_status("Please select a restaurant to edit");
            }
        }
        KeyCode::Char('t') if app.api_client.role == Some(UserRole::Admin) => {
            if let Some(idx) = app.selected_restaurant_index {
                let status_info = app.restaurants.get(idx).map(|r| (r.id, r.open_status));
                if let Some((id, open_status)) = status_info {
                    let new_status = !open_status;
                    app.set_status("Toggling status...");
                    match app.api_client.update_restaurant_status(id, new_status).await {
                        Ok(_) => {
                            app.fetch_restaurants().await;
                            app.set_status("Status toggled successfully");
                        }
                        Err(e) => {
                            app.set_status(&format!("Toggle failed: {}", e));
                        }
                    }
                }
            }
        }
        KeyCode::Char('d') if app.api_client.role == Some(UserRole::Admin) => {
            if let Some(idx) = app.selected_restaurant_index {
                let id_to_delete = app.restaurants.get(idx).map(|r| r.id);
                if let Some(id) = id_to_delete {
                    app.set_status("Deleting entry...");
                    match app.api_client.delete_restaurant(id).await {
                        Ok(_) => {
                            app.selected_restaurant_index = None;
                            app.fetch_restaurants().await;
                            app.set_status("Restaurant deleted successfully");
                        }
                        Err(e) => {
                            app.set_status(&format!("Delete failed: {}", e));
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

async fn handle_form_keys(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Tab => {
            app.focus = match app.focus {
                Focus::FormName => Focus::FormType,
                Focus::FormType => Focus::FormLocation,
                Focus::FormLocation => Focus::FormOpenTime,
                Focus::FormOpenTime => Focus::FormCloseTime,
                Focus::FormCloseTime => Focus::FormOpenStatus,
                Focus::FormOpenStatus => Focus::FormDescription,
                _ => Focus::FormName,
            };
        }
        KeyCode::Char(' ') => {
            if app.focus == Focus::FormType {
                app.form_type = match app.form_type {
                    RestaurantType::FoodStall => RestaurantType::FoodTruck,
                    RestaurantType::FoodTruck => RestaurantType::BrickAndMortar,
                    RestaurantType::BrickAndMortar => RestaurantType::FoodStall,
                };
            } else if app.focus == Focus::FormOpenStatus {
                app.form_open_status = !app.form_open_status;
            } else {
                match app.focus {
                    Focus::FormName => app.form_name.push(' '),
                    Focus::FormLocation => app.form_location.push(' '),
                    Focus::FormDescription => app.form_description.push(' '),
                    _ => {}
                }
            }
        }
        KeyCode::Char(c) => {
            match app.focus {
                Focus::FormName => app.form_name.push(c),
                Focus::FormLocation => app.form_location.push(c),
                Focus::FormOpenTime => app.form_open_time.push(c),
                Focus::FormCloseTime => app.form_close_time.push(c),
                Focus::FormDescription => app.form_description.push(c),
                _ => {}
            }
        }
        KeyCode::Backspace => {
            match app.focus {
                Focus::FormName => { app.form_name.pop(); }
                Focus::FormLocation => { app.form_location.pop(); }
                Focus::FormOpenTime => { app.form_open_time.pop(); }
                Focus::FormCloseTime => { app.form_close_time.pop(); }
                Focus::FormDescription => { app.form_description.pop(); }
                _ => {}
            }
        }
        KeyCode::Esc => {
            app.screen = Screen::Dashboard;
            app.focus = Focus::RestaurantList;
            app.clear_status();
        }
        KeyCode::Enter => {
            if app.form_open_time.len() != 8 || app.form_close_time.len() != 8 {
                app.set_status("Validation error: Time format must be HH:MM:SS");
                return;
            }

            app.set_status("Saving restaurant entry...");
            let desc = if app.form_description.trim().is_empty() {
                None
            } else {
                Some(app.form_description.clone())
            };

            match app.screen {
                Screen::AddRestaurant => {
                    let r = RestaurantCreate {
                        name: app.form_name.clone(),
                        restaurant_type: app.form_type.clone(),
                        location: app.form_location.clone(),
                        open_time: app.form_open_time.clone(),
                        close_time: app.form_close_time.clone(),
                        open_status: app.form_open_status,
                        description: desc,
                    };
                    match app.api_client.create_restaurant(&r).await {
                        Ok(_) => {
                            app.screen = Screen::Dashboard;
                            app.focus = Focus::RestaurantList;
                            app.fetch_restaurants().await;
                            app.set_status("Restaurant added successfully");
                        }
                        Err(e) => {
                            app.set_status(&format!("Create failed: {}", e));
                        }
                    }
                }
                Screen::EditRestaurant => {
                    if let Some(idx) = app.selected_restaurant_index {
                        let id_to_update = app.restaurants.get(idx).map(|r| r.id);
                        if let Some(id) = id_to_update {
                            let r_update = RestaurantUpdate {
                                name: Some(app.form_name.clone()),
                                restaurant_type: Some(app.form_type.clone()),
                                location: Some(app.form_location.clone()),
                                open_time: Some(app.form_open_time.clone()),
                                close_time: Some(app.form_close_time.clone()),
                                open_status: Some(app.form_open_status),
                                description: desc,
                            };
                            match app.api_client.update_restaurant(id, &r_update).await {
                                Ok(_) => {
                                    app.screen = Screen::Dashboard;
                                    app.focus = Focus::RestaurantList;
                                    app.fetch_restaurants().await;
                                    app.set_status("Restaurant updated successfully");
                                }
                                Err(e) => {
                                    app.set_status(&format!("Update failed: {}", e));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_initialization() {
        let app = App::new("http://localhost:8000");
        assert_eq!(app.screen, Screen::Login);
        assert_eq!(app.focus, Focus::LoginUsername);
        assert!(app.input_username.is_empty());
        assert!(app.input_password.is_empty());
    }

    #[tokio::test]
    async fn test_login_input_buffering() {
        let mut app = App::new("http://localhost:8000");
        
        handle_login_keys(&mut app, event::KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)).await;
        handle_login_keys(&mut app, event::KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE)).await;
        handle_login_keys(&mut app, event::KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE)).await;
        assert_eq!(app.input_username, "abc");

        handle_login_keys(&mut app, event::KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)).await;
        assert_eq!(app.input_username, "ab");

        handle_login_keys(&mut app, event::KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)).await;
        assert_eq!(app.focus, Focus::LoginPassword);

        handle_login_keys(&mut app, event::KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE)).await;
        handle_login_keys(&mut app, event::KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)).await;
        handle_login_keys(&mut app, event::KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE)).await;
        assert_eq!(app.input_password, "pas");
    }

    #[tokio::test]
    async fn test_screen_transitions() {
        let mut app = App::new("http://localhost:8000");
        
        // Press 'Ctrl+S' to go to Signup
        handle_login_keys(&mut app, event::KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL)).await;
        assert_eq!(app.screen, Screen::Signup);
        assert_eq!(app.focus, Focus::SignupUsername);

        // Cycle focus in signup
        handle_signup_keys(&mut app, event::KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)).await;
        assert_eq!(app.focus, Focus::SignupPassword);
        handle_signup_keys(&mut app, event::KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)).await;
        assert_eq!(app.focus, Focus::SignupRole);

        // Toggle role with Space
        assert_eq!(app.input_role, UserRole::Customer);
        handle_signup_keys(&mut app, event::KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)).await;
        assert_eq!(app.input_role, UserRole::Admin);
        handle_signup_keys(&mut app, event::KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)).await;
        assert_eq!(app.input_role, UserRole::Customer);

        // Back to login with Esc
        handle_signup_keys(&mut app, event::KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)).await;
        assert_eq!(app.screen, Screen::Login);
    }

    #[tokio::test]
    async fn test_form_time_validation() {
        let mut app = App::new("http://localhost:8000");
        app.screen = Screen::AddRestaurant;
        app.api_client.set_token("dummy".to_string(), "admin".to_string(), UserRole::Admin);
        
        app.form_open_time = "08:00".to_string();
        handle_form_keys(&mut app, event::KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)).await;
        assert_eq!(app.screen, Screen::AddRestaurant);
        assert!(app.status_message.as_ref().unwrap().contains("Validation error"));
    }
}
