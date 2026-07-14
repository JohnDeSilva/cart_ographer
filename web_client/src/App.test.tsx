import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, test, expect, vi, beforeEach } from 'vitest';
import App from './App';
import { api, Restaurant } from './api';

vi.mock('./api', async (importOriginal) => {
  const actual: any = await importOriginal();
  return {
    ...actual,
    api: {
      login: vi.fn(),
      signup: vi.fn(),
      resetPassword: vi.fn(),
      getRestaurants: vi.fn().mockResolvedValue([]),
      logout: vi.fn(),
      submitRestaurant: vi.fn(),
      getMyRestaurants: vi.fn().mockResolvedValue([]),
      approveRestaurant: vi.fn(),
      approveLocationChange: vi.fn(),
      addFavorite: vi.fn(),
      removeFavorite: vi.fn(),
      getFavorites: vi.fn().mockResolvedValue([]),
      updateRestaurant: vi.fn(),
      updateRestaurantStatus: vi.fn(),
      deleteRestaurant: vi.fn(),
      createRestaurant: vi.fn(),
    }
  };
});

const setupAuth = (role: string, username = 'testuser') => {
  localStorage.setItem('access_token', 'test-token');
  localStorage.setItem('username', username);
  localStorage.setItem('role', role);
};

const mockLocation = (overrides = {}) => ({
  id: 1,
  location_type: 'street_address',
  formatted: '123 Main St, Portland, OR',
  address: '123 Main St',
  city: 'Portland',
  state: 'OR',
  description: '123 Main St',
  ...overrides,
});

const mockRestaurant = (overrides = {}) => ({
  id: 1,
  name: 'Test Restaurant',
  restaurant_type: 'Food Stall' as Restaurant['restaurant_type'],
  cuisine_type: 'Italian',
  location: mockLocation(),
  open_time: '08:00:00',
  close_time: '22:00:00',
  open_status: true,
  description: 'A test restaurant',
  menu_items: 'Pizza, Pasta',
  is_approved: true,
  location_change_pending: false,
  ...overrides,
} as Restaurant);

describe('Web UI - App Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
  });

  test('renders Sign In screen by default', () => {
    render(<App />);
    expect(screen.getByRole('heading', { name: /sign in/i })).toBeInTheDocument();
    expect(screen.getByPlaceholderText(/admin or customer/i)).toBeInTheDocument();
  });

  test('switches to Sign Up view upon link click', () => {
    render(<App />);
    const signupLink = screen.getByText(/sign up/i);
    fireEvent.click(signupLink);
    expect(screen.getByRole('heading', { name: /sign up/i })).toBeInTheDocument();
  });

  test('switches to Reset Password view upon link click', () => {
    render(<App />);
    const resetLink = screen.getByText(/reset/i);
    fireEvent.click(resetLink);
    expect(screen.getByRole('heading', { name: /reset password/i })).toBeInTheDocument();
  });

  test('submits login credentials and updates storage', async () => {
    vi.mocked(api.login).mockResolvedValue({
      access_token: 'dummy_web_jwt',
      role: 'Customer',
      username: 'test_web_user',
    });

    render(<App />);

    const userField = screen.getByPlaceholderText(/admin or customer/i);
    const passwordField = screen.getByPlaceholderText(/••••••••/i);
    const submitButton = screen.getByRole('button', { name: /sign in/i });

    fireEvent.change(userField, { target: { value: 'test_web_user' } });
    fireEvent.change(passwordField, { target: { value: 'test_web_password' } });
    fireEvent.click(submitButton);

    await waitFor(() => {
      expect(api.login).toHaveBeenCalledWith('test_web_user', 'test_web_password');
    });
  });

  test('signup form includes Consumer, Customer, and Admin role options', () => {
    render(<App />);
    const signupLink = screen.getByText(/sign up/i);
    fireEvent.click(signupLink);
    const dropdown = screen.getByRole('combobox');
    expect(dropdown).toContainHTML('Consumer');
    expect(dropdown).toContainHTML('Customer');
    expect(dropdown).toContainHTML('Admin');
  });

  test('Admin sees Approvals navigation tab', () => {
    setupAuth('Admin');
    render(<App />);
    expect(screen.getByText(/approvals/i)).toBeInTheDocument();
  });

  test('Customer sees My Submissions navigation tab', () => {
    setupAuth('Customer');
    render(<App />);
    expect(screen.getByRole('button', { name: /my submissions/i })).toBeInTheDocument();
  });

  test('Consumer sees My Favorites navigation tab', () => {
    setupAuth('Consumer');
    render(<App />);
    expect(screen.getByText(/my favorites/i)).toBeInTheDocument();
  });

  test('Admin does not see My Submissions tab', () => {
    setupAuth('Admin');
    render(<App />);
    expect(screen.queryByText(/my submissions/i)).not.toBeInTheDocument();
  });

  test('Consumer does not see Approvals tab', () => {
    setupAuth('Consumer');
    render(<App />);
    expect(screen.queryByText(/approvals/i)).not.toBeInTheDocument();
  });

  test('Customer does not see My Favorites tab', () => {
    setupAuth('Customer');
    render(<App />);
    expect(screen.queryByText(/my favorites/i)).not.toBeInTheDocument();
  });

  test('Consumer sees Add to Favorites button on restaurant detail', async () => {
    setupAuth('Consumer');
    const mockRestaurants = [mockRestaurant()];
    vi.mocked(api.getRestaurants).mockResolvedValue(mockRestaurants);

    render(<App />);

    const browseButton = screen.getByRole('button', { name: /browse/i });
    fireEvent.click(browseButton);

    await waitFor(() => {
      const sidebarItems = screen.getAllByText('Test Restaurant');
      expect(sidebarItems.length).toBeGreaterThan(0);
    });

    const sidebarItem = screen.getAllByText('Test Restaurant')[0];
    fireEvent.click(sidebarItem);

    await waitFor(() => {
      expect(screen.getByText(/add to favorites/i)).toBeInTheDocument();
    });
  });

  test('Consumer can toggle favorite status on a restaurant', async () => {
    setupAuth('Consumer');
    const restaurantData = mockRestaurant();
    vi.mocked(api.getRestaurants).mockResolvedValue([restaurantData]);
    vi.mocked(api.getFavorites).mockResolvedValue([]);
    vi.mocked(api.addFavorite).mockResolvedValue({
      id: 1,
      consumer_id: 1,
      restaurant_id: 1,
      restaurant: restaurantData,
    });

    render(<App />);

    const browseButton = screen.getByRole('button', { name: /browse/i });
    fireEvent.click(browseButton);

    await waitFor(() => {
      const sidebarItems = screen.getAllByText('Test Restaurant');
      expect(sidebarItems.length).toBeGreaterThan(0);
    });

    const sidebarItem = screen.getAllByText('Test Restaurant')[0];
    fireEvent.click(sidebarItem);

    await waitFor(() => {
      expect(screen.getByText(/add to favorites/i)).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText(/add to favorites/i));

    await waitFor(() => {
      expect(api.addFavorite).toHaveBeenCalledWith(1);
    });
  });

  test('Customer submissions view shows Submit New button', async () => {
    setupAuth('Customer');
    vi.mocked(api.getMyRestaurants).mockResolvedValue([]);

    render(<App />);

    const submissionsTab = screen.getByRole('button', { name: /my submissions/i });
    fireEvent.click(submissionsTab);

    await waitFor(() => {
      expect(screen.getByText(/submit new restaurant/i)).toBeInTheDocument();
    });
  });

  test('Admin approvals view loads pending restaurants', async () => {
    setupAuth('Admin');
    const pendingRestaurant = mockRestaurant({ is_approved: false, name: 'Pending Place' });
    const approvedRestaurant = mockRestaurant({ id: 2, is_approved: true, name: 'Approved Place' });
    vi.mocked(api.getRestaurants).mockResolvedValue([pendingRestaurant, approvedRestaurant]);

    render(<App />);

    const approvalsTab = screen.getByText(/approvals/i);
    fireEvent.click(approvalsTab);

    await waitFor(() => {
      expect(screen.getByText('Pending Place')).toBeInTheDocument();
    });
  });

  test('Consumer favorites view loads and displays favorited restaurants', async () => {
    setupAuth('Consumer');
    const restaurantData = mockRestaurant();
    vi.mocked(api.getFavorites).mockResolvedValue([
      { id: 1, consumer_id: 1, restaurant_id: 1, restaurant: restaurantData },
    ]);

    render(<App />);

    const favTab = screen.getByText(/my favorites/i);
    fireEvent.click(favTab);

    await waitFor(() => {
      expect(screen.getByText('Test Restaurant')).toBeInTheDocument();
    });
  });

  test('Restaurant detail shows pending location information', async () => {
    setupAuth('Admin');
    const restaurantWithPendingLocation = mockRestaurant({
      location: mockLocation(),
      pending_location: mockLocation({ formatted: '456 New St', address: '456 New St' }),
      location_change_pending: true,
    });
    vi.mocked(api.getRestaurants).mockResolvedValue([restaurantWithPendingLocation]);

    render(<App />);

    await waitFor(() => {
      const sidebarItems = screen.getAllByText('Test Restaurant');
      expect(sidebarItems.length).toBeGreaterThan(0);
    });

    const sidebarItem = screen.getAllByText('Test Restaurant')[0];
    fireEvent.click(sidebarItem);

    await waitFor(() => {
      expect(screen.getByText(/456 New St/)).toBeInTheDocument();
    });
  });

  test('Add Restaurant form includes Cuisine Type and Menu Items fields', async () => {
    setupAuth('Admin');
    render(<App />);

    const addButton = screen.getByText(/add restaurant/i);
    fireEvent.click(addButton);

    await waitFor(() => {
      expect(screen.getByPlaceholderText(/e.g. Italian, Mexican/i)).toBeInTheDocument();
      expect(screen.getByPlaceholderText(/list menu items/i)).toBeInTheDocument();
    });
  });
});
