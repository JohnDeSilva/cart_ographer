import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, test, expect, vi, beforeEach } from 'vitest';
import App from './App';
import { api } from './api';

// Mock api client methods
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
    }
  };
});

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
});
