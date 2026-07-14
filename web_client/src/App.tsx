import React, { useState, useEffect } from 'react';
import { 
  MapPin, Clock, Search, Shield, User, Power, 
  FileEdit, Trash2, PlusCircle, AlertCircle, 
  ChevronRight, Sparkles, Key, CheckCircle
} from 'lucide-react';
import { api, Restaurant, RestaurantType, UserRole } from './api';

export default function App() {
  const [view, setView] = useState<'login' | 'signup' | 'reset' | 'dashboard' | 'form-add' | 'form-edit'>('login');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [role, setRole] = useState<UserRole>('Customer');
  const [authName, setAuthName] = useState<string | null>(null);
  const [authRole, setAuthRole] = useState<UserRole | null>(null);

  // Error/Success status
  const [statusMsg, setStatusMsg] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  // Dashboard state
  const [restaurants, setRestaurants] = useState<Restaurant[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedId, setSelectedId] = useState<number | null>(null);

  // Form states
  const [formName, setFormName] = useState('');
  const [formType, setFormType] = useState<RestaurantType>('Food Stall');
  const [formLocation, setFormLocation] = useState('');
  const [formOpenTime, setFormOpenTime] = useState('08:00:00');
  const [formCloseTime, setFormCloseTime] = useState('22:00:00');
  const [formOpenStatus, setFormOpenStatus] = useState(true);
  const [formDescription, setFormDescription] = useState('');

  // Sync auth on mount and token changes
  const checkAuth = () => {
    const token = localStorage.getItem('access_token');
    const u = localStorage.getItem('username');
    const r = localStorage.getItem('role') as UserRole | null;
    if (token && u && r) {
      setAuthName(u);
      setAuthRole(r);
      setView('dashboard');
    } else {
      setAuthName(null);
      setAuthRole(null);
      setView('login');
    }
  };

  useEffect(() => {
    checkAuth();
    window.addEventListener('auth_change', checkAuth);
    return () => window.removeEventListener('auth_change', checkAuth);
  }, []);

  // Fetch list of restaurants
  const fetchRestaurants = async (filter?: string) => {
    try {
      const data = await api.getRestaurants(filter);
      setRestaurants(data);
      if (data.length > 0 && !selectedId) {
        setSelectedId(data[0].id);
      }
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  useEffect(() => {
    if (view === 'dashboard') {
      fetchRestaurants(searchQuery);
    }
  }, [view, searchQuery]);

  const showMsg = (type: 'success' | 'error', text: string) => {
    setStatusMsg({ type, text });
    setTimeout(() => setStatusMsg(null), 5000);
  };

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await api.login(username, password);
      showMsg('success', 'Logged in successfully!');
      setUsername('');
      setPassword('');
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  const handleSignup = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await api.signup(username, password, role);
      showMsg('success', 'Account created successfully! Please sign in.');
      setView('login');
      setUsername('');
      setPassword('');
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  const handleResetPassword = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await api.resetPassword(username, password);
      showMsg('success', 'Password reset successfully! Please sign in.');
      setView('login');
      setUsername('');
      setPassword('');
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  const handleLogout = () => {
    api.logout();
    showMsg('success', 'Logged out successfully');
  };

  const toggleRestaurantStatus = async (r: Restaurant) => {
    try {
      await api.updateRestaurantStatus(r.id, !r.open_status);
      fetchRestaurants(searchQuery);
      showMsg('success', 'Restaurant status updated');
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  const handleDeleteRestaurant = async (id: number) => {
    if (!window.confirm('Are you sure you want to delete this restaurant?')) return;
    try {
      await api.deleteRestaurant(id);
      setSelectedId(null);
      fetchRestaurants(searchQuery);
      showMsg('success', 'Restaurant deleted successfully');
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  const openEditForm = (r: Restaurant) => {
    setFormName(r.name);
    setFormType(r.restaurant_type);
    setFormLocation(r.location);
    setFormOpenTime(r.open_time);
    setFormCloseTime(r.close_time);
    setFormOpenStatus(r.open_status);
    setFormDescription(r.description || '');
    setView('form-edit');
  };

  const openAddForm = () => {
    setFormName('');
    setFormType('Food Stall');
    setFormLocation('');
    setFormOpenTime('08:00:00');
    setFormCloseTime('22:00:00');
    setFormOpenStatus(true);
    setFormDescription('');
    setView('form-add');
  };

  const handleSaveRestaurant = async (e: React.FormEvent) => {
    e.preventDefault();
    if (formOpenTime.length !== 8 || formCloseTime.length !== 8) {
      showMsg('error', 'Time parameters must match HH:MM:SS format');
      return;
    }

    try {
      const payload = {
        name: formName,
        restaurant_type: formType,
        location: formLocation,
        open_time: formOpenTime,
        close_time: formCloseTime,
        open_status: formOpenStatus,
        description: formDescription.trim() ? formDescription : undefined,
      };

      if (view === 'form-add') {
        await api.createRestaurant(payload);
        showMsg('success', 'Restaurant created successfully');
      } else if (view === 'form-edit' && selectedId) {
        await api.updateRestaurant(selectedId, payload);
        showMsg('success', 'Restaurant updated successfully');
      }

      setView('dashboard');
      fetchRestaurants(searchQuery);
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  const selectedRestaurant = restaurants.find(r => r.id === selectedId);

  return (
    <div className="container">
      {/* Header */}
      <header className="app-header">
        <div className="logo">
          <Sparkles size={24} /> CART_OGRAPHER
        </div>
        {authName && (
          <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
            <div className="user-badge">
              <User size={14} />
              <span>
                {authName} (<span className="role">{authRole}</span>)
              </span>
            </div>
            <button onClick={handleLogout} className="btn btn-secondary" style={{ padding: '6px 12px', width: 'auto' }}>
              <Power size={14} />
            </button>
          </div>
        )}
      </header>

      {statusMsg && (
        <div className={`status-alert ${statusMsg.type}`}>
          <AlertCircle size={16} />
          {statusMsg.text}
        </div>
      )}

      {/* Screen Views */}
      {view === 'login' && (
        <div className="auth-wrapper">
          <div className="auth-card glass-panel">
            <h2>Sign In</h2>
            <div className="subtitle">Enter credentials to view tracking route</div>
            <form onSubmit={handleLogin}>
              <div className="form-group">
                <label>Username</label>
                <input 
                  type="text" 
                  className="form-control" 
                  value={username} 
                  onChange={e => setUsername(e.target.value)} 
                  required 
                  placeholder="admin or customer"
                />
              </div>
              <div className="form-group">
                <label>Password</label>
                <input 
                  type="password" 
                  className="form-control" 
                  value={password} 
                  onChange={e => setPassword(e.target.value)} 
                  required 
                  placeholder="••••••••"
                />
              </div>
              <button type="submit" className="btn btn-primary">Sign In</button>
            </form>
            <div className="auth-links">
              <a href="#signup" onClick={() => setView('signup')}>Need an account? Sign up</a>
              <a href="#reset" onClick={() => setView('reset')}>Forgot password? Reset</a>
            </div>
          </div>
        </div>
      )}

      {view === 'signup' && (
        <div className="auth-wrapper">
          <div className="auth-card glass-panel">
            <h2>Sign Up</h2>
            <div className="subtitle">Register to discover local vendors</div>
            <form onSubmit={handleSignup}>
              <div className="form-group">
                <label>Username</label>
                <input 
                  type="text" 
                  className="form-control" 
                  value={username} 
                  onChange={e => setUsername(e.target.value)} 
                  required 
                  placeholder="Choose username"
                />
              </div>
              <div className="form-group">
                <label>Password</label>
                <input 
                  type="password" 
                  className="form-control" 
                  value={password} 
                  onChange={e => setPassword(e.target.value)} 
                  required 
                  placeholder="Choose password"
                />
              </div>
              <div className="form-group">
                <label>Role</label>
                <select 
                  className="form-control" 
                  value={role} 
                  onChange={e => setRole(e.target.value as UserRole)}
                >
                  <option value="Customer">Customer (Read-only)</option>
                  <option value="Admin">Admin (CRUD access)</option>
                </select>
              </div>
              <button type="submit" className="btn btn-primary">Sign Up</button>
            </form>
            <div className="auth-links">
              <a href="#login" onClick={() => setView('login')}>Already have an account? Sign in</a>
            </div>
          </div>
        </div>
      )}

      {view === 'reset' && (
        <div className="auth-wrapper">
          <div className="auth-card glass-panel">
            <h2>Reset Password</h2>
            <div className="subtitle">Set a new password for account recovery</div>
            <form onSubmit={handleResetPassword}>
              <div className="form-group">
                <label>Target Username</label>
                <input 
                  type="text" 
                  className="form-control" 
                  value={username} 
                  onChange={e => setUsername(e.target.value)} 
                  required 
                  placeholder="Enter username"
                />
              </div>
              <div className="form-group">
                <label>New Password</label>
                <input 
                  type="password" 
                  className="form-control" 
                  value={password} 
                  onChange={e => setPassword(e.target.value)} 
                  required 
                  placeholder="Enter new password"
                />
              </div>
              <button type="submit" className="btn btn-primary">Update Password</button>
            </form>
            <div className="auth-links">
              <a href="#login" onClick={() => setView('login')}>Back to sign in</a>
            </div>
          </div>
        </div>
      )}

      {view === 'dashboard' && (
        <div className="dashboard-grid">
          {/* Sidebar Left */}
          <div className="sidebar-panel glass-panel">
            <div className="search-box">
              <Search size={16} className="icon" />
              <input 
                type="text" 
                className="form-control" 
                placeholder="Search by name..." 
                value={searchQuery}
                onChange={e => setSearchQuery(e.target.value)}
              />
            </div>
            
            {authRole === 'Admin' && (
              <button 
                onClick={openAddForm} 
                className="btn btn-primary" 
                style={{ marginBottom: '20px' }}
              >
                <PlusCircle size={16} /> Add Restaurant
              </button>
            )}

            <div className="restaurant-list">
              {restaurants.map(r => (
                <div 
                  key={r.id} 
                  className={`restaurant-item ${selectedId === r.id ? 'active' : ''}`}
                  onClick={() => setSelectedId(r.id)}
                >
                  <div>
                    <div className="restaurant-name">{r.name}</div>
                    <div className="restaurant-meta">{r.restaurant_type}</div>
                  </div>
                  <span className={`badge ${r.open_status ? 'badge-open' : 'badge-closed'}`}>
                    {r.open_status ? 'Open' : 'Closed'}
                  </span>
                </div>
              ))}
              {restaurants.length === 0 && (
                <div className="empty-state">
                  <Search className="icon" size={32} />
                  <span>No restaurants match your search.</span>
                </div>
              )}
            </div>
          </div>

          {/* Details Card Right */}
          <div className="details-panel glass-panel">
            {selectedRestaurant ? (
              <>
                <div className="details-header">
                  <div className="details-title">
                    <h2>{selectedRestaurant.name}</h2>
                    <div className="type">{selectedRestaurant.restaurant_type}</div>
                  </div>
                  <span className={`badge ${selectedRestaurant.open_status ? 'badge-open' : 'badge-closed'}`} style={{ padding: '6px 12px', fontSize: '13px' }}>
                    {selectedRestaurant.open_status ? 'Open for business' : 'Closed'}
                  </span>
                </div>

                <div className="details-body">
                  <div className="detail-row">
                    <MapPin size={16} className="icon" />
                    <span className="value">{selectedRestaurant.location}</span>
                  </div>
                  <div className="detail-row">
                    <Clock size={16} className="icon" />
                    <span className="value">
                      Operating Hours: {selectedRestaurant.open_time} - {selectedRestaurant.close_time}
                    </span>
                  </div>

                  <div className="details-description">
                    <h3>Description</h3>
                    <p>{selectedRestaurant.description || 'No description provided.'}</p>
                  </div>

                  {authRole === 'Admin' && (
                    <div className="admin-actions">
                      <button 
                        onClick={() => openEditForm(selectedRestaurant)} 
                        className="btn btn-secondary"
                      >
                        <FileEdit size={16} /> Edit Details
                      </button>
                      <button 
                        onClick={() => toggleRestaurantStatus(selectedRestaurant)} 
                        className="btn btn-secondary"
                      >
                        <CheckCircle size={16} /> Toggle Status
                      </button>
                      <button 
                        onClick={() => handleDeleteRestaurant(selectedRestaurant.id)} 
                        className="btn btn-danger"
                      >
                        <Trash2 size={16} /> Delete
                      </button>
                    </div>
                  )}
                </div>
              </>
            ) : (
              <div className="empty-state" style={{ margin: 'auto' }}>
                <MapPin size={48} className="icon" />
                <h2>No Vendor Selected</h2>
                <span>Select a restaurant from the sidebar menu to view operating details, times, and location.</span>
              </div>
            )}
          </div>
        </div>
      )}

      {(view === 'form-add' || view === 'form-edit') && (
        <div className="form-panel glass-panel">
          <h2>{view === 'form-add' ? 'Create Restaurant Entry' : 'Update Restaurant Details'}</h2>
          <form onSubmit={handleSaveRestaurant}>
            <div className="form-grid">
              <div className="form-group form-span-2">
                <label>Restaurant Name</label>
                <input 
                  type="text" 
                  className="form-control" 
                  value={formName}
                  onChange={e => setFormName(e.target.value)}
                  required
                  placeholder="e.g. Gourmet Burger Corner"
                />
              </div>

              <div className="form-group">
                <label>Restaurant Type</label>
                <select 
                  className="form-control"
                  value={formType}
                  onChange={e => setFormType(e.target.value as RestaurantType)}
                >
                  <option value="Food Stall">Food Stall</option>
                  <option value="Food Truck">Food Truck</option>
                  <option value="Brick and mortar Restaurant">Brick & mortar Restaurant</option>
                </select>
              </div>

              <div className="form-group">
                <label>Location Address</label>
                <input 
                  type="text" 
                  className="form-control" 
                  value={formLocation}
                  onChange={e => setFormLocation(e.target.value)}
                  required
                  placeholder="e.g. 5th Ave Street Stall 4"
                />
              </div>

              <div className="form-group">
                <label>Open Time (HH:MM:SS)</label>
                <input 
                  type="text" 
                  className="form-control" 
                  value={formOpenTime}
                  onChange={e => setFormOpenTime(e.target.value)}
                  required
                />
              </div>

              <div className="form-group">
                <label>Close Time (HH:MM:SS)</label>
                <input 
                  type="text" 
                  className="form-control" 
                  value={formCloseTime}
                  onChange={e => setFormCloseTime(e.target.value)}
                  required
                />
              </div>

              <div className="form-group form-span-2">
                <div className="checkbox-group">
                  <input 
                    type="checkbox" 
                    id="openStatusCheckbox"
                    checked={formOpenStatus}
                    onChange={e => setFormOpenStatus(e.target.checked)}
                  />
                  <label htmlFor="openStatusCheckbox" style={{ margin: 0, cursor: 'pointer' }}>
                    Active (Flag as currently open for business)
                  </label>
                </div>
              </div>

              <div className="form-group form-span-2">
                <label>Description Details</label>
                <textarea 
                  className="form-control" 
                  rows={4}
                  value={formDescription}
                  onChange={e => setFormDescription(e.target.value)}
                  placeholder="Add details, menu highlights, specials..."
                />
              </div>
            </div>

            <div style={{ display: 'flex', gap: '16px', marginTop: '24px' }}>
              <button type="submit" className="btn btn-primary" style={{ width: 'auto', padding: '12px 32px' }}>
                Save Changes
              </button>
              <button 
                type="button" 
                onClick={() => setView('dashboard')} 
                className="btn btn-secondary" 
                style={{ width: 'auto', padding: '12px 32px' }}
              >
                Cancel
              </button>
            </div>
          </form>
        </div>
      )}
    </div>
  );
}
