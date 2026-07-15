use crate::api::{ApiClient, FavoriteResponse, MenuItem, Restaurant, RestaurantType, UserRole};

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
    FormMenuItemName,
    FormMenuItemPrice,
    FormMenuItemDescription,
    FormMenuList,
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
    pub form_menu_items: Vec<MenuItem>,
    pub form_menu_item_name: String,
    pub form_menu_item_price: String,
    pub form_menu_item_description: String,
    pub form_menu_item_selected: Option<usize>,
    pub form_menu_items_removed: Vec<i64>,
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
            form_menu_items: Vec::new(),
            form_menu_item_name: String::new(),
            form_menu_item_price: String::new(),
            form_menu_item_description: String::new(),
            form_menu_item_selected: None,
            form_menu_items_removed: Vec::new(),
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
