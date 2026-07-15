import { MapPin, PlusCircle, FileEdit, CheckCircle } from 'lucide-react';
import { Restaurant } from '../api';

interface SubmissionsViewProps {
  myRestaurants: Restaurant[];
  openEditForm: (r: Restaurant) => void;
  toggleRestaurantStatus: (r: Restaurant) => Promise<void>;
  openAddForm: () => void;
  navigateToDetail: (id: number) => void;
}

export default function SubmissionsView({ myRestaurants, openEditForm, toggleRestaurantStatus, openAddForm, navigateToDetail }: SubmissionsViewProps) {
  return (
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
            <div key={r.id} style={{ padding: '16px', borderRadius: '10px', background: 'rgba(255, 255, 255, 0.02)', border: '1px solid var(--panel-border)', display: 'flex', justifyContent: 'space-between', alignItems: 'center', cursor: 'pointer', transition: 'all 0.2s ease' }}
              onClick={() => navigateToDetail(r.id)}
              onMouseOver={e => (e.currentTarget.style.borderColor = 'rgba(6, 182, 212, 0.4)')}
              onMouseOut={e => (e.currentTarget.style.borderColor = 'var(--panel-border)')}
            >
              <div>
                <div style={{ fontWeight: 600, fontSize: '16px', marginBottom: '4px' }}>{r.name}</div>
                <div style={{ fontSize: '12px', color: 'var(--text-secondary)', display: 'flex', gap: '8px', alignItems: 'center' }}>
                  <span>{r.restaurant_type}</span>
                  <span>•</span>
                  <span className={`badge ${r.is_approved ? 'badge-open' : 'badge-closed'}`} style={{ fontSize: '10px' }}>{r.is_approved ? 'Approved' : 'Pending'}</span>
                  <span>•</span>
                  <span className={`badge ${r.open_status ? 'badge-open' : 'badge-closed'}`} style={{ fontSize: '10px' }}>{r.open_status ? 'Open' : 'Closed'}</span>
                </div>
              </div>
              <div style={{ display: 'flex', gap: '8px' }} onClick={e => e.stopPropagation()}>
                <button onClick={() => openEditForm(r)} className="btn btn-secondary" style={{ width: 'auto', padding: '6px 12px', fontSize: '12px' }}>
                  <FileEdit size={12} /> Edit
                </button>
                <button onClick={() => toggleRestaurantStatus(r)} className="btn btn-secondary" style={{ width: 'auto', padding: '6px 12px', fontSize: '12px' }}>
                  <CheckCircle size={12} /> Toggle
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
