import { RestaurantType, UserRole, MenuItem } from '../api';
import { api } from '../api';

interface RestaurantFormProps {
  view: string;
  formName: string;
  setFormName: (v: string) => void;
  formType: RestaurantType;
  setFormType: (v: RestaurantType) => void;
  formCuisineType: string;
  setFormCuisineType: (v: string) => void;
  formLocation: string;
  setFormLocation: (v: string) => void;
  formOpenTime: string;
  setFormOpenTime: (v: string) => void;
  formCloseTime: string;
  setFormCloseTime: (v: string) => void;
  formOpenStatus: boolean;
  setFormOpenStatus: (v: boolean) => void;
  formDescription: string;
  setFormDescription: (v: string) => void;
  formMenuItems: MenuItem[];
  setFormMenuItems: (items: MenuItem[] | ((prev: MenuItem[]) => MenuItem[])) => void;
  formNewMenuName: string;
  setFormNewMenuName: (v: string) => void;
  formNewMenuPrice: string;
  setFormNewMenuPrice: (v: string) => void;
  formNewMenuDesc: string;
  setFormNewMenuDesc: (v: string) => void;
  editingMenuId: number | null;
  setEditingMenuId: (id: number | null) => void;
  formLocDescription: string;
  setFormLocDescription: (v: string) => void;
  formLocLat: string;
  setFormLocLat: (v: string) => void;
  formLocLng: string;
  setFormLocLng: (v: string) => void;
  formLocAddress: string;
  setFormLocAddress: (v: string) => void;
  formLocCity: string;
  setFormLocCity: (v: string) => void;
  formLocState: string;
  setFormLocState: (v: string) => void;
  formLocZip: string;
  setFormLocZip: (v: string) => void;
  formLocRoad1: string;
  setFormLocRoad1: (v: string) => void;
  formLocRoad2: string;
  setFormLocRoad2: (v: string) => void;
  formLocVenue: string;
  setFormLocVenue: (v: string) => void;
  formLocStall: string;
  setFormLocStall: (v: string) => void;
  formLocLot: string;
  setFormLocLot: (v: string) => void;
  selectedId: number | null;
  authRole: UserRole | null;
  handleSaveRestaurant: (e: React.FormEvent) => Promise<void>;
  setView: (v: any) => void;
  showMsg: (type: 'success' | 'error', text: string) => void;
}

export default function RestaurantForm({
  view, formName, setFormName, formType, setFormType,
  formCuisineType, setFormCuisineType, formLocation, setFormLocation,
  formOpenTime, setFormOpenTime, formCloseTime, setFormCloseTime,
  formOpenStatus, setFormOpenStatus, formDescription, setFormDescription,
  formMenuItems, setFormMenuItems,
  formNewMenuName, setFormNewMenuName,
  formNewMenuPrice, setFormNewMenuPrice,
  formNewMenuDesc, setFormNewMenuDesc,
  editingMenuId, setEditingMenuId,
  formLocDescription, setFormLocDescription,
  formLocLat, setFormLocLat, formLocLng, setFormLocLng,
  formLocAddress, setFormLocAddress, formLocCity, setFormLocCity,
  formLocState, setFormLocState, formLocZip, setFormLocZip,
  formLocRoad1, setFormLocRoad1, formLocRoad2, setFormLocRoad2,
  formLocVenue, setFormLocVenue, formLocStall, setFormLocStall,
  formLocLot, setFormLocLot,
  selectedId, authRole, handleSaveRestaurant, setView, showMsg,
}: RestaurantFormProps) {
  return (
    <div className="form-panel glass-panel">
      <h2>{view === 'form-add' ? 'Create Restaurant Entry' : 'Update Restaurant Details'}</h2>
      <form onSubmit={handleSaveRestaurant}>
        <div className="form-grid">
          <div className="form-group form-span-2">
            <label>Restaurant Name</label>
            <input type="text" className="form-control" value={formName} onChange={e => setFormName(e.target.value)} required
              disabled={view === 'form-edit' && authRole === 'Customer'} placeholder="e.g. Gourmet Burger Corner" />
            {authRole === 'Customer' && view === 'form-edit' && (
              <div style={{ fontSize: '12px', color: 'var(--text-secondary)', marginTop: '4px' }}>Name cannot be changed after submission.</div>
            )}
          </div>

          <div className="form-group">
            <label>Restaurant Type</label>
            <select className="form-control" value={formType} onChange={e => setFormType(e.target.value as RestaurantType)}>
              <option value="Food Stall">Food Stall</option>
              <option value="Food Truck">Food Truck</option>
              <option value="Food Cart">Food Cart</option>
              <option value="Brick and mortar Restaurant">Brick & mortar Restaurant</option>
            </select>
          </div>

          <div className="form-group">
            <label>Cuisine Type</label>
            <input type="text" className="form-control" value={formCuisineType} onChange={e => setFormCuisineType(e.target.value)} placeholder="e.g. Italian, Mexican, Japanese" />
          </div>

          <div className="form-group form-span-2" style={{ border: '1px solid var(--panel-border)', borderRadius: '10px', padding: '16px', marginBottom: '8px' }}>
            <label style={{ fontWeight: 600, marginBottom: '12px', display: 'block' }}>Location Details</label>
            <div className="form-group" style={{ marginBottom: '12px' }}>
              <label>Description (free-form)</label>
              <input type="text" className="form-control" value={formLocation} onChange={e => setFormLocation(e.target.value)} placeholder="e.g. 5th Ave Street Stall 4" />
            </div>
            <div className="form-group">
              <label>Latitude</label>
              <input type="text" className="form-control" value={formLocLat} onChange={e => setFormLocLat(e.target.value)} placeholder="e.g. 45.5152" />
            </div>
            <div className="form-group">
              <label>Longitude</label>
              <input type="text" className="form-control" value={formLocLng} onChange={e => setFormLocLng(e.target.value)} placeholder="e.g. -122.6784" />
            </div>
            <div className="form-group">
              <label>Street Address</label>
              <input type="text" className="form-control" value={formLocAddress} onChange={e => setFormLocAddress(e.target.value)} placeholder="e.g. 123 Main St" />
            </div>
            <div className="form-group">
              <label>City</label>
              <input type="text" className="form-control" value={formLocCity} onChange={e => setFormLocCity(e.target.value)} placeholder="e.g. Portland" />
            </div>
            <div className="form-group">
              <label>State</label>
              <input type="text" className="form-control" value={formLocState} onChange={e => setFormLocState(e.target.value)} placeholder="e.g. OR" />
            </div>
            <div className="form-group">
              <label>ZIP Code</label>
              <input type="text" className="form-control" value={formLocZip} onChange={e => setFormLocZip(e.target.value)} placeholder="e.g. 97201" />
            </div>
            <div className="form-group">
              <label>Intersection — Road 1</label>
              <input type="text" className="form-control" value={formLocRoad1} onChange={e => setFormLocRoad1(e.target.value)} placeholder="e.g. 5th Ave" />
            </div>
            <div className="form-group">
              <label>Intersection — Road 2</label>
              <input type="text" className="form-control" value={formLocRoad2} onChange={e => setFormLocRoad2(e.target.value)} placeholder="e.g. Stark St" />
            </div>
            <div className="form-group">
              <label>Venue / Mall Name</label>
              <input type="text" className="form-control" value={formLocVenue} onChange={e => setFormLocVenue(e.target.value)} placeholder="e.g. Portland Food Hall" />
            </div>
            <div className="form-group">
              <label>Stall / Cart Number</label>
              <input type="text" className="form-control" value={formLocStall} onChange={e => setFormLocStall(e.target.value)} placeholder="e.g. Stall 7" />
            </div>
            <div className="form-group">
              <label>Parking Lot Name</label>
              <input type="text" className="form-control" value={formLocLot} onChange={e => setFormLocLot(e.target.value)} placeholder="e.g. Lot A" />
            </div>
          </div>

          <div className="form-group">
            <label>Open Time (HH:MM:SS)</label>
            <input type="text" className="form-control" value={formOpenTime} onChange={e => setFormOpenTime(e.target.value)} required />
          </div>

          <div className="form-group">
            <label>Close Time (HH:MM:SS)</label>
            <input type="text" className="form-control" value={formCloseTime} onChange={e => setFormCloseTime(e.target.value)} required />
          </div>

          <div className="form-group form-span-2">
            <div className="checkbox-group">
              <input type="checkbox" id="openStatusCheckbox" checked={formOpenStatus} onChange={e => setFormOpenStatus(e.target.checked)} />
              <label htmlFor="openStatusCheckbox" style={{ margin: 0, cursor: 'pointer' }}>Active (Flag as currently open for business)</label>
            </div>
          </div>

          <div className="form-group form-span-2">
            <label>Description Details</label>
            <textarea className="form-control" rows={3} value={formDescription} onChange={e => setFormDescription(e.target.value)} placeholder="Add details, menu highlights, specials..." />
          </div>

          <div className="form-group form-span-2" style={{ border: '1px solid var(--panel-border)', borderRadius: '10px', padding: '16px', marginBottom: '8px' }}>
            <label style={{ fontWeight: 600, marginBottom: '12px', display: 'block' }}>Menu Items</label>
            {formMenuItems.map((item, idx) => (
              <div key={item.id || idx} style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '6px 0', borderBottom: '1px solid var(--panel-border)' }}>
                <span style={{ flex: 1, textDecoration: item.is_sold_out ? 'line-through' : 'none' }}>{item.name}</span>
                {item.price !== null && item.price !== undefined && (
                  <span style={{ fontSize: '13px', color: 'var(--text-secondary)' }}>${item.price.toFixed(2)}</span>
                )}
                {view === 'form-edit' && (
                  <>
                    <button type="button" className="btn btn-secondary" style={{ width: 'auto', padding: '4px 8px', fontSize: '11px' }}
                      onClick={() => {
                        setEditingMenuId(item.id);
                        setFormNewMenuName(item.name);
                        setFormNewMenuPrice(item.price != null ? String(item.price) : '');
                        setFormNewMenuDesc(item.description || '');
                      }}>
                      Edit
                    </button>
                    <button type="button" className="btn btn-secondary" style={{ width: 'auto', padding: '4px 8px', fontSize: '11px' }}
                      onClick={async () => {
                        try {
                          await api.toggleSoldOut(selectedId!, item.id, !item.is_sold_out);
                          setFormMenuItems((prev: MenuItem[]) => prev.map(mi => mi.id === item.id ? { ...mi, is_sold_out: !mi.is_sold_out } : mi));
                          showMsg('success', `Item ${item.is_sold_out ? 'enabled' : 'sold out'}`);
                        } catch (err: any) { showMsg('error', err.message); }
                      }}>
                      {item.is_sold_out ? 'Restock' : 'Sold Out'}
                    </button>
                    <button type="button" className="btn btn-danger" style={{ width: 'auto', padding: '4px 8px', fontSize: '11px' }}
                      onClick={async () => {
                        try {
                          await api.deleteMenuItem(selectedId!, item.id);
                          setFormMenuItems((prev: MenuItem[]) => prev.filter(mi => mi.id !== item.id));
                          showMsg('success', 'Item removed');
                        } catch (err: any) { showMsg('error', err.message); }
                      }}>
                      Remove
                    </button>
                  </>
                )}
              </div>
            ))}
            <div style={{ display: 'flex', gap: '8px', marginTop: '12px', alignItems: 'center' }}>
              <input type="text" className="form-control" placeholder="Item name" value={formNewMenuName}
                onChange={e => setFormNewMenuName(e.target.value)} style={{ flex: 1 }} />
              <input type="number" step="0.01" className="form-control" placeholder="Price" value={formNewMenuPrice}
                onChange={e => setFormNewMenuPrice(e.target.value)} style={{ width: '80px' }} />
              <input type="text" className="form-control" placeholder="Description" value={formNewMenuDesc}
                onChange={e => setFormNewMenuDesc(e.target.value)} style={{ flex: 1 }} />
              <button type="button" className="btn btn-primary" style={{ width: 'auto', padding: '8px 16px', fontSize: '13px' }}
                onClick={async () => {
                  if (!formNewMenuName.trim()) { showMsg('error', 'Item name is required'); return; }
                  try {
                    const r_id = view === 'form-edit' ? selectedId! : null;
                    if (!r_id) {
                      showMsg('error', 'Save the restaurant first, then add menu items');
                      return;
                    }
                    if (editingMenuId != null) {
                      const updated = await api.updateMenuItem(r_id, editingMenuId, {
                        name: formNewMenuName.trim(),
                        price: formNewMenuPrice ? parseFloat(formNewMenuPrice) : undefined,
                        description: formNewMenuDesc.trim() || undefined,
                      });
                      setFormMenuItems((prev: MenuItem[]) => prev.map(mi => mi.id === editingMenuId ? updated : mi));
                      showMsg('success', 'Menu item updated');
                    } else {
                      const newItem = await api.createMenuItem(r_id, {
                        name: formNewMenuName.trim(),
                        price: formNewMenuPrice ? parseFloat(formNewMenuPrice) : undefined,
                        description: formNewMenuDesc.trim() || undefined,
                      });
                      setFormMenuItems((prev: MenuItem[]) => [...prev, newItem]);
                      showMsg('success', 'Menu item added');
                    }
                    setEditingMenuId(null);
                    setFormNewMenuName('');
                    setFormNewMenuPrice('');
                    setFormNewMenuDesc('');
                  } catch (err: any) { showMsg('error', err.message); }
                }}>
                {editingMenuId != null ? 'Update Item' : 'Add Item'}
              </button>
              {editingMenuId != null && (
                <button type="button" className="btn btn-secondary" style={{ width: 'auto', padding: '8px 12px', fontSize: '13px' }}
                  onClick={() => {
                    setEditingMenuId(null);
                    setFormNewMenuName('');
                    setFormNewMenuPrice('');
                    setFormNewMenuDesc('');
                  }}>
                  Cancel
                </button>
              )}
            </div>
          </div>
        </div>

        <div style={{ display: 'flex', gap: '16px', marginTop: '24px' }}>
          <button type="submit" className="btn btn-primary" style={{ width: 'auto', padding: '12px 32px' }}>Save Changes</button>
          <button type="button" onClick={() => setView('dashboard')} className="btn btn-secondary" style={{ width: 'auto', padding: '12px 32px' }}>Cancel</button>
        </div>
      </form>
    </div>
  );
}
