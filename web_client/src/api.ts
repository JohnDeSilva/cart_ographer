export type RestaurantType = 'Food Stall' | 'Food Truck' | 'Brick and mortar Restaurant' | 'Food Cart';

export type UserRole = 'Admin' | 'Customer' | 'Consumer';

export interface Location {
  id: number;
  location_type: string;
  formatted: string;
  description?: string;
  lat?: number;
  lng?: number;
  address?: string;
  city?: string;
  state?: string;
  zip_code?: string;
  road_1?: string;
  road_2?: string;
  venue_name?: string;
  stall_number?: string;
  lot_name?: string;
}

export interface LocationCreatePayload {
  location_type: string;
  description?: string;
  lat?: number;
  lng?: number;
  address?: string;
  city?: string;
  state?: string;
  zip_code?: string;
  road_1?: string;
  road_2?: string;
  venue_name?: string;
  stall_number?: string;
  lot_name?: string;
}

export interface LocationUpdatePayload {
  description?: string;
  lat?: number;
  lng?: number;
  address?: string;
  city?: string;
  state?: string;
  zip_code?: string;
  road_1?: string;
  road_2?: string;
  venue_name?: string;
  stall_number?: string;
  lot_name?: string;
}

export interface MenuItem {
  id: number;
  restaurant_id: number;
  name: string;
  description?: string;
  price?: number;
  is_sold_out: boolean;
  sort_order: number;
}

export interface MenuItemCreate {
  name: string;
  description?: string;
  price?: number;
  sort_order?: number;
}

export interface MenuItemUpdate {
  name?: string;
  description?: string;
  price?: number;
  is_sold_out?: boolean;
  sort_order?: number;
}

export interface Restaurant {
  id: number;
  name: string;
  restaurant_type: RestaurantType;
  cuisine_type: string;
  location: Location;
  open_time: string;
  close_time: string;
  open_status: boolean;
  description?: string;
  menu_items: MenuItem[];
  is_approved: boolean;
  owner_id?: number;
}

export interface RestaurantCreate {
  name: string;
  restaurant_type: RestaurantType;
  cuisine_type: string;
  location: LocationCreatePayload;
  open_time: string;
  close_time: string;
  open_status: boolean;
  description?: string;
}

export interface RestaurantSubmit {
  name: string;
  restaurant_type: RestaurantType;
  cuisine_type: string;
  location: LocationCreatePayload;
  open_time: string;
  close_time: string;
  open_status: boolean;
  description?: string;
}

export interface RestaurantUpdate {
  name?: string;
  restaurant_type?: RestaurantType;
  cuisine_type?: string;
  location?: LocationUpdatePayload;
  open_time?: string;
  close_time?: string;
  open_status?: boolean;
  description?: string;
}

export interface FavoriteResponse {
  id: number;
  consumer_id: number;
  restaurant_id: number;
  restaurant?: Restaurant;
}

const BASE_URL = 'http://localhost:8000';

class ApiClient {
  private getHeaders(): HeadersInit {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };
    const token = localStorage.getItem('access_token');
    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }
    return headers;
  }

  async request<T>(path: string, options: RequestInit = {}): Promise<T> {
    const url = `${BASE_URL}${path}`;
    const response = await fetch(url, {
      ...options,
      headers: {
        ...this.getHeaders(),
        ...options.headers,
      },
    });

    if (response.status === 401) {
      localStorage.clear();
      window.dispatchEvent(new Event('auth_change'));
    }

    if (!response.ok) {
      let errorMsg = 'An error occurred';
      try {
        const errorData = await response.json();
        errorMsg = errorData.detail || errorMsg;
      } catch {
        // use default
      }
      throw new Error(errorMsg);
    }

    if (response.status === 204) {
      return {} as T;
    }

    return response.json();
  }

  async login(username: string, password: string): Promise<{ access_token: string; role: UserRole; username: string }> {
    const res = await this.request<{ access_token: string; role: UserRole; username: string }>('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ username, password }),
    });
    localStorage.setItem('access_token', res.access_token);
    localStorage.setItem('username', res.username);
    localStorage.setItem('role', res.role);
    window.dispatchEvent(new Event('auth_change'));
    return res;
  }

  signup(username: string, password: string, role: UserRole = 'Consumer'): Promise<{ id: number; username: string; role: UserRole }> {
    return this.request('/auth/signup', {
      method: 'POST',
      body: JSON.stringify({ username, password, role }),
    });
  }

  resetPassword(username: string, password: string): Promise<{ id: number; username: string; role: UserRole }> {
    return this.request('/auth/reset-password', {
      method: 'POST',
      body: JSON.stringify({ username, new_password: password }),
    });
  }

  logout(): void {
    localStorage.clear();
    window.dispatchEvent(new Event('auth_change'));
  }

  getRestaurants(nameFilter?: string): Promise<Restaurant[]> {
    const query = nameFilter ? `?name=${encodeURIComponent(nameFilter)}` : '';
    return this.request<Restaurant[]>(`/restaurants${query}`);
  }

  createRestaurant(restaurant: RestaurantCreate): Promise<Restaurant> {
    return this.request<Restaurant>('/restaurants', {
      method: 'POST',
      body: JSON.stringify(restaurant),
    });
  }

  submitRestaurant(restaurant: RestaurantSubmit): Promise<Restaurant> {
    return this.request<Restaurant>('/restaurants/submit', {
      method: 'POST',
      body: JSON.stringify(restaurant),
    });
  }

  getMyRestaurants(): Promise<Restaurant[]> {
    return this.request<Restaurant[]>('/me/restaurants');
  }

  updateRestaurant(id: number, restaurant: RestaurantUpdate): Promise<Restaurant> {
    return this.request<Restaurant>(`/restaurants/${id}`, {
      method: 'PUT',
      body: JSON.stringify(restaurant),
    });
  }

  updateRestaurantStatus(id: number, open_status: boolean): Promise<Restaurant> {
    return this.request<Restaurant>(`/restaurants/${id}/status`, {
      method: 'PATCH',
      body: JSON.stringify({ open_status }),
    });
  }

  deleteRestaurant(id: number): Promise<void> {
    return this.request<void>(`/restaurants/${id}`, {
      method: 'DELETE',
    });
  }

  approveRestaurant(id: number, isApproved: boolean): Promise<Restaurant> {
    return this.request<Restaurant>(`/restaurants/${id}/approve`, {
      method: 'PATCH',
      body: JSON.stringify({ is_approved: isApproved }),
    });
  }

  addFavorite(restaurantId: number): Promise<FavoriteResponse> {
    return this.request<FavoriteResponse>('/favorites', {
      method: 'POST',
      body: JSON.stringify({ restaurant_id: restaurantId }),
    });
  }

  removeFavorite(favoriteId: number): Promise<void> {
    return this.request<void>(`/favorites/${favoriteId}`, {
      method: 'DELETE',
    });
  }

  getFavorites(): Promise<FavoriteResponse[]> {
    return this.request<FavoriteResponse[]>('/favorites');
  }

  createMenuItem(restaurantId: number, item: MenuItemCreate): Promise<MenuItem> {
    return this.request<MenuItem>(`/restaurants/${restaurantId}/menu-items`, {
      method: 'POST',
      body: JSON.stringify(item),
    });
  }

  getMenuItems(restaurantId: number): Promise<MenuItem[]> {
    return this.request<MenuItem[]>(`/restaurants/${restaurantId}/menu-items`);
  }

  updateMenuItem(restaurantId: number, itemId: number, item: MenuItemUpdate): Promise<MenuItem> {
    return this.request<MenuItem>(`/restaurants/${restaurantId}/menu-items/${itemId}`, {
      method: 'PUT',
      body: JSON.stringify(item),
    });
  }

  toggleSoldOut(restaurantId: number, itemId: number, isSoldOut: boolean): Promise<MenuItem> {
    return this.request<MenuItem>(`/restaurants/${restaurantId}/menu-items/${itemId}/sold-out`, {
      method: 'PATCH',
      body: JSON.stringify({ is_sold_out: isSoldOut }),
    });
  }

  deleteMenuItem(restaurantId: number, itemId: number): Promise<void> {
    return this.request<void>(`/restaurants/${restaurantId}/menu-items/${itemId}`, {
      method: 'DELETE',
    });
  }
}

export const api = new ApiClient();
