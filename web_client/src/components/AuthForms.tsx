import { UserRole } from '../api';

interface AuthFormsProps {
  view: string;
  username: string;
  setUsername: (v: string) => void;
  password: string;
  setPassword: (v: string) => void;
  role: UserRole;
  setRole: (v: UserRole) => void;
  setView: (v: any) => void;
  handleLogin: (e: React.FormEvent) => Promise<void>;
  handleSignup: (e: React.FormEvent) => Promise<void>;
  handleResetPassword: (e: React.FormEvent) => Promise<void>;
}

export default function AuthForms({ view, username, setUsername, password, setPassword, role, setRole, setView, handleLogin, handleSignup, handleResetPassword }: AuthFormsProps) {
  if (view === 'login') {
    return (
      <div className="auth-wrapper">
        <div className="auth-card glass-panel">
          <h2>Sign In</h2>
          <div className="subtitle">Enter credentials to view tracking route</div>
          <form onSubmit={handleLogin}>
            <div className="form-group">
              <label>Username</label>
              <input type="text" className="form-control" value={username} onChange={e => setUsername(e.target.value)} required placeholder="admin or customer" />
            </div>
            <div className="form-group">
              <label>Password</label>
              <input type="password" className="form-control" value={password} onChange={e => setPassword(e.target.value)} required placeholder="••••••••" />
            </div>
            <button type="submit" className="btn btn-primary">Sign In</button>
          </form>
          <div className="auth-links">
            <a href="#signup" onClick={() => setView('signup')}>Need an account? Sign up</a>
            <a href="#reset" onClick={() => setView('reset')}>Forgot password? Reset</a>
          </div>
        </div>
      </div>
    );
  }

  if (view === 'signup') {
    return (
      <div className="auth-wrapper">
        <div className="auth-card glass-panel">
          <h2>Sign Up</h2>
          <div className="subtitle">Register to discover local vendors</div>
          <form onSubmit={handleSignup}>
            <div className="form-group">
              <label>Username</label>
              <input type="text" className="form-control" value={username} onChange={e => setUsername(e.target.value)} required placeholder="Choose username" />
            </div>
            <div className="form-group">
              <label>Password</label>
              <input type="password" className="form-control" value={password} onChange={e => setPassword(e.target.value)} required placeholder="Choose password" />
            </div>
            <div className="form-group">
              <label>Role</label>
              <select className="form-control" value={role} onChange={e => setRole(e.target.value as UserRole)}>
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
    );
  }

  if (view === 'reset') {
    return (
      <div className="auth-wrapper">
        <div className="auth-card glass-panel">
          <h2>Reset Password</h2>
          <div className="subtitle">Set a new password for account recovery</div>
          <form onSubmit={handleResetPassword}>
            <div className="form-group">
              <label>Target Username</label>
              <input type="text" className="form-control" value={username} onChange={e => setUsername(e.target.value)} required placeholder="Enter username" />
            </div>
            <div className="form-group">
              <label>New Password</label>
              <input type="password" className="form-control" value={password} onChange={e => setPassword(e.target.value)} required placeholder="Enter new password" />
            </div>
            <button type="submit" className="btn btn-primary">Update Password</button>
          </form>
          <div className="auth-links">
            <a href="#login" onClick={() => setView('login')}>Back to sign in</a>
          </div>
        </div>
      </div>
    );
  }

  return null;
}
