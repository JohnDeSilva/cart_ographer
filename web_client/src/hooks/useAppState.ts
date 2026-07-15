import { useState, useEffect } from 'react';
import { api, Restaurant, RestaurantType, UserRole, FavoriteResponse, MenuItem } from '../api';

export function useAppState() {
  const [view, setView] = useState<'login' | 'signup' | 'reset' | 'dashboard' | 'list' | 'map' | 'form-add' | 'form-edit' | 'customer-submissions' | 'admin-approvals' | 'consumer-favorites'>('login');
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
  const [formMenuItems, setFormMenuItems] = useState<MenuItem[]>([]);
  const [formNewMenuName, setFormNewMenuName] = useState('');
  const [formNewMenuPrice, setFormNewMenuPrice] = useState('');
  const [formNewMenuDesc, setFormNewMenuDesc] = useState('');
  const [editingMenuId, setEditingMenuId] = useState<number | null>(null);
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

  const [consumerFavorites, setConsumerFavorites] = useState<FavoriteResponse[]>([]);
  const [favoriteRestaurantIds, setFavoriteRestaurantIds] = useState<Set<number>>(new Set());

  const [menuOpen, setMenuOpen] = useState(false);
  const [detailOnly, setDetailOnly] = useState(false);
  const closeMenu = () => setMenuOpen(false);

  const checkAuth = () => {
    const token = localStorage.getItem('access_token');
    const u = localStorage.getItem('username');
    const r = localStorage.getItem('role') as UserRole | null;
    if (token && u && r) {
      setAuthName(u);
      setAuthRole(r);
      setView(r === 'Consumer' ? 'map' : 'dashboard');
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
    if (view === 'dashboard' || view === 'map' || view === 'list') {
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
    setSelectedId(r.id);
    setFormName(r.name);
    setFormType(r.restaurant_type);
    setFormCuisineType(r.cuisine_type);
    setFormLocation(r.location.formatted);
    setFormOpenTime(r.open_time);
    setFormCloseTime(r.close_time);
    setFormOpenStatus(r.open_status);
    setFormDescription(r.description || '');
    setFormMenuItems(r.menu_items || []);
    setFormNewMenuName('');
    setFormNewMenuPrice('');
    setFormNewMenuDesc('');
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
    setFormMenuItems([]);
    setFormNewMenuName('');
    setFormNewMenuPrice('');
    setFormNewMenuDesc('');
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
    setDetailOnly(true);
    setView('dashboard');
  };

  const selectedRestaurant = restaurants.find(r => r.id === selectedId);
  const isSelectedFavorited = selectedRestaurant ? favoriteRestaurantIds.has(selectedRestaurant.id) : false;

  return {
    view, setView,
    username, setUsername,
    password, setPassword,
    role, setRole,
    authName, authRole,
    statusMsg,
    restaurants, setRestaurants,
    searchQuery, setSearchQuery,
    selectedId, setSelectedId,
    formName, setFormName,
    formType, setFormType,
    formCuisineType, setFormCuisineType,
    formLocation, setFormLocation,
    formOpenTime, setFormOpenTime,
    formCloseTime, setFormCloseTime,
    formOpenStatus, setFormOpenStatus,
    formDescription, setFormDescription,
    formMenuItems, setFormMenuItems,
    formNewMenuName, setFormNewMenuName,
    formNewMenuPrice, setFormNewMenuPrice,
    formNewMenuDesc, setFormNewMenuDesc,
    editingMenuId, setEditingMenuId,
    formLocDescription, setFormLocDescription,
    formLocLat, setFormLocLat,
    formLocLng, setFormLocLng,
    formLocAddress, setFormLocAddress,
    formLocCity, setFormLocCity,
    formLocState, setFormLocState,
    formLocZip, setFormLocZip,
    formLocRoad1, setFormLocRoad1,
    formLocRoad2, setFormLocRoad2,
    formLocVenue, setFormLocVenue,
    formLocStall, setFormLocStall,
    formLocLot, setFormLocLot,
    myRestaurants,
    approvalRestaurants,
    consumerFavorites, favoriteRestaurantIds,
    menuOpen, setMenuOpen,
    closeMenu, detailOnly, setDetailOnly,
    selectedRestaurant, isSelectedFavorited,
    handleLogin, handleSignup, handleResetPassword, handleLogout,
    toggleRestaurantStatus, handleDeleteRestaurant,
    openEditForm, openAddForm, handleSaveRestaurant,
    handleApproveRestaurant,
    handleToggleFavorite, handleRemoveFavorite,
    navigateToDetail,
    fetchRestaurants, fetchMyRestaurants,
  };
}
