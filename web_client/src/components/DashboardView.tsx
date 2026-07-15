import { Search, MapPin, Clock, FileEdit, CheckCircle, Trash2, PlusCircle, Heart, AlertCircle } from 'lucide-react';
import { Restaurant, UserRole, FavoriteResponse, MenuItem } from '../api';
import { api } from '../api';

interface DashboardViewProps {
  restaurants: Restaurant[];
  setRestaurants: (r: Restaurant[] | ((prev: Restaurant[]) => Restaurant[])) => void;
  searchQuery: string;
  setSearchQuery: (v: string) => void;
  selectedId: number | null;
  setSelectedId: (id: number | null) => void;
  authRole: UserRole | null;
  authName: string | null;
  selectedRestaurant: Restaurant | undefined;
  isSelectedFavorited: boolean;
  openEditForm: (r: Restaurant) => void;
  toggleRestaurantStatus: (r: Restaurant) => Promise<void>;
  handleDeleteRestaurant: (id: number) => Promise<void>;
  handleToggleFavorite: (restaurantId: number) => Promise<void>;
  openAddForm: () => void;
  showMsg: (type: 'success' | 'error', text: string) => void;
  setEditingMenuId: (id: number | null) => void;
  setFormNewMenuName: (v: string) => void;
  setFormNewMenuPrice: (v: string) => void;
  setFormNewMenuDesc: (v: string) => void;
  sidebar?: boolean;
  setDetailOnly?: (v: boolean) => void;
  setView?: (v: string) => void;
}

export default function DashboardView({
  restaurants, setRestaurants, searchQuery, setSearchQuery,
  selectedId, setSelectedId, authRole, selectedRestaurant,
  isSelectedFavorited, openEditForm, toggleRestaurantStatus,
  handleDeleteRestaurant, handleToggleFavorite, openAddForm,
  showMsg, setEditingMenuId, setFormNewMenuName, setFormNewMenuPrice, setFormNewMenuDesc,
  sidebar = true, setDetailOnly, setView,
}: DashboardViewProps) {
  return (
    <div className={sidebar ? 'dashboard-grid' : ''} style={sidebar ? undefined : { width: '100%', maxWidth: '900px', margin: '0 auto' }}>
      {sidebar && (
      <div className="sidebar-panel glass-panel">
        <div className="search-box">
          <Search size={16} className="icon" />
          <input type="text" className="form-control" placeholder="Search by name..." value={searchQuery} onChange={e => setSearchQuery(e.target.value)} />
        </div>

        {authRole === 'Admin' && (
          <button onClick={openAddForm} className="btn btn-primary" style={{ marginBottom: '20px' }}>
            <PlusCircle size={16} /> Add Restaurant
          </button>
        )}

        <div className="restaurant-list">
          {restaurants.map(r => (
            <div key={r.id} className={`restaurant-item ${selectedId === r.id ? 'active' : ''}`} onClick={() => { setDetailOnly?.(false); setSelectedId(r.id); }}>
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
      )}

      <div className="details-panel glass-panel" style={sidebar ? undefined : { maxWidth: '900px', margin: '0 auto', width: '100%' }}>
        {selectedRestaurant ? (
          <>
            {!sidebar && (
              <button
                onClick={() => { setDetailOnly?.(false); setView?.('map'); }}
                className="btn btn-secondary"
                style={{ marginBottom: '12px', width: 'auto', padding: '4px 12px', fontSize: '13px' }}
              >
                ← Back
              </button>
            )}
            <div className="details-header">
              <div className="details-title">
                <h2>{selectedRestaurant.name}</h2>
                <div className="type">{selectedRestaurant.restaurant_type}</div>
              </div>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', alignItems: 'flex-end' }}>
                {(authRole === 'Admin' || authRole === 'Customer') && (
                  <div style={{ display: 'flex', gap: '6px' }}>
                    <button onClick={() => openEditForm(selectedRestaurant)} className="btn btn-secondary" style={{ width: 'auto', padding: '4px 10px', fontSize: '12px' }}>
                      <FileEdit size={12} /> Edit
                    </button>
                    <button onClick={() => toggleRestaurantStatus(selectedRestaurant)} className="btn btn-secondary" style={{ width: 'auto', padding: '4px 10px', fontSize: '12px' }}>
                      <CheckCircle size={12} /> Toggle
                    </button>
                    {authRole === 'Admin' && (
                      <button onClick={() => handleDeleteRestaurant(selectedRestaurant.id)} className="btn btn-danger" style={{ width: 'auto', padding: '4px 10px', fontSize: '12px' }}>
                        <Trash2 size={12} /> Delete
                      </button>
                    )}
                  </div>
                )}
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

              {selectedRestaurant.menu_items.length > 0 && (
                <div className="details-description">
                  <h3>Menu Items</h3>
                  {selectedRestaurant.menu_items.map(item => (
                    <div key={item.id} style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '4px 0', textDecoration: item.is_sold_out ? 'line-through' : 'none', opacity: item.is_sold_out ? 0.6 : 1 }}>
                      <span style={{ flex: 1 }}>{item.name}</span>
                      {item.price !== null && item.price !== undefined && (
                        <span style={{ color: 'var(--text-secondary)', fontSize: '13px' }}>${item.price.toFixed(2)}</span>
                      )}
                      {item.is_sold_out && (
                        <span className="badge badge-closed" style={{ fontSize: '10px' }}>Sold Out</span>
                      )}
                      {(authRole === 'Admin' || authRole === 'Customer') && (
                        <>
                          <button type="button" className="btn btn-secondary" style={{ width: 'auto', padding: '2px 6px', fontSize: '10px' }}
                            onClick={() => {
                              openEditForm(selectedRestaurant);
                              setEditingMenuId(item.id);
                              setFormNewMenuName(item.name);
                              setFormNewMenuPrice(item.price != null ? String(item.price) : '');
                              setFormNewMenuDesc(item.description || '');
                            }}>
                            Edit
                          </button>
                          <button type="button" className="btn btn-secondary" style={{ width: 'auto', padding: '2px 6px', fontSize: '10px' }}
                            onClick={async () => {
                              try {
                                await api.toggleSoldOut(selectedRestaurant.id, item.id, !item.is_sold_out);
                                setRestaurants(prev => prev.map((r: Restaurant) => r.id === selectedRestaurant.id ? {
                                  ...r,
                                  menu_items: r.menu_items.map((mi: MenuItem) => mi.id === item.id ? { ...mi, is_sold_out: !mi.is_sold_out } : mi)
                                } : r));
                                showMsg('success', `Item ${item.is_sold_out ? 'enabled' : 'sold out'}`);
                              } catch (err: any) { showMsg('error', err.message); }
                            }}>
                            {item.is_sold_out ? 'Restock' : 'Sold Out'}
                          </button>
                          <button type="button" className="btn btn-danger" style={{ width: 'auto', padding: '2px 6px', fontSize: '10px' }}
                            onClick={async () => {
                              if (!window.confirm(`Remove "${item.name}"?`)) return;
                              try {
                                await api.deleteMenuItem(selectedRestaurant.id, item.id);
                                setRestaurants(prev => prev.map((r: Restaurant) => r.id === selectedRestaurant.id ? {
                                  ...r,
                                  menu_items: r.menu_items.filter((mi: MenuItem) => mi.id !== item.id)
                                } : r));
                                showMsg('success', 'Item removed');
                              } catch (err: any) { showMsg('error', err.message); }
                            }}>
                            Remove
                          </button>
                        </>
                      )}
                    </div>
                  ))}
                </div>
              )}

              {authRole === 'Consumer' && (
                <div className="admin-actions">
                  <button onClick={() => handleToggleFavorite(selectedRestaurant.id)} className="btn btn-secondary">
                    <Heart size={16} fill={isSelectedFavorited ? 'var(--error-color)' : 'none'} color={isSelectedFavorited ? 'var(--error-color)' : undefined} />
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
  );
}
