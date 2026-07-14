use std::{io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod api;
mod ui;

use api::{ApiClient, FavoriteResponse, LocationCreatePayload, LocationUpdatePayload, Restaurant, RestaurantCreate, RestaurantUpdate, RestaurantType, UserRole};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Login,
    Signup,
    ResetPassword,
    Dashboard,
    AddRestaurant,
    EditRestaurant,
    MyRestaurants,
    FavoritesView,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    LoginUsername,
    LoginPassword,
    SignupUsername,
    SignupPassword,
    SignupRole,
    ResetUsername,
    ResetPassword,
    SearchInput,
    RestaurantList,
    DetailPane,
    FormName,
    FormType,
    FormCuisineType,
    FormLocation,
    FormLocationDesc,
    FormLocationLat,
    FormLocationLng,
    FormLocationAddress,
    FormLocationCity,
    FormLocationState,
    FormLocationZip,
    FormLocationRoad1,
    FormLocationRoad2,
    FormLocationVenue,
    FormLocationStall,
    FormLocationLot,
    FormOpenTime,
    FormCloseTime,
    FormOpenStatus,
    FormDescription,
    FormMenuItems,
    MyRestaurantsList,
    MyRestaurantsDetail,
    FavoritesList,
    FavoritesDetail,
}

pub struct App {
    pub screen: Screen,
    pub focus: Focus,
    pub api_client: ApiClient,
    pub restaurants: Vec<Restaurant>,
    pub my_restaurants: Vec<Restaurant>,
    pub favorites: Vec<FavoriteResponse>,
    pub selected_restaurant_index: Option<usize>,
    pub selected_my_restaurant_index: Option<usize>,
    pub selected_favorite_index: Option<usize>,
    pub status_message: Option<String>,
    pub editing_restaurant_id: Option<i32>,

    pub input_username: String,
    pub input_password: String,
    pub input_role: UserRole,

    pub search_query: String,

    pub form_name: String,
    pub form_type: RestaurantType,
    pub form_cuisine_type: String,
    pub form_location: String,
    pub form_location_desc: String,
    pub form_location_lat: String,
    pub form_location_lng: String,
    pub form_location_address: String,
    pub form_location_city: String,
    pub form_location_state: String,
    pub form_location_zip: String,
    pub form_location_road1: String,
    pub form_location_road2: String,
    pub form_location_venue: String,
    pub form_location_stall: String,
    pub form_location_lot: String,
    pub form_open_time: String,
    pub form_close_time: String,
    pub form_open_status: bool,
    pub form_description: String,
    pub form_menu_items: String,
}

impl App {
    pub fn new(base_url: &str) -> Self {
        Self {
            screen: Screen::Login,
            focus: Focus::LoginUsername,
            api_client: ApiClient::new(base_url),
            restaurants: Vec::new(),
            my_restaurants: Vec::new(),
            favorites: Vec::new(),
            selected_restaurant_index: None,
            selected_my_restaurant_index: None,
            selected_favorite_index: None,
            status_message: None,
            editing_restaurant_id: None,
            input_username: String::new(),
            input_password: String::new(),
            input_role: UserRole::Customer,
            search_query: String::new(),
            form_name: String::new(),
            form_type: RestaurantType::FoodStall,
            form_cuisine_type: String::new(),
            form_location: String::new(),
            form_location_desc: String::new(),
            form_location_lat: String::new(),
            form_location_lng: String::new(),
            form_location_address: String::new(),
            form_location_city: String::new(),
            form_location_state: String::new(),
            form_location_zip: String::new(),
            form_location_road1: String::new(),
            form_location_road2: String::new(),
            form_location_venue: String::new(),
            form_location_stall: String::new(),
            form_location_lot: String::new(),
            form_open_time: "08:00:00".to_string(),
            form_close_time: "22:00:00".to_string(),
            form_open_status: true,
            form_description: String::new(),
            form_menu_items: String::new(),
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

    pub async fn fetch_my_restaurants(&mut self) {
        self.set_status("Loading your restaurants...");
        match self.api_client.get_my_restaurants().await {
            Ok(list) => {
                self.my_restaurants = list;
                if self.my_restaurants.is_empty() {
                    self.selected_my_restaurant_index = None;
                } else if self.selected_my_restaurant_index.is_none() {
                    self.selected_my_restaurant_index = Some(0);
                } else {
                    let max_idx = self.my_restaurants.len() - 1;
                    if let Some(ref mut idx) = self.selected_my_restaurant_index {
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

    pub async fn fetch_favorites(&mut self) {
        self.set_status("Loading favorites...");
        match self.api_client.get_favorites().await {
            Ok(list) => {
                self.favorites = list;
                if self.favorites.is_empty() {
                    self.selected_favorite_index = None;
                } else if self.selected_favorite_index.is_none() {
                    self.selected_favorite_index = Some(0);
                } else {
                    let max_idx = self.favorites.len() - 1;
                    if let Some(ref mut idx) = self.selected_favorite_index {
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
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let base_url = "http://127.0.0.1:8000";
    let mut app = App::new(base_url);

    let run_res = run_app(&mut terminal, &mut app).await;

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
                    if app.screen == Screen::Login && key.code == KeyCode::Esc {
                        return Ok(());
                    }

                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('l') {
                        app.api_client.logout();
                        app.screen = Screen::Login;
                        app.focus = Focus::LoginUsername;
                        app.input_username.clear();
                        app.input_password.clear();
                        app.my_restaurants.clear();
                        app.favorites.clear();
                        app.selected_my_restaurant_index = None;
                        app.selected_favorite_index = None;
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
                        Screen::MyRestaurants => handle_my_restaurants_keys(app, key).await,
                        Screen::FavoritesView => handle_favorites_keys(app, key).await,
                    }
                }
            }
        }
    }
}

async fn handle_login_keys(app: &mut App, key: event::KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('s') {
        app.screen = Screen::Signup;
        app.focus = Focus::SignupUsername;
        app.input_username.clear();
        app.input_password.clear();
        app.clear_status();
        return;
    }
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
                    if app.api_client.role == Some(UserRole::Customer) {
                        app.screen = Screen::MyRestaurants;
                        app.focus = Focus::MyRestaurantsList;
                        app.fetch_my_restaurants().await;
                    } else {
                        app.screen = Screen::Dashboard;
                        app.focus = Focus::RestaurantList;
                        app.fetch_restaurants().await;
                    }
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
                UserRole::Admin => UserRole::Consumer,
                UserRole::Consumer => UserRole::Customer,
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
        KeyCode::Char('m') if app.api_client.role == Some(UserRole::Customer) => {
            app.screen = Screen::MyRestaurants;
            app.focus = Focus::MyRestaurantsList;
            app.fetch_my_restaurants().await;
            app.clear_status();
        }
        KeyCode::Char('f') if app.api_client.role == Some(UserRole::Consumer) => {
            if let Some(idx) = app.selected_restaurant_index {
                let restaurant_id = app.restaurants.get(idx).map(|r| r.id);
                if let Some(id) = restaurant_id {
                    app.set_status("Adding favorite...");
                    match app.api_client.add_favorite(id).await {
                        Ok(_) => {
                            app.set_status("Favorite added successfully");
                        }
                        Err(e) => {
                            app.set_status(&format!("Failed to add favorite: {}", e));
                        }
                    }
                }
            } else {
                app.set_status("Please select a restaurant to favorite");
            }
        }
        KeyCode::Char('v') if app.api_client.role == Some(UserRole::Consumer) => {
            app.screen = Screen::FavoritesView;
            app.focus = Focus::FavoritesList;
            app.fetch_favorites().await;
            app.clear_status();
        }
        KeyCode::Char('a') if app.api_client.role == Some(UserRole::Admin) => {
            app.screen = Screen::AddRestaurant;
            app.focus = Focus::FormName;
            app.editing_restaurant_id = None;
            clear_form(app);
            app.clear_status();
        }
        KeyCode::Char('e') if app.api_client.role == Some(UserRole::Admin) => {
            if let Some(idx) = app.selected_restaurant_index {
                let restaurant_info = app.restaurants.get(idx).cloned();
                if let Some(r) = restaurant_info {
                    app.screen = Screen::EditRestaurant;
                    app.focus = Focus::FormName;
                    app.editing_restaurant_id = Some(r.id);
                    populate_form_from_restaurant(app, &r);
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
        KeyCode::Char('r') if app.api_client.role == Some(UserRole::Admin) => {
            if let Some(idx) = app.selected_restaurant_index {
                let restaurant_info = app.restaurants.get(idx).cloned();
                if let Some(r) = restaurant_info {
                    let new_approved = !r.is_approved;
                    app.set_status("Updating approval status...");
                    match app.api_client.approve_restaurant(r.id, new_approved).await {
                        Ok(_) => {
                            app.fetch_restaurants().await;
                            app.set_status("Approval status updated");
                        }
                        Err(e) => {
                            app.set_status(&format!("Approval update failed: {}", e));
                        }
                    }
                }
            } else {
                app.set_status("Please select a restaurant to approve");
            }
        }
        KeyCode::Char('l') if app.api_client.role == Some(UserRole::Admin) => {
            if let Some(idx) = app.selected_restaurant_index {
                let restaurant_info = app.restaurants.get(idx).cloned();
                if let Some(r) = restaurant_info {
                    if r.location_change_pending {
                        app.set_status("Approving location change...");
                        match app.api_client.approve_location_change(r.id, true).await {
                            Ok(_) => {
                                app.fetch_restaurants().await;
                                app.set_status("Location change approved");
                            }
                            Err(e) => {
                                app.set_status(&format!("Location approval failed: {}", e));
                            }
                        }
                    } else {
                        app.set_status("No pending location change for this restaurant");
                    }
                }
            }
        }
        KeyCode::Char('L') if app.api_client.role == Some(UserRole::Admin) => {
            if let Some(idx) = app.selected_restaurant_index {
                let restaurant_info = app.restaurants.get(idx).cloned();
                if let Some(r) = restaurant_info {
                    if r.location_change_pending {
                        app.set_status("Rejecting location change...");
                        match app.api_client.approve_location_change(r.id, false).await {
                            Ok(_) => {
                                app.fetch_restaurants().await;
                                app.set_status("Location change rejected");
                            }
                            Err(e) => {
                                app.set_status(&format!("Location rejection failed: {}", e));
                            }
                        }
                    } else {
                        app.set_status("No pending location change for this restaurant");
                    }
                }
            }
        }
        _ => {}
    }
}

async fn handle_my_restaurants_keys(app: &mut App, key: event::KeyEvent) {
    if app.focus == Focus::MyRestaurantsList {
        match key.code {
            KeyCode::Up => {
                if !app.my_restaurants.is_empty() {
                    if let Some(ref mut idx) = app.selected_my_restaurant_index {
                        if *idx > 0 {
                            *idx -= 1;
                        }
                    } else {
                        app.selected_my_restaurant_index = Some(0);
                    }
                }
                return;
            }
            KeyCode::Down => {
                if !app.my_restaurants.is_empty() {
                    if let Some(ref mut idx) = app.selected_my_restaurant_index {
                        if *idx < app.my_restaurants.len() - 1 {
                            *idx += 1;
                        }
                    } else {
                        app.selected_my_restaurant_index = Some(0);
                    }
                }
                return;
            }
            _ => {}
        }
    }

    match key.code {
        KeyCode::Tab => {
            app.focus = match app.focus {
                Focus::MyRestaurantsList => Focus::MyRestaurantsDetail,
                Focus::MyRestaurantsDetail => Focus::MyRestaurantsList,
                _ => Focus::MyRestaurantsList,
            };
        }
        KeyCode::Esc => {
            app.screen = Screen::Dashboard;
            app.focus = Focus::RestaurantList;
            app.clear_status();
        }
        KeyCode::Char('a') => {
            app.screen = Screen::AddRestaurant;
            app.focus = Focus::FormName;
            app.editing_restaurant_id = None;
            clear_form(app);
            app.clear_status();
        }
        KeyCode::Char('e') => {
            if let Some(idx) = app.selected_my_restaurant_index {
                let restaurant_info = app.my_restaurants.get(idx).cloned();
                if let Some(r) = restaurant_info {
                    app.screen = Screen::EditRestaurant;
                    app.focus = Focus::FormName;
                    app.editing_restaurant_id = Some(r.id);
                    populate_form_from_restaurant(app, &r);
                    app.clear_status();
                }
            } else {
                app.set_status("Please select a restaurant to edit");
            }
        }
        KeyCode::Char('t') => {
            if let Some(idx) = app.selected_my_restaurant_index {
                let status_info = app.my_restaurants.get(idx).map(|r| (r.id, r.open_status));
                if let Some((id, open_status)) = status_info {
                    let new_status = !open_status;
                    app.set_status("Toggling status...");
                    match app.api_client.update_restaurant_status(id, new_status).await {
                        Ok(_) => {
                            app.fetch_my_restaurants().await;
                            app.set_status("Status toggled successfully");
                        }
                        Err(e) => {
                            app.set_status(&format!("Toggle failed: {}", e));
                        }
                    }
                }
            }
        }
        KeyCode::Char('d') => {
            if let Some(idx) = app.selected_my_restaurant_index {
                let id_to_delete = app.my_restaurants.get(idx).map(|r| r.id);
                if let Some(id) = id_to_delete {
                    app.set_status("Deleting entry...");
                    match app.api_client.delete_restaurant(id).await {
                        Ok(_) => {
                            app.selected_my_restaurant_index = None;
                            app.fetch_my_restaurants().await;
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

async fn handle_favorites_keys(app: &mut App, key: event::KeyEvent) {
    if app.focus == Focus::FavoritesList {
        match key.code {
            KeyCode::Up => {
                if !app.favorites.is_empty() {
                    if let Some(ref mut idx) = app.selected_favorite_index {
                        if *idx > 0 {
                            *idx -= 1;
                        }
                    } else {
                        app.selected_favorite_index = Some(0);
                    }
                }
                return;
            }
            KeyCode::Down => {
                if !app.favorites.is_empty() {
                    if let Some(ref mut idx) = app.selected_favorite_index {
                        if *idx < app.favorites.len() - 1 {
                            *idx += 1;
                        }
                    } else {
                        app.selected_favorite_index = Some(0);
                    }
                }
                return;
            }
            _ => {}
        }
    }

    match key.code {
        KeyCode::Tab => {
            app.focus = match app.focus {
                Focus::FavoritesList => Focus::FavoritesDetail,
                Focus::FavoritesDetail => Focus::FavoritesList,
                _ => Focus::FavoritesList,
            };
        }
        KeyCode::Esc => {
            app.screen = Screen::Dashboard;
            app.focus = Focus::RestaurantList;
            app.clear_status();
        }
        KeyCode::Char('d') => {
            if let Some(idx) = app.selected_favorite_index {
                let favorite_id = app.favorites.get(idx).map(|f| f.id);
                if let Some(id) = favorite_id {
                    app.set_status("Removing favorite...");
                    match app.api_client.remove_favorite(id).await {
                        Ok(_) => {
                            app.selected_favorite_index = None;
                            app.fetch_favorites().await;
                            app.set_status("Favorite removed successfully");
                        }
                        Err(e) => {
                            app.set_status(&format!("Remove failed: {}", e));
                        }
                    }
                }
            } else {
                app.set_status("Please select a favorite to remove");
            }
        }
        _ => {}
    }
}

fn clear_form(app: &mut App) {
    app.form_name.clear();
    app.form_type = RestaurantType::FoodStall;
    app.form_cuisine_type.clear();
    app.form_location.clear();
    app.form_location_desc.clear();
    app.form_location_lat.clear();
    app.form_location_lng.clear();
    app.form_location_address.clear();
    app.form_location_city.clear();
    app.form_location_state.clear();
    app.form_location_zip.clear();
    app.form_location_road1.clear();
    app.form_location_road2.clear();
    app.form_location_venue.clear();
    app.form_location_stall.clear();
    app.form_location_lot.clear();
    app.form_open_time = "08:00:00".to_string();
    app.form_close_time = "22:00:00".to_string();
    app.form_open_status = true;
    app.form_description.clear();
    app.form_menu_items.clear();
}

fn populate_form_from_restaurant(app: &mut App, r: &Restaurant) {
    app.form_name = r.name.clone();
    app.form_type = r.restaurant_type.clone();
    app.form_cuisine_type = r.cuisine_type.clone();
    app.form_location = r.location.formatted.clone();
    app.form_location_desc = r.location.description.clone().unwrap_or_default();
    app.form_location_lat = r.location.lat.map(|v| v.to_string()).unwrap_or_default();
    app.form_location_lng = r.location.lng.map(|v| v.to_string()).unwrap_or_default();
    app.form_location_address = r.location.address.clone().unwrap_or_default();
    app.form_location_city = r.location.city.clone().unwrap_or_default();
    app.form_location_state = r.location.state.clone().unwrap_or_default();
    app.form_location_zip = r.location.zip_code.clone().unwrap_or_default();
    app.form_location_road1 = r.location.road_1.clone().unwrap_or_default();
    app.form_location_road2 = r.location.road_2.clone().unwrap_or_default();
    app.form_location_venue = r.location.venue_name.clone().unwrap_or_default();
    app.form_location_stall = r.location.stall_number.clone().unwrap_or_default();
    app.form_location_lot = r.location.lot_name.clone().unwrap_or_default();
    app.form_open_time = r.open_time.clone();
    app.form_close_time = r.close_time.clone();
    app.form_open_status = r.open_status;
    app.form_description = r.description.clone().unwrap_or_default();
    app.form_menu_items = r.menu_items.clone().unwrap_or_default();
}

async fn handle_form_keys(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Tab => {
            app.focus = match app.focus {
                Focus::FormName => Focus::FormType,
                Focus::FormType => Focus::FormCuisineType,
                Focus::FormCuisineType => Focus::FormLocation,
                Focus::FormLocation => Focus::FormLocationDesc,
                Focus::FormLocationDesc => Focus::FormLocationLat,
                Focus::FormLocationLat => Focus::FormLocationLng,
                Focus::FormLocationLng => Focus::FormLocationAddress,
                Focus::FormLocationAddress => Focus::FormLocationCity,
                Focus::FormLocationCity => Focus::FormLocationState,
                Focus::FormLocationState => Focus::FormLocationZip,
                Focus::FormLocationZip => Focus::FormLocationRoad1,
                Focus::FormLocationRoad1 => Focus::FormLocationRoad2,
                Focus::FormLocationRoad2 => Focus::FormLocationVenue,
                Focus::FormLocationVenue => Focus::FormLocationStall,
                Focus::FormLocationStall => Focus::FormLocationLot,
                Focus::FormLocationLot => Focus::FormOpenTime,
                Focus::FormOpenTime => Focus::FormCloseTime,
                Focus::FormCloseTime => Focus::FormOpenStatus,
                Focus::FormOpenStatus => Focus::FormDescription,
                Focus::FormDescription => Focus::FormMenuItems,
                _ => Focus::FormName,
            };
        }
        KeyCode::Char(' ') => {
            if app.focus == Focus::FormType {
                app.form_type = match app.form_type {
                    RestaurantType::FoodStall => RestaurantType::FoodTruck,
                    RestaurantType::FoodTruck => RestaurantType::FoodCart,
                    RestaurantType::FoodCart => RestaurantType::BrickAndMortar,
                    RestaurantType::BrickAndMortar => RestaurantType::FoodStall,
                };
            } else if app.focus == Focus::FormOpenStatus {
                app.form_open_status = !app.form_open_status;
            } else {
                match app.focus {
                    Focus::FormName => app.form_name.push(' '),
                    Focus::FormCuisineType => app.form_cuisine_type.push(' '),
                    Focus::FormLocation | Focus::FormLocationDesc | Focus::FormLocationAddress |
                    Focus::FormLocationCity | Focus::FormLocationState | Focus::FormLocationZip |
                    Focus::FormLocationRoad1 | Focus::FormLocationRoad2 | Focus::FormLocationVenue |
                    Focus::FormLocationStall | Focus::FormLocationLot |
                    Focus::FormLocationLat | Focus::FormLocationLng => {
                        match app.focus {
                            Focus::FormLocation => app.form_location.push(' '),
                            Focus::FormLocationDesc => app.form_location_desc.push(' '),
                            Focus::FormLocationLat => app.form_location_lat.push(' '),
                            Focus::FormLocationLng => app.form_location_lng.push(' '),
                            Focus::FormLocationAddress => app.form_location_address.push(' '),
                            Focus::FormLocationCity => app.form_location_city.push(' '),
                            Focus::FormLocationState => app.form_location_state.push(' '),
                            Focus::FormLocationZip => app.form_location_zip.push(' '),
                            Focus::FormLocationRoad1 => app.form_location_road1.push(' '),
                            Focus::FormLocationRoad2 => app.form_location_road2.push(' '),
                            Focus::FormLocationVenue => app.form_location_venue.push(' '),
                            Focus::FormLocationStall => app.form_location_stall.push(' '),
                            Focus::FormLocationLot => app.form_location_lot.push(' '),
                            _ => {}
                        }
                    }
                    Focus::FormDescription => app.form_description.push(' '),
                    Focus::FormMenuItems => app.form_menu_items.push(' '),
                    _ => {}
                }
            }
        }
        KeyCode::Char(c) => {
            match app.focus {
                Focus::FormName => app.form_name.push(c),
                Focus::FormCuisineType => app.form_cuisine_type.push(c),
                Focus::FormLocation => app.form_location.push(c),
                Focus::FormLocationDesc => app.form_location_desc.push(c),
                Focus::FormLocationLat => app.form_location_lat.push(c),
                Focus::FormLocationLng => app.form_location_lng.push(c),
                Focus::FormLocationAddress => app.form_location_address.push(c),
                Focus::FormLocationCity => app.form_location_city.push(c),
                Focus::FormLocationState => app.form_location_state.push(c),
                Focus::FormLocationZip => app.form_location_zip.push(c),
                Focus::FormLocationRoad1 => app.form_location_road1.push(c),
                Focus::FormLocationRoad2 => app.form_location_road2.push(c),
                Focus::FormLocationVenue => app.form_location_venue.push(c),
                Focus::FormLocationStall => app.form_location_stall.push(c),
                Focus::FormLocationLot => app.form_location_lot.push(c),
                Focus::FormOpenTime => app.form_open_time.push(c),
                Focus::FormCloseTime => app.form_close_time.push(c),
                Focus::FormDescription => app.form_description.push(c),
                Focus::FormMenuItems => app.form_menu_items.push(c),
                _ => {}
            }
        }
        KeyCode::Backspace => {
            match app.focus {
                Focus::FormName => { app.form_name.pop(); }
                Focus::FormCuisineType => { app.form_cuisine_type.pop(); }
                Focus::FormLocation => { app.form_location.pop(); }
                Focus::FormLocationDesc => { app.form_location_desc.pop(); }
                Focus::FormLocationLat => { app.form_location_lat.pop(); }
                Focus::FormLocationLng => { app.form_location_lng.pop(); }
                Focus::FormLocationAddress => { app.form_location_address.pop(); }
                Focus::FormLocationCity => { app.form_location_city.pop(); }
                Focus::FormLocationState => { app.form_location_state.pop(); }
                Focus::FormLocationZip => { app.form_location_zip.pop(); }
                Focus::FormLocationRoad1 => { app.form_location_road1.pop(); }
                Focus::FormLocationRoad2 => { app.form_location_road2.pop(); }
                Focus::FormLocationVenue => { app.form_location_venue.pop(); }
                Focus::FormLocationStall => { app.form_location_stall.pop(); }
                Focus::FormLocationLot => { app.form_location_lot.pop(); }
                Focus::FormOpenTime => { app.form_open_time.pop(); }
                Focus::FormCloseTime => { app.form_close_time.pop(); }
                Focus::FormDescription => { app.form_description.pop(); }
                Focus::FormMenuItems => { app.form_menu_items.pop(); }
                _ => {}
            }
        }
        KeyCode::Esc => {
            let target_screen = if app.api_client.role == Some(UserRole::Customer) {
                Screen::MyRestaurants
            } else {
                Screen::Dashboard
            };
            app.screen = target_screen;
            app.focus = if target_screen == Screen::MyRestaurants {
                Focus::MyRestaurantsList
            } else {
                Focus::RestaurantList
            };
            app.editing_restaurant_id = None;
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
            let menu = if app.form_menu_items.trim().is_empty() {
                None
            } else {
                Some(app.form_menu_items.clone())
            };

            let cuisine = app.form_cuisine_type.clone();
            let loc_type = match app.form_type {
                RestaurantType::BrickAndMortar => "street_address",
                RestaurantType::FoodCart => "food_court",
                RestaurantType::FoodTruck => "gps",
                RestaurantType::FoodStall => "intersection",
            };
            let parse_opt_f64 = |s: &str| -> Option<f64> {
                let t = s.trim();
                if t.is_empty() { None } else { t.parse::<f64>().ok() }
            };
            let opt_str = |s: &str| -> Option<String> {
                let t = s.trim().to_string();
                if t.is_empty() { None } else { Some(t) }
            };
            let loc_payload = LocationCreatePayload {
                location_type: loc_type.to_string(),
                description: opt_str(&app.form_location),
                lat: parse_opt_f64(&app.form_location_lat),
                lng: parse_opt_f64(&app.form_location_lng),
                address: opt_str(&app.form_location_address),
                city: opt_str(&app.form_location_city),
                state: opt_str(&app.form_location_state),
                zip_code: opt_str(&app.form_location_zip),
                road_1: opt_str(&app.form_location_road1),
                road_2: opt_str(&app.form_location_road2),
                venue_name: opt_str(&app.form_location_venue),
                stall_number: opt_str(&app.form_location_stall),
                lot_name: opt_str(&app.form_location_lot),
            };

            match app.screen {
                Screen::AddRestaurant => {
                    let r = RestaurantCreate {
                        name: app.form_name.clone(),
                        restaurant_type: app.form_type.clone(),
                        cuisine_type: cuisine,
                        location: loc_payload,
                        open_time: app.form_open_time.clone(),
                        close_time: app.form_close_time.clone(),
                        open_status: app.form_open_status,
                        description: desc,
                        menu_items: menu,
                    };

                    let is_customer = app.api_client.role == Some(UserRole::Customer);
                    let result = if is_customer {
                        app.api_client.submit_restaurant(&r).await
                    } else {
                        app.api_client.create_restaurant(&r).await
                    };

                    match result {
                        Ok(_) => {
                            app.editing_restaurant_id = None;
                            if is_customer {
                                app.screen = Screen::MyRestaurants;
                                app.focus = Focus::MyRestaurantsList;
                                app.fetch_my_restaurants().await;
                                app.set_status("Restaurant submitted for approval");
                            } else {
                                app.screen = Screen::Dashboard;
                                app.focus = Focus::RestaurantList;
                                app.fetch_restaurants().await;
                                app.set_status("Restaurant added successfully");
                            }
                        }
                        Err(e) => {
                            app.set_status(&format!("Create failed: {}", e));
                        }
                    }
                }
                Screen::EditRestaurant => {
                    if let Some(id) = app.editing_restaurant_id {
                        let r_update = RestaurantUpdate {
                            name: Some(app.form_name.clone()),
                            restaurant_type: Some(app.form_type.clone()),
                            cuisine_type: Some(cuisine),
                            location: Some(LocationUpdatePayload {
                                description: opt_str(&app.form_location),
                                lat: parse_opt_f64(&app.form_location_lat),
                                lng: parse_opt_f64(&app.form_location_lng),
                                address: opt_str(&app.form_location_address),
                                city: opt_str(&app.form_location_city),
                                state: opt_str(&app.form_location_state),
                                zip_code: opt_str(&app.form_location_zip),
                                road_1: opt_str(&app.form_location_road1),
                                road_2: opt_str(&app.form_location_road2),
                                venue_name: opt_str(&app.form_location_venue),
                                stall_number: opt_str(&app.form_location_stall),
                                lot_name: opt_str(&app.form_location_lot),
                            }),
                            open_time: Some(app.form_open_time.clone()),
                            close_time: Some(app.form_close_time.clone()),
                            open_status: Some(app.form_open_status),
                            description: desc,
                            menu_items: menu,
                        };
                        match app.api_client.update_restaurant(id, &r_update).await {
                            Ok(_) => {
                                app.editing_restaurant_id = None;
                                let is_customer = app.api_client.role == Some(UserRole::Customer);
                                if is_customer {
                                    app.screen = Screen::MyRestaurants;
                                    app.focus = Focus::MyRestaurantsList;
                                    app.fetch_my_restaurants().await;
                                } else {
                                    app.screen = Screen::Dashboard;
                                    app.focus = Focus::RestaurantList;
                                    app.fetch_restaurants().await;
                                }
                                app.set_status("Restaurant updated successfully");
                            }
                            Err(e) => {
                                app.set_status(&format!("Update failed: {}", e));
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
    use api::Location;

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

        handle_login_keys(&mut app, event::KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL)).await;
        assert_eq!(app.screen, Screen::Signup);
        assert_eq!(app.focus, Focus::SignupUsername);

        handle_signup_keys(&mut app, event::KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)).await;
        assert_eq!(app.focus, Focus::SignupPassword);
        handle_signup_keys(&mut app, event::KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)).await;
        assert_eq!(app.focus, Focus::SignupRole);

        assert_eq!(app.input_role, UserRole::Customer);
        handle_signup_keys(&mut app, event::KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)).await;
        assert_eq!(app.input_role, UserRole::Admin);
        handle_signup_keys(&mut app, event::KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)).await;
        assert_eq!(app.input_role, UserRole::Consumer);
        handle_signup_keys(&mut app, event::KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)).await;
        assert_eq!(app.input_role, UserRole::Customer);

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

    #[tokio::test]
    async fn test_form_type_cycling_includes_food_cart() {
        let mut app = App::new("http://localhost:8000");
        app.screen = Screen::AddRestaurant;
        app.focus = Focus::FormType;

        assert_eq!(app.form_type, RestaurantType::FoodStall);
        handle_form_keys(&mut app, event::KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)).await;
        assert_eq!(app.form_type, RestaurantType::FoodTruck);
        handle_form_keys(&mut app, event::KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)).await;
        assert_eq!(app.form_type, RestaurantType::FoodCart);
        handle_form_keys(&mut app, event::KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)).await;
        assert_eq!(app.form_type, RestaurantType::BrickAndMortar);
        handle_form_keys(&mut app, event::KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)).await;
        assert_eq!(app.form_type, RestaurantType::FoodStall);
    }

    #[tokio::test]
    async fn test_form_tab_sequence_includes_new_fields() {
        let mut app = App::new("http://localhost:8000");
        app.screen = Screen::AddRestaurant;
        app.focus = Focus::FormName;

        let tab_sequence = [
            Focus::FormType,
            Focus::FormCuisineType,
            Focus::FormLocation,
            Focus::FormLocationDesc,
            Focus::FormLocationLat,
            Focus::FormLocationLng,
            Focus::FormLocationAddress,
            Focus::FormLocationCity,
            Focus::FormLocationState,
            Focus::FormLocationZip,
            Focus::FormLocationRoad1,
            Focus::FormLocationRoad2,
            Focus::FormLocationVenue,
            Focus::FormLocationStall,
            Focus::FormLocationLot,
            Focus::FormOpenTime,
            Focus::FormCloseTime,
            Focus::FormOpenStatus,
            Focus::FormDescription,
            Focus::FormMenuItems,
            Focus::FormName,
        ];

        for expected_focus in &tab_sequence {
            handle_form_keys(&mut app, event::KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)).await;
            assert_eq!(app.focus, *expected_focus);
        }
    }

    #[tokio::test]
    async fn test_my_restaurants_navigation() {
        let mut app = App::new("http://localhost:8000");
        app.screen = Screen::MyRestaurants;
        app.focus = Focus::MyRestaurantsList;

        let loc1 = Location {
            id: 1,
            location_type: "other".to_string(),
            formatted: "Loc1".to_string(),
            description: Some("Loc1".to_string()),
            lat: None, lng: None, address: None, city: None, state: None,
            zip_code: None, road_1: None, road_2: None,
            venue_name: None, stall_number: None, lot_name: None,
        };
        let loc2 = Location {
            id: 2,
            location_type: "other".to_string(),
            formatted: "Loc2".to_string(),
            description: Some("Loc2".to_string()),
            lat: None, lng: None, address: None, city: None, state: None,
            zip_code: None, road_1: None, road_2: None,
            venue_name: None, stall_number: None, lot_name: None,
        };
        let r1 = Restaurant {
            id: 1,
            name: "Test1".to_string(),
            restaurant_type: RestaurantType::FoodStall,
            cuisine_type: String::new(),
            location: loc1,
            open_time: "08:00:00".to_string(),
            close_time: "22:00:00".to_string(),
            open_status: true,
            description: None,
            menu_items: None,
            is_approved: false,
            owner_id: Some(1),
            pending_location: None,
            location_change_pending: false,
        };
        let r2 = Restaurant {
            id: 2,
            name: "Test2".to_string(),
            restaurant_type: RestaurantType::FoodTruck,
            cuisine_type: String::new(),
            location: loc2,
            open_time: "09:00:00".to_string(),
            close_time: "21:00:00".to_string(),
            open_status: false,
            description: None,
            menu_items: None,
            is_approved: false,
            owner_id: Some(1),
            pending_location: None,
            location_change_pending: false,
        };
        app.my_restaurants = vec![r1, r2];

        assert_eq!(app.selected_my_restaurant_index, None);
        handle_my_restaurants_keys(&mut app, event::KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)).await;
        assert_eq!(app.selected_my_restaurant_index, Some(0));

        handle_my_restaurants_keys(&mut app, event::KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)).await;
        assert_eq!(app.selected_my_restaurant_index, Some(1));

        handle_my_restaurants_keys(&mut app, event::KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)).await;
        assert_eq!(app.selected_my_restaurant_index, Some(0));
    }

    #[tokio::test]
    async fn test_favorites_screen_exit() {
        let mut app = App::new("http://localhost:8000");
        app.screen = Screen::FavoritesView;
        app.focus = Focus::FavoritesList;

        handle_favorites_keys(&mut app, event::KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)).await;
        assert_eq!(app.screen, Screen::Dashboard);
    }
}
