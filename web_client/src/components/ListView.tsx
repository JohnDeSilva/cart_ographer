import { useState } from 'react';
import { Search } from 'lucide-react';
import { Restaurant } from '../api';
import RestaurantCard from './RestaurantCard';

interface ListViewProps {
  restaurants: Restaurant[];
  navigateToDetail: (id: number) => void;
}

export default function ListView({ restaurants, navigateToDetail }: ListViewProps) {
  const [query, setQuery] = useState('');

  const filtered = query
    ? restaurants.filter(r => r.name.toLowerCase().includes(query.toLowerCase()))
    : restaurants;

  return (
    <div className="form-panel glass-panel" style={{ maxWidth: '900px' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
        <h2 style={{ margin: 0, border: 'none', padding: 0 }}>All Restaurants</h2>
        <div className="search-box" style={{ width: '260px' }}>
          <Search size={16} className="icon" />
          <input type="text" className="form-control" placeholder="Search by name..." value={query} onChange={e => setQuery(e.target.value)} />
        </div>
      </div>
      <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        {filtered.map(r => (
          <RestaurantCard key={r.id} restaurant={r} variant="list" onClick={navigateToDetail} />
        ))}
        {filtered.length === 0 && (
          <div className="empty-state" style={{ padding: '40px' }}>
            <Search size={32} className="icon" />
            <span>No restaurants found.</span>
          </div>
        )}
      </div>
    </div>
  );
}
