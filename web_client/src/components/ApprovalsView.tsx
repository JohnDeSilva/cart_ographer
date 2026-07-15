import { CheckCircle, Check } from 'lucide-react';
import { Restaurant } from '../api';

interface ApprovalsViewProps {
  approvalRestaurants: Restaurant[];
  handleApproveRestaurant: (id: number) => Promise<void>;
}

export default function ApprovalsView({ approvalRestaurants, handleApproveRestaurant }: ApprovalsViewProps) {
  return (
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
              <div key={r.id} style={{ padding: '16px', borderRadius: '10px', background: 'rgba(255, 255, 255, 0.02)', border: '1px solid var(--panel-border)', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <div>
                  <div style={{ fontWeight: 600, fontSize: '16px', marginBottom: '4px' }}>{r.name}</div>
                  <div style={{ fontSize: '12px', color: 'var(--text-secondary)' }}>{r.restaurant_type} • {r.location.formatted}</div>
                </div>
                <button onClick={() => handleApproveRestaurant(r.id)} className="btn btn-primary" style={{ width: 'auto', padding: '8px 20px', fontSize: '13px' }}>
                  <Check size={14} /> Approve
                </button>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
