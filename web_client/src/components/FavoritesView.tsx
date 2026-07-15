import { Heart, X } from 'lucide-react';
import { FavoriteResponse } from '../api';
import RestaurantCard from './RestaurantCard';

interface FavoritesViewProps {
  consumerFavorites: FavoriteResponse[];
  handleRemoveFavorite: (id: number) => Promise<void>;
  navigateToDetail: (id: number) => void;
}

export default function FavoritesView({ consumerFavorites, handleRemoveFavorite, navigateToDetail }: FavoritesViewProps) {
  return (
    <div className="form-panel glass-panel" style={{ maxWidth: '900px' }}>
      <h2 style={{ margin: 0, border: 'none', padding: 0, marginBottom: '24px' }}>My Favorites</h2>
      {consumerFavorites.length === 0 ? (
        <div className="empty-state">
          <Heart size={48} className="icon" />
          <h2>No Favorites Yet</h2>
          <span>Browse restaurants and add your favorites.</span>
        </div>
      ) : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
          {consumerFavorites.map(fav => (
            <div key={fav.id} style={{ position: 'relative' }}>
              <RestaurantCard
                restaurant={fav.restaurant!}
                variant="list"
                onClick={navigateToDetail}
              />
              <button
                onClick={e => { e.stopPropagation(); handleRemoveFavorite(fav.id); }}
                className="btn btn-danger"
                style={{ position: 'absolute', top: '8px', right: '8px', width: 'auto', padding: '4px 8px', fontSize: '11px' }}
              >
                <X size={12} /> Remove
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
