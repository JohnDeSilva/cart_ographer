import { useState, useEffect } from 'react';
import {
  MapPin, Clock, Search, User, Power,
  FileEdit, Trash2, PlusCircle, AlertCircle,
  Sparkles, CheckCircle,
  Heart, Check, X
} from 'lucide-react';
import { api, Restaurant, RestaurantType, UserRole, FavoriteResponse } from './api';
import MapView from './MapView';

export default function App() {
  const [view, setView] = useState<'login' | 'signup' | 'reset' | 'dashboard' | 'map' | 'form-add' | 'form-edit' | 'customer-submissions' | 'admin-approvals' | 'consumer-favorites'>('login');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [role, setRole] = useState<UserRole>('Consumer');
  const [authName, setAuthName] = useState<string | null>(null);
  const [authRole, setAuthRole] = useState<UserRole | null>(null);

  const [statusMsg, setStatusMsg] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  const [restaurants, setRestaurants] = useState<Restaurant[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedId, setSelectedId] = useState<number | null>(null);

  const [formName, setFormName] = useState('');
  const [formType, setFormType] = useState<RestaurantType>('Food Stall');
  const [formCuisineType, setFormCuisineType] = useState('');
  const [formLocation, setFormLocation] = useState('');
  const [formOpenTime, setFormOpenTime] = useState('08:00:00');
  const [formCloseTime, setFormCloseTime] = useState('22:00:00');
  const [formOpenStatus, setFormOpenStatus] = useState(true);
  const [formDescription, setFormDescription] = useState('');
  const [formMenuItems, setFormMenuItems] = useState('');
  const [formLocDescription, setFormLocDescription] = useState('');
  const [formLocLat, setFormLocLat] = useState('');
  const [formLocLng, setFormLocLng] = useState('');
  const [formLocAddress, setFormLocAddress] = useState('');
  const [formLocCity, setFormLocCity] = useState('');
  const [formLocState, setFormLocState] = useState('');
  const [formLocZip, setFormLocZip] = useState('');
  const [formLocRoad1, setFormLocRoad1] = useState('');
  const [formLocRoad2, setFormLocRoad2] = useState('');
  const [formLocVenue, setFormLocVenue] = useState('');
  const [formLocStall, setFormLocStall] = useState('');
  const [formLocLot, setFormLocLot] = useState('');

  const [myRestaurants, setMyRestaurants] = useState<Restaurant[]>([]);

  const [approvalRestaurants, setApprovalRestaurants] = useState<Restaurant[]>([]);
  const [approvalLocationChanges, setApprovalLocationChanges] = useState<Restaurant[]>([]);

  const [consumerFavorites, setConsumerFavorites] = useState<FavoriteResponse[]>([]);
  const [favoriteRestaurantIds, setFavoriteRestaurantIds] = useState<Set<number>>(new Set());

  const checkAuth = () => {
    const token = localStorage.getItem('access_token');
    const u = localStorage.getItem('username');
    const r = localStorage.getItem('role') as UserRole | null;
    if (token && u && r) {
      setAuthName(u);
      setAuthRole(r);
      setView(r === 'Customer' ? 'customer-submissions' : r === 'Consumer' ? 'map' : 'dashboard');
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
    if (view === 'dashboard' || view === 'map') {
      fetchRestaurants(searchQuery);
    }
  }, [view, searchQuery]);

  useEffect(() => {
    if (view === 'dashboard' && authRole === 'Consumer') {
      loadFavorites();
    }
  }, [view, authRole]);

  const fetchMyRestaurants = async () => {
    try {
      const data = await api.getMyRestaurants();
      setMyRestaurants(data);
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  useEffect(() => {
    if (view === 'customer-submissions') {
      fetchMyRestaurants();
    }
  }, [view]);

  const fetchApprovalsData = async () => {
    try {
      const allData = await api.getRestaurants();
      setApprovalRestaurants(allData.filter(r => !r.is_approved));
      setApprovalLocationChanges(allData.filter(r => r.location_change_pending));
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  useEffect(() => {
    if (view === 'admin-approvals') {
      fetchApprovalsData();
    }
  }, [view]);

  const loadFavorites = async () => {
    try {
      const favs = await api.getFavorites();
      setConsumerFavorites(favs);
      setFavoriteRestaurantIds(new Set(favs.map(f => f.restaurant_id)));
    } catch {
      // silent
    }
  };

  useEffect(() => {
    if (view === 'consumer-favorites') {
      loadFavorites();
    }
  }, [view]);

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
      if (view === 'customer-submissions') {
        fetchMyRestaurants();
      } else {
        fetchRestaurants(searchQuery);
      }
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
    setFormCuisineType(r.cuisine_type);
    setFormLocation(r.location.formatted);
    setFormOpenTime(r.open_time);
    setFormCloseTime(r.close_time);
    setFormOpenStatus(r.open_status);
    setFormDescription(r.description || '');
    setFormMenuItems(r.menu_items || '');
    setFormLocDescription(r.location.description || '');
    setFormLocLat(r.location.lat !== undefined && r.location.lat !== null ? String(r.location.lat) : '');
    setFormLocLng(r.location.lng !== undefined && r.location.lng !== null ? String(r.location.lng) : '');
    setFormLocAddress(r.location.address || '');
    setFormLocCity(r.location.city || '');
    setFormLocState(r.location.state || '');
    setFormLocZip(r.location.zip_code || '');
    setFormLocRoad1(r.location.road_1 || '');
    setFormLocRoad2(r.location.road_2 || '');
    setFormLocVenue(r.location.venue_name || '');
    setFormLocStall(r.location.stall_number || '');
    setFormLocLot(r.location.lot_name || '');
    setView('form-edit');
  };

  const openAddForm = () => {
    setFormName('');
    setFormType('Food Stall');
    setFormCuisineType('');
    setFormLocation('');
    setFormOpenTime('08:00:00');
    setFormCloseTime('22:00:00');
    setFormOpenStatus(true);
    setFormDescription('');
    setFormMenuItems('');
    setFormLocDescription('');
    setFormLocLat('');
    setFormLocLng('');
    setFormLocAddress('');
    setFormLocCity('');
    setFormLocState('');
    setFormLocZip('');
    setFormLocRoad1('');
    setFormLocRoad2('');
    setFormLocVenue('');
    setFormLocStall('');
    setFormLocLot('');
    setView('form-add');
  };

  const handleSaveRestaurant = async (e: React.FormEvent) => {
    e.preventDefault();
    if (formOpenTime.length !== 8 || formCloseTime.length !== 8) {
      showMsg('error', 'Time parameters must match HH:MM:SS format');
      return;
    }

    const buildLocationPayload = (isCreate: boolean) => {
      const locType = formType === 'Brick and mortar Restaurant' ? 'street_address' :
        formType === 'Food Cart' ? 'food_court' :
        formType === 'Food Truck' ? 'gps' :
        formType === 'Food Stall' ? 'intersection' : 'other';
      const base: any = {
        location_type: locType,
        description: formLocation || undefined,
        lat: formLocLat ? parseFloat(formLocLat) : undefined,
        lng: formLocLng ? parseFloat(formLocLng) : undefined,
        address: formLocAddress || undefined,
        city: formLocCity || undefined,
        state: formLocState || undefined,
        zip_code: formLocZip || undefined,
        road_1: formLocRoad1 || undefined,
        road_2: formLocRoad2 || undefined,
        venue_name: formLocVenue || undefined,
        stall_number: formLocStall || undefined,
        lot_name: formLocLot || undefined,
      };
      if (!isCreate) delete (base as any).location_type;
      return base;
    };

    try {
      const isAdd = view === 'form-add';
      const locPayload = buildLocationPayload(isAdd);
      const { location_type, ...locUpdatePayload } = locPayload;

      if (isAdd) {
        if (authRole === 'Customer') {
          await api.submitRestaurant({
            name: formName,
            restaurant_type: formType,
            cuisine_type: formCuisineType,
            location: locPayload,
            open_time: formOpenTime,
            close_time: formCloseTime,
            open_status: formOpenStatus,
            description: formDescription.trim() ? formDescription : undefined,
            menu_items: formMenuItems.trim() ? formMenuItems : undefined,
          });
          showMsg('success', 'Restaurant submitted for approval');
        } else {
          await api.createRestaurant({
            name: formName,
            restaurant_type: formType,
            cuisine_type: formCuisineType,
            location: locPayload,
            open_time: formOpenTime,
            close_time: formCloseTime,
            open_status: formOpenStatus,
            description: formDescription.trim() ? formDescription : undefined,
            menu_items: formMenuItems.trim() ? formMenuItems : undefined,
          });
          showMsg('success', 'Restaurant created successfully');
        }
      } else if (view === 'form-edit' && selectedId) {
        await api.updateRestaurant(selectedId, {
          name: formName,
          restaurant_type: formType,
          cuisine_type: formCuisineType,
          open_time: formOpenTime,
          close_time: formCloseTime,
          open_status: formOpenStatus,
          description: formDescription.trim() ? formDescription : undefined,
          menu_items: formMenuItems.trim() ? formMenuItems : undefined,
          location: locUpdatePayload,
        });
        showMsg('success', 'Restaurant updated successfully');
      }

      setView('dashboard');
      fetchRestaurants(searchQuery);
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  const handleApproveRestaurant = async (id: number) => {
    try {
      await api.approveRestaurant(id, true);
      showMsg('success', 'Restaurant approved successfully');
      fetchApprovalsData();
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  const handleApproveLocation = async (id: number, approve: boolean) => {
    try {
      await api.approveLocationChange(id, approve);
      showMsg('success', approve ? 'Location change approved' : 'Location change rejected');
      fetchApprovalsData();
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  const handleToggleFavorite = async (restaurantId: number) => {
    try {
      const isFavorited = favoriteRestaurantIds.has(restaurantId);
      if (isFavorited) {
        const fav = consumerFavorites.find(f => f.restaurant_id === restaurantId);
        if (fav) {
          await api.removeFavorite(fav.id);
          showMsg('success', 'Removed from favorites');
        }
      } else {
        await api.addFavorite(restaurantId);
        showMsg('success', 'Added to favorites');
      }
      await loadFavorites();
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  const handleRemoveFavorite = async (favoriteId: number) => {
    try {
      await api.removeFavorite(favoriteId);
      showMsg('success', 'Removed from favorites');
      loadFavorites();
    } catch (err: any) {
      showMsg('error', err.message);
    }
  };

  const navigateToDetail = (restaurantId: number) => {
    setSelectedId(restaurantId);
    setView('dashboard');
  };

  const selectedRestaurant = restaurants.find(r => r.id === selectedId);
  const isSelectedFavorited = selectedRestaurant ? favoriteRestaurantIds.has(selectedRestaurant.id) : false;

  return (
    <div className="container">
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

      {authName && (
        <nav style={{ display: 'flex', gap: '8px', marginBottom: '24px', flexWrap: 'wrap' }}>
          <button
            onClick={() => { setSelectedId(null); setView('dashboard'); }}
            className="btn btn-secondary"
            style={{
              width: 'auto',
              padding: '8px 16px',
              fontSize: '14px',
              ...(view === 'dashboard' ? { borderColor: 'var(--primary-color)', background: 'rgba(6, 182, 212, 0.1)' } : {}),
            }}
          >
            Browse
          </button>
          {authRole === 'Customer' && (
            <button
              onClick={() => { setSelectedId(null); setView('customer-submissions'); }}
              className="btn btn-secondary"
              style={{
                width: 'auto',
                padding: '8px 16px',
                fontSize: '14px',
                ...(view === 'customer-submissions' ? { borderColor: 'var(--primary-color)', background: 'rgba(6, 182, 212, 0.1)' } : {}),
              }}
            >
              My Submissions
            </button>
          )}
          {authRole === 'Admin' && (
            <button
              onClick={() => { setSelectedId(null); setView('admin-approvals'); }}
              className="btn btn-secondary"
              style={{
                width: 'auto',
                padding: '8px 16px',
                fontSize: '14px',
                ...(view === 'admin-approvals' ? { borderColor: 'var(--primary-color)', background: 'rgba(6, 182, 212, 0.1)' } : {}),
              }}
            >
              Approvals
            </button>
          )}
          {authRole === 'Consumer' && (
            <>
              <button
                onClick={() => { setSelectedId(null); setView('map'); }}
                className="btn btn-secondary"
                style={{
                  width: 'auto',
                  padding: '8px 16px',
                  fontSize: '14px',
                  ...(view === 'map' ? { borderColor: 'var(--primary-color)', background: 'rgba(6, 182, 212, 0.1)' } : {}),
                }}
              >
                Map
              </button>
              <button
                onClick={() => { setSelectedId(null); setView('consumer-favorites'); }}
                className="btn btn-secondary"
                style={{
                  width: 'auto',
                  padding: '8px 16px',
                  fontSize: '14px',
                  ...(view === 'consumer-favorites' ? { borderColor: 'var(--primary-color)', background: 'rgba(6, 182, 212, 0.1)' } : {}),
                }}
              >
                My Favorites
              </button>
            </>
          )}
        </nav>
      )}

      {statusMsg && (
        <div className={`status-alert ${statusMsg.type}`}>
          <AlertCircle size={16} />
          {statusMsg.text}
        </div>
      )}

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
                  <option value="Consumer">Consumer (Browse & Favorites)</option>
                  <option value="Customer">Customer (Restaurant Owner)</option>
                  <option value="Admin">Admin (Full Access)</option>
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

          <div className="details-panel glass-panel">
            {selectedRestaurant ? (
              <>
                <div className="details-header">
                  <div className="details-title">
                    <h2>{selectedRestaurant.name}</h2>
                    <div className="type">{selectedRestaurant.restaurant_type}</div>
                  </div>
                  <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', alignItems: 'flex-end' }}>
                    <span className={`badge ${selectedRestaurant.open_status ? 'badge-open' : 'badge-closed'}`} style={{ padding: '6px 12px', fontSize: '13px' }}>
                      {selectedRestaurant.open_status ? 'Open for business' : 'Closed'}
                    </span>
                    {!selectedRestaurant.is_approved && (
                      <span className="badge badge-closed" style={{ padding: '6px 12px', fontSize: '13px' }}>
                        Pending Approval
                      </span>
                    )}
                  </div>
                </div>

                <div className="details-body">
                  {(() => {
                    const loc = selectedRestaurant.location;
                    const rows: JSX.Element[] = [];
                    rows.push(
                      <div className="detail-row" key="loc-type">
                        <MapPin size={16} className="icon" />
                        <span className="value">
                          {loc.location_type.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase())}
                          {loc.formatted ? ` — ${loc.formatted}` : ''}
                        </span>
                      </div>
                    );
                    if (loc.description) rows.push(
                      <div className="detail-row" key="loc-desc" style={{ paddingLeft: '24px' }}>
                        <span className="value" style={{ fontSize: '13px', color: 'var(--text-secondary)' }}>{loc.description}</span>
                      </div>
                    );
                    if (loc.address) rows.push(
                      <div className="detail-row" key="loc-addr" style={{ paddingLeft: '24px' }}>
                        <span className="value">{loc.address}{loc.city ? `, ${loc.city}` : ''}{loc.state ? `, ${loc.state}` : ''}{loc.zip_code ? ` ${loc.zip_code}` : ''}</span>
                      </div>
                    );
                    if (loc.road_1) rows.push(
                      <div className="detail-row" key="loc-int" style={{ paddingLeft: '24px' }}>
                        <span className="value">{loc.road_1} & {loc.road_2}</span>
                      </div>
                    );
                    if (loc.lat !== undefined && loc.lat !== null) rows.push(
                      <div className="detail-row" key="loc-gps" style={{ paddingLeft: '24px' }}>
                        <span className="value">{loc.lat?.toFixed(4)}, {loc.lng?.toFixed(4)}</span>
                      </div>
                    );
                    if (loc.venue_name) rows.push(
                      <div className="detail-row" key="loc-venue" style={{ paddingLeft: '24px' }}>
                        <span className="value">{loc.venue_name}{loc.stall_number ? `, Stall ${loc.stall_number}` : ''}</span>
                      </div>
                    );
                    if (loc.lot_name) rows.push(
                      <div className="detail-row" key="loc-lot" style={{ paddingLeft: '24px' }}>
                        <span className="value">Lot: {loc.lot_name}</span>
                      </div>
                    );
                    if (selectedRestaurant.pending_location) {
                      const ploc = selectedRestaurant.pending_location;
                      rows.push(
                        <div className="detail-row" key="pending-header" style={{ color: 'var(--accent-color)', marginTop: '8px' }}>
                          <MapPin size={16} className="icon" />
                          <span className="value">Pending location change</span>
                        </div>
                      );
                      rows.push(
                        <div className="detail-row" key="pending-val" style={{ color: 'var(--accent-color)', paddingLeft: '24px' }}>
                          <span className="value">{ploc.formatted}</span>
                        </div>
                      );
                    }
                    return rows;
                  })()}
                  <div className="detail-row">
                    <Clock size={16} className="icon" />
                    <span className="value">
                      Operating Hours: {selectedRestaurant.open_time} - {selectedRestaurant.close_time}
                    </span>
                  </div>
                  {selectedRestaurant.cuisine_type && (
                    <div className="detail-row">
                      <span className="value">Cuisine: {selectedRestaurant.cuisine_type}</span>
                    </div>
                  )}

                  <div className="details-description">
                    <h3>Description</h3>
                    <p>{selectedRestaurant.description || 'No description provided.'}</p>
                  </div>

                  {selectedRestaurant.menu_items && (
                    <div className="details-description">
                      <h3>Menu Items</h3>
                      <p>{selectedRestaurant.menu_items}</p>
                    </div>
                  )}

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

                  {authRole === 'Consumer' && (
                    <div className="admin-actions">
                      <button
                        onClick={() => handleToggleFavorite(selectedRestaurant.id)}
                        className="btn btn-secondary"
                      >
                        <Heart
                          size={16}
                          fill={isSelectedFavorited ? 'var(--error-color)' : 'none'}
                          color={isSelectedFavorited ? 'var(--error-color)' : undefined}
                        />
                        {isSelectedFavorited ? ' Remove from Favorites' : ' Add to Favorites'}
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

      {view === 'customer-submissions' && (
        <div className="form-panel glass-panel" style={{ maxWidth: '900px' }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '24px' }}>
            <h2 style={{ margin: 0, border: 'none', padding: 0 }}>My Submissions</h2>
            <button onClick={openAddForm} className="btn btn-primary" style={{ width: 'auto', padding: '10px 20px' }}>
              <PlusCircle size={16} /> Submit New Restaurant
            </button>
          </div>

          {myRestaurants.length === 0 ? (
            <div className="empty-state">
              <MapPin size={48} className="icon" />
              <h2>No Submissions Yet</h2>
              <span>Submit your first restaurant for approval.</span>
            </div>
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
              {myRestaurants.map(r => (
                <div
                  key={r.id}
                  style={{
                    padding: '16px',
                    borderRadius: '10px',
                    background: 'rgba(255, 255, 255, 0.02)',
                    border: '1px solid var(--panel-border)',
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    cursor: 'pointer',
                    transition: 'all 0.2s ease',
                  }}
                  onClick={() => navigateToDetail(r.id)}
                  onMouseOver={e => (e.currentTarget.style.borderColor = 'rgba(6, 182, 212, 0.4)')}
                  onMouseOut={e => (e.currentTarget.style.borderColor = 'var(--panel-border)')}
                >
                  <div>
                    <div style={{ fontWeight: 600, fontSize: '16px', marginBottom: '4px' }}>{r.name}</div>
                    <div style={{ fontSize: '12px', color: 'var(--text-secondary)', display: 'flex', gap: '8px', alignItems: 'center' }}>
                      <span>{r.restaurant_type}</span>
                      <span>•</span>
                      <span className={`badge ${r.is_approved ? 'badge-open' : 'badge-closed'}`} style={{ fontSize: '10px' }}>
                        {r.is_approved ? 'Approved' : 'Pending'}
                      </span>
                      <span>•</span>
                      <span className={`badge ${r.open_status ? 'badge-open' : 'badge-closed'}`} style={{ fontSize: '10px' }}>
                        {r.open_status ? 'Open' : 'Closed'}
                      </span>
                    </div>
                  </div>
                  <div style={{ display: 'flex', gap: '8px' }} onClick={e => e.stopPropagation()}>
                    <button
                      onClick={() => openEditForm(r)}
                      className="btn btn-secondary"
                      style={{ width: 'auto', padding: '6px 12px', fontSize: '12px' }}
                    >
                      <FileEdit size={12} /> Edit
                    </button>
                    <button
                      onClick={() => toggleRestaurantStatus(r)}
                      className="btn btn-secondary"
                      style={{ width: 'auto', padding: '6px 12px', fontSize: '12px' }}
                    >
                      <CheckCircle size={12} /> Toggle
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {view === 'admin-approvals' && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '24px' }}>
          <div className="form-panel glass-panel" style={{ maxWidth: '900px' }}>
            <h2 style={{ margin: 0, border: 'none', padding: 0, marginBottom: '20px' }}>Pending Restaurant Approvals</h2>
            {approvalRestaurants.length === 0 ? (
              <div className="empty-state">
                <CheckCircle size={48} className="icon" />
                <span>All restaurants have been approved.</span>
              </div>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
                {approvalRestaurants.map(r => (
                  <div
                    key={r.id}
                    style={{
                      padding: '16px',
                      borderRadius: '10px',
                      background: 'rgba(255, 255, 255, 0.02)',
                      border: '1px solid var(--panel-border)',
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                    }}
                  >
                    <div>
                      <div style={{ fontWeight: 600, fontSize: '16px', marginBottom: '4px' }}>{r.name}</div>
                      <div style={{ fontSize: '12px', color: 'var(--text-secondary)' }}>
                        {r.restaurant_type} • {r.location.formatted}
                      </div>
                    </div>
                    <button
                      onClick={() => handleApproveRestaurant(r.id)}
                      className="btn btn-primary"
                      style={{ width: 'auto', padding: '8px 20px', fontSize: '13px' }}
                    >
                      <Check size={14} /> Approve
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>

          <div className="form-panel glass-panel" style={{ maxWidth: '900px' }}>
            <h2 style={{ margin: 0, border: 'none', padding: 0, marginBottom: '20px' }}>Pending Location Changes</h2>
            {approvalLocationChanges.length === 0 ? (
              <div className="empty-state">
                <MapPin size={48} className="icon" />
                <span>No pending location changes.</span>
              </div>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
                {approvalLocationChanges.map(r => (
                  <div
                    key={r.id}
                    style={{
                      padding: '16px',
                      borderRadius: '10px',
                      background: 'rgba(255, 255, 255, 0.02)',
                      border: '1px solid var(--panel-border)',
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                    }}
                  >
                    <div>
                      <div style={{ fontWeight: 600, fontSize: '16px', marginBottom: '4px' }}>{r.name}</div>
                      <div style={{ fontSize: '12px', color: 'var(--text-secondary)', marginBottom: '2px' }}>
                        Current: {r.location.formatted}
                      </div>
                      <div style={{ fontSize: '12px', color: 'var(--accent-color)' }}>
                        Proposed: {r.pending_location?.formatted}
                      </div>
                    </div>
                    <div style={{ display: 'flex', gap: '8px' }}>
                      <button
                        onClick={() => handleApproveLocation(r.id, true)}
                        className="btn btn-primary"
                        style={{ width: 'auto', padding: '8px 20px', fontSize: '13px' }}
                      >
                        <Check size={14} /> Approve
                      </button>
                      <button
                        onClick={() => handleApproveLocation(r.id, false)}
                        className="btn btn-danger"
                        style={{ width: 'auto', padding: '8px 20px', fontSize: '13px' }}
                      >
                        <X size={14} /> Reject
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      )}

      {view === 'consumer-favorites' && (
        <div className="form-panel glass-panel" style={{ maxWidth: '900px' }}>
          <h2 style={{ margin: 0, border: 'none', padding: 0, marginBottom: '24px' }}>My Favorites</h2>
          {consumerFavorites.length === 0 ? (
            <div className="empty-state">
              <Heart size={48} className="icon" />
              <h2>No Favorites Yet</h2>
              <span>Browse restaurants and add your favorites.</span>
            </div>
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
              {consumerFavorites.map(fav => (
                <div
                  key={fav.id}
                  style={{
                    padding: '16px',
                    borderRadius: '10px',
                    background: 'rgba(255, 255, 255, 0.02)',
                    border: '1px solid var(--panel-border)',
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    cursor: 'pointer',
                    transition: 'all 0.2s ease',
                  }}
                  onClick={() => fav.restaurant && navigateToDetail(fav.restaurant_id)}
                  onMouseOver={e => (e.currentTarget.style.borderColor = 'rgba(6, 182, 212, 0.4)')}
                  onMouseOut={e => (e.currentTarget.style.borderColor = 'var(--panel-border)')}
                >
                  <div>
                    <div style={{ fontWeight: 600, fontSize: '16px', marginBottom: '4px' }}>
                      {fav.restaurant ? fav.restaurant.name : `Restaurant #${fav.restaurant_id}`}
                    </div>
                    <div style={{ fontSize: '12px', color: 'var(--text-secondary)' }}>
                      {fav.restaurant ? `${fav.restaurant.restaurant_type}` : ''}
                    </div>
                  </div>
                  <button
                    onClick={e => { e.stopPropagation(); handleRemoveFavorite(fav.id); }}
                    className="btn btn-danger"
                    style={{ width: 'auto', padding: '6px 12px', fontSize: '12px' }}
                  >
                    <X size={12} /> Remove
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {view === 'map' && (
        <div style={{ width: '100%', flex: 1, minHeight: 0 }}>
          <MapView restaurants={restaurants} />
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
                  disabled={view === 'form-edit' && authRole === 'Customer'}
                  placeholder="e.g. Gourmet Burger Corner"
                />
                {authRole === 'Customer' && view === 'form-edit' && (
                  <div style={{ fontSize: '12px', color: 'var(--text-secondary)', marginTop: '4px' }}>
                    Name cannot be changed after submission.
                  </div>
                )}
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
                  <option value="Food Cart">Food Cart</option>
                  <option value="Brick and mortar Restaurant">Brick & mortar Restaurant</option>
                </select>
              </div>

              <div className="form-group">
                <label>Cuisine Type</label>
                <input
                  type="text"
                  className="form-control"
                  value={formCuisineType}
                  onChange={e => setFormCuisineType(e.target.value)}
                  placeholder="e.g. Italian, Mexican, Japanese"
                />
              </div>

              <div className="form-group form-span-2" style={{ border: '1px solid var(--panel-border)', borderRadius: '10px', padding: '16px', marginBottom: '8px' }}>
                <label style={{ fontWeight: 600, marginBottom: '12px', display: 'block' }}>Location Details</label>
                <div className="form-group" style={{ marginBottom: '12px' }}>
                  <label>Description (free-form)</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocation}
                    onChange={e => setFormLocation(e.target.value)}
                    placeholder="e.g. 5th Ave Street Stall 4"
                  />
                </div>
                <div className="form-group">
                  <label>Latitude</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocLat}
                    onChange={e => setFormLocLat(e.target.value)}
                    placeholder="e.g. 45.5152"
                  />
                </div>
                <div className="form-group">
                  <label>Longitude</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocLng}
                    onChange={e => setFormLocLng(e.target.value)}
                    placeholder="e.g. -122.6784"
                  />
                </div>
                <div className="form-group">
                  <label>Street Address</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocAddress}
                    onChange={e => setFormLocAddress(e.target.value)}
                    placeholder="e.g. 123 Main St"
                  />
                </div>
                <div className="form-group">
                  <label>City</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocCity}
                    onChange={e => setFormLocCity(e.target.value)}
                    placeholder="e.g. Portland"
                  />
                </div>
                <div className="form-group">
                  <label>State</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocState}
                    onChange={e => setFormLocState(e.target.value)}
                    placeholder="e.g. OR"
                  />
                </div>
                <div className="form-group">
                  <label>ZIP Code</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocZip}
                    onChange={e => setFormLocZip(e.target.value)}
                    placeholder="e.g. 97201"
                  />
                </div>
                <div className="form-group">
                  <label>Intersection — Road 1</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocRoad1}
                    onChange={e => setFormLocRoad1(e.target.value)}
                    placeholder="e.g. 5th Ave"
                  />
                </div>
                <div className="form-group">
                  <label>Intersection — Road 2</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocRoad2}
                    onChange={e => setFormLocRoad2(e.target.value)}
                    placeholder="e.g. Stark St"
                  />
                </div>
                <div className="form-group">
                  <label>Venue / Mall Name</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocVenue}
                    onChange={e => setFormLocVenue(e.target.value)}
                    placeholder="e.g. Portland Food Hall"
                  />
                </div>
                <div className="form-group">
                  <label>Stall / Cart Number</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocStall}
                    onChange={e => setFormLocStall(e.target.value)}
                    placeholder="e.g. Stall 7"
                  />
                </div>
                <div className="form-group">
                  <label>Parking Lot Name</label>
                  <input
                    type="text"
                    className="form-control"
                    value={formLocLot}
                    onChange={e => setFormLocLot(e.target.value)}
                    placeholder="e.g. Lot A"
                  />
                </div>
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
                  rows={3}
                  value={formDescription}
                  onChange={e => setFormDescription(e.target.value)}
                  placeholder="Add details, menu highlights, specials..."
                />
              </div>

              <div className="form-group form-span-2">
                <label>Menu Items</label>
                <textarea
                  className="form-control"
                  rows={3}
                  value={formMenuItems}
                  onChange={e => setFormMenuItems(e.target.value)}
                  placeholder="List menu items, separated by commas"
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
