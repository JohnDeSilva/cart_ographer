import { Restaurant } from '../api';

interface RestaurantCardProps {
  restaurant: Restaurant;
  variant: 'map' | 'list';
  onClick?: (id: number) => void;
}

export default function RestaurantCard({ restaurant: r, variant, onClick }: RestaurantCardProps) {
  return (
    <div
      className="restaurant-item"
      onClick={() => onClick?.(r.id)}
      style={variant === 'list' ? { padding: '16px' } : undefined}
    >
      <div>
        <div className="restaurant-name">{r.name}</div>
        <div className="restaurant-meta" style={{ display: 'flex', gap: '6px', alignItems: 'center', flexWrap: 'wrap' }}>
          <span>{r.restaurant_type}</span>
          <span className={`badge ${r.open_status ? 'badge-open' : 'badge-closed'}`} style={{ fontSize: '10px' }}>
            {r.open_status ? 'Open' : 'Closed'}
          </span>
        </div>
        <div className="restaurant-meta" style={{ display: 'flex', gap: '6px', alignItems: 'center', justifyContent: 'space-between' }}>
          {r.cuisine_type && (
            <span style={{ fontSize: '11px', color: 'var(--text-secondary)' }}>
              {r.cuisine_type}
            </span>
          )}
          <span style={{ fontSize: '11px', color: 'var(--text-secondary)', fontVariantNumeric: 'tabular-nums' }}>
            {r.open_time} – {r.close_time}
          </span>
        </div>
        {variant === 'list' && (
          <>
            {r.description && (
              <div style={{ fontSize: '12px', color: 'var(--text-secondary)', marginTop: '6px', lineHeight: 1.4 }}>
                {r.description}
              </div>
            )}
            <div style={{ fontSize: '12px', color: 'var(--text-secondary)', marginTop: '4px' }}>
              {r.location.formatted}
            </div>
          </>
        )}
      </div>
    </div>
  );
}
