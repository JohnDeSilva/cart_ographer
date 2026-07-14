export type RestaurantType = 'Food Stall' | 'Food Truck' | 'Brick and mortar Restaurant';

export type UserRole = 'Admin' | 'Customer';

export interface Restaurant {
  id: number;
  name: string;
  restaurant_type: RestaurantType;
  location: string;
  open_time: string;
  close_time: string;
  open_status: boolean;
  description?: string;
}

export interface RestaurantCreate {
  name: string;
  restaurant_type: RestaurantType;
  location: string;
  open_time: string;
  close_time: string;
  open_status: boolean;
  description?: string;
}

export interface RestaurantUpdate {
  name?: string;
  restaurant_type?: RestaurantType;
  location?: string;
  open_time?: string;
  close_time?: string;
  open_status?: boolean;
  description?: string;
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

  signup(username: string, password: string, role: UserRole = 'Customer'): Promise<{ id: number; username: string; role: UserRole }> {
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
}

export const api = new ApiClient();
