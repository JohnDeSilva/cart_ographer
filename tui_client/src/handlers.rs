use crossterm::event::{self, KeyCode, KeyModifiers};
use crate::app::{App, Screen, Focus};
use crate::api::{
    LocationCreatePayload, LocationUpdatePayload, MenuItem, MenuItemCreate,
    MenuItemUpdate, Restaurant, RestaurantCreate, RestaurantUpdate, UserRole,
};

pub async fn handle_login_keys(app: &mut App, key: event::KeyEvent) {
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

pub async fn handle_signup_keys(app: &mut App, key: event::KeyEvent) {
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

pub async fn handle_reset_keys(app: &mut App, key: event::KeyEvent) {
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

pub async fn handle_dashboard_keys(app: &mut App, key: event::KeyEvent) {
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
        _ => {}
    }
}

pub async fn handle_my_restaurants_keys(app: &mut App, key: event::KeyEvent) {
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

pub async fn handle_favorites_keys(app: &mut App, key: event::KeyEvent) {
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

pub fn clear_form(app: &mut App) {
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
    app.form_menu_items = Vec::new();
    app.form_menu_item_name.clear();
    app.form_menu_item_price.clear();
    app.form_menu_item_description.clear();
    app.form_menu_item_selected = None;
    app.form_menu_items_removed = Vec::new();
}

pub fn populate_form_from_restaurant(app: &mut App, r: &Restaurant) {
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
    app.form_menu_items = r.menu_items.clone();
    app.form_menu_item_name.clear();
    app.form_menu_item_price.clear();
    app.form_menu_item_description.clear();
    app.form_menu_item_selected = None;
    app.form_menu_items_removed = Vec::new();
}

pub async fn handle_form_keys(app: &mut App, key: event::KeyEvent) {
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
                Focus::FormDescription => Focus::FormMenuItemName,
                Focus::FormMenuItemName => Focus::FormMenuItemPrice,
                Focus::FormMenuItemPrice => Focus::FormMenuItemDescription,
                Focus::FormMenuItemDescription => Focus::FormMenuList,
                Focus::FormMenuList => Focus::FormName,
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
                    Focus::FormMenuItemName => app.form_menu_item_name.push(' '),
                    Focus::FormMenuItemPrice => app.form_menu_item_price.push(' '),
                    Focus::FormMenuItemDescription => app.form_menu_item_description.push(' '),
                    _ => {}
                }
            }
        }
        KeyCode::Char('a') if app.focus == Focus::FormMenuItemName || app.focus == Focus::FormMenuItemPrice || app.focus == Focus::FormMenuItemDescription || app.focus == Focus::FormMenuList => {
            let name = app.form_menu_item_name.trim().to_string();
            if name.is_empty() {
                app.set_status("Menu item name cannot be empty");
                return;
            }
            let price = app.form_menu_item_price.trim().parse::<f64>().ok();
            let description = if app.form_menu_item_description.trim().is_empty() {
                None
            } else {
                Some(app.form_menu_item_description.trim().to_string())
            };
            let new_item = MenuItem {
                id: 0,
                restaurant_id: 0,
                name,
                description,
                price,
                is_sold_out: false,
                sort_order: (app.form_menu_items.len() + 1) as i32,
            };
            app.form_menu_items.push(new_item);
            app.form_menu_item_name.clear();
            app.form_menu_item_price.clear();
            app.form_menu_item_description.clear();
            app.set_status("Menu item added");
        }
        KeyCode::Char('t') if app.focus == Focus::FormMenuList => {
            if let Some(idx) = app.form_menu_item_selected {
                if let Some(item) = app.form_menu_items.get_mut(idx) {
                    item.is_sold_out = !item.is_sold_out;
                    app.set_status("Menu item toggled");
                }
            }
        }
        KeyCode::Char('d') if app.focus == Focus::FormMenuList => {
            if let Some(idx) = app.form_menu_item_selected {
                if let Some(item) = app.form_menu_items.get(idx) {
                    if item.id > 0 {
                        app.form_menu_items_removed.push(item.id);
                    }
                }
                app.form_menu_items.remove(idx);
                if let Some(sel) = app.form_menu_item_selected {
                    if sel >= app.form_menu_items.len() {
                        app.form_menu_item_selected = if app.form_menu_items.is_empty() { None } else { Some(app.form_menu_items.len() - 1) };
                    }
                }
                app.set_status("Menu item removed");
            }
        }
        KeyCode::Up if app.focus == Focus::FormMenuList => {
            if !app.form_menu_items.is_empty() {
                if let Some(ref mut idx) = app.form_menu_item_selected {
                    if *idx > 0 {
                        *idx -= 1;
                    }
                } else {
                    app.form_menu_item_selected = Some(app.form_menu_items.len() - 1);
                }
            }
        }
        KeyCode::Down if app.focus == Focus::FormMenuList => {
            if !app.form_menu_items.is_empty() {
                if let Some(ref mut idx) = app.form_menu_item_selected {
                    if *idx < app.form_menu_items.len() - 1 {
                        *idx += 1;
                    }
                } else {
                    app.form_menu_item_selected = Some(0);
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
                Focus::FormMenuItemName => app.form_menu_item_name.push(c),
                Focus::FormMenuItemPrice => app.form_menu_item_price.push(c),
                Focus::FormMenuItemDescription => app.form_menu_item_description.push(c),
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
                Focus::FormMenuItemName => { app.form_menu_item_name.pop(); }
                Focus::FormMenuItemPrice => { app.form_menu_item_price.pop(); }
                Focus::FormMenuItemDescription => { app.form_menu_item_description.pop(); }
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
            if app.focus == Focus::FormMenuItemName || app.focus == Focus::FormMenuItemPrice || app.focus == Focus::FormMenuItemDescription {
                let name = app.form_menu_item_name.trim().to_string();
                if name.is_empty() {
                    app.set_status("Menu item name cannot be empty");
                    return;
                }
                let price = app.form_menu_item_price.trim().parse::<f64>().ok();
                let description = if app.form_menu_item_description.trim().is_empty() {
                    None
                } else {
                    Some(app.form_menu_item_description.trim().to_string())
                };
                let new_item = MenuItem {
                    id: 0,
                    restaurant_id: 0,
                    name,
                    description,
                    price,
                    is_sold_out: false,
                    sort_order: (app.form_menu_items.len() + 1) as i32,
                };
                app.form_menu_items.push(new_item);
                app.form_menu_item_name.clear();
                app.form_menu_item_price.clear();
                app.form_menu_item_description.clear();
                app.set_status("Menu item added");
                return;
            }

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
                    };

                    let is_customer = app.api_client.role == Some(UserRole::Customer);
                    let create_result = if is_customer {
                        app.api_client.submit_restaurant(&r).await
                    } else {
                        app.api_client.create_restaurant(&r).await
                    };

                    match create_result {
                        Ok(created) => {
                            let restaurant_id = created.id;
                            for item in &app.form_menu_items {
                                if item.id == 0 {
                                    let create_payload = MenuItemCreate {
                                        name: item.name.clone(),
                                        description: item.description.clone(),
                                        price: item.price,
                                        sort_order: Some(item.sort_order),
                                    };
                                    let _ = app.api_client.create_menu_item(restaurant_id, &create_payload).await;
                                }
                            }
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
                        };
                        match app.api_client.update_restaurant(id, &r_update).await {
                            Ok(_) => {
                                for removed_id in &app.form_menu_items_removed {
                                    let _ = app.api_client.delete_menu_item(id, *removed_id).await;
                                }
                                for item in &app.form_menu_items {
                                    if item.id == 0 {
                                        let create_payload = MenuItemCreate {
                                            name: item.name.clone(),
                                            description: item.description.clone(),
                                            price: item.price,
                                            sort_order: Some(item.sort_order),
                                        };
                                        let _ = app.api_client.create_menu_item(id, &create_payload).await;
                                    } else {
                                        let update_payload = MenuItemUpdate {
                                            name: Some(item.name.clone()),
                                            description: item.description.clone(),
                                            price: item.price,
                                            is_sold_out: Some(item.is_sold_out),
                                            sort_order: Some(item.sort_order),
                                        };
                                        let _ = app.api_client.update_menu_item(id, item.id, &update_payload).await;
                                    }
                                }
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
