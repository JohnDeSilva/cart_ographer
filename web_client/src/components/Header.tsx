import { Search, MapPin, User, Power, FileEdit, Clock, Menu, Heart, CheckCircle, Sparkles } from 'lucide-react';
import { UserRole } from '../api';

interface HeaderProps {
  authName: string | null;
  authRole: UserRole | null;
  menuOpen: boolean;
  setMenuOpen: (v: boolean) => void;
  closeMenu: () => void;
  setSelectedId: (id: number | null) => void;
  setView: (v: any) => void;
  handleLogout: () => void;
  setDetailOnly?: (v: boolean) => void;
}

export default function Header({ authName, authRole, menuOpen, setMenuOpen, closeMenu, setSelectedId, setView, handleLogout, setDetailOnly }: HeaderProps) {
  return (
    <header className="app-header">
      <div className="logo" onClick={() => { closeMenu(); setSelectedId(null); setView('dashboard'); }} style={{ cursor: 'pointer' }}>
        <Sparkles size={24} /> CART_OGRAPHER
      </div>
      {authName && (
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', position: 'relative' }}>
          <div className="user-badge">
            <User size={14} />
            <span>
              {authName} (<span className="role">{authRole}</span>)
            </span>
          </div>
          <button onClick={() => setMenuOpen(!menuOpen)} className="btn btn-secondary" aria-label="Menu" style={{ padding: '6px 8px', width: 'auto' }}>
            <Menu size={16} />
          </button>
          {menuOpen && (
            <div style={{
              position: 'absolute', top: '100%', right: 0, marginTop: '4px',
              background: 'var(--panel-bg)', border: '1px solid var(--panel-border)',
              borderRadius: '8px', padding: '8px', minWidth: '180px',
              display: 'flex', flexDirection: 'column', gap: '4px', zIndex: 100,
            }}>
              <div className="hamburger-item" onClick={() => { closeMenu(); setSelectedId(null); setDetailOnly?.(false); setView('list'); }}>
                <Search size={14} /> List
              </div>
              {authRole === 'Consumer' && (
                <div className="hamburger-item" onClick={() => { closeMenu(); setSelectedId(null); setView('map'); }}>
                  <MapPin size={14} /> Map
                </div>
              )}
              {authRole === 'Consumer' && (
                <div className="hamburger-item" onClick={() => { closeMenu(); setSelectedId(null); setView('consumer-favorites'); }}>
                  <Heart size={14} /> My Favorites
                </div>
              )}
              {authRole === 'Customer' && (
                <div className="hamburger-item" onClick={() => { closeMenu(); setSelectedId(null); setView('customer-submissions'); }}>
                  <FileEdit size={14} /> My Submissions
                </div>
              )}
              {authRole === 'Admin' && (
                <div className="hamburger-item" onClick={() => { closeMenu(); setSelectedId(null); setView('admin-approvals'); }}>
                  <CheckCircle size={14} /> Approvals
                </div>
              )}
              <div style={{ borderTop: '1px solid var(--panel-border)', margin: '4px 0' }} />
              <div className="hamburger-item" onClick={() => { closeMenu(); setView('reset'); }}>
                <Clock size={14} /> Reset Password
              </div>
              <div className="hamburger-item" onClick={() => { closeMenu(); handleLogout(); }}>
                <Power size={14} /> Logout
              </div>
            </div>
          )}
        </div>
      )}
    </header>
  );
}
