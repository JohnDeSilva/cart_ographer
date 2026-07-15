import { useEffect, useRef, useState, useCallback } from 'react';
import L from 'leaflet';
import { Restaurant } from './api';
import RestaurantCard from './components/RestaurantCard';

const DEFAULT_LOCATION: [number, number] = [36.1699, -115.1398];

function createTypeIcon(type: string): L.DivIcon {
  const colors: Record<string, string> = {
    'Brick and mortar Restaurant': '#06b6d4',
    'Food Stall': '#f59e0b',
    'Food Truck': '#10b981',
    'Food Cart': '#8b5cf6',
  };
  const labels: Record<string, string> = {
    'Brick and mortar Restaurant': 'B',
    'Food Stall': 'S',
    'Food Truck': 'T',
    'Food Cart': 'C',
  };
  const color = colors[type] || '#6b7280';
  const label = labels[type] || '?';
  return L.divIcon({
    className: '',
    html: `<div style="
      width: 32px; height: 32px; border-radius: 50%;
      background: ${color}; color: white;
      display: flex; align-items: center; justify-content: center;
      font-weight: 700; font-size: 14px; font-family: sans-serif;
      border: 3px solid white; box-shadow: 0 2px 6px rgba(0,0,0,0.3);
      cursor: pointer;
    ">${label}</div>`,
    iconSize: [32, 32],
    iconAnchor: [16, 16],
    popupAnchor: [0, -16],
  });
}

const userIcon = L.divIcon({
  className: '',
  html: `<div style="
    width: 24px; height: 24px; border-radius: 50%;
    background: #3b82f6; border: 3px solid white;
    box-shadow: 0 0 0 4px rgba(59,130,246,0.3), 0 2px 6px rgba(0,0,0,0.3);
  "></div>`,
  iconSize: [24, 24],
  iconAnchor: [12, 12],
});

export default function MapView({ restaurants, onSelectRestaurant }: { restaurants: Restaurant[]; onSelectRestaurant?: (id: number) => void }) {
  const containerRef = useRef<HTMLDivElement>(null);
  const mapRef = useRef<L.Map | null>(null);
  const markersRef = useRef<L.Marker[]>([]);
  const userMarkerRef = useRef<L.Marker | null>(null);
  const [mapReady, setMapReady] = useState(false);
  const [visibleIds, setVisibleIds] = useState<Set<number>>(new Set());
  const [userLocation, setUserLocation] = useState<[number, number] | null>(null);

  const withLocation = restaurants.filter(
    r => r.is_approved && r.location?.lat != null && r.location?.lng != null
  );

  const updateVisible = useCallback(() => {
    const map = mapRef.current;
    if (!map) return;
    const bounds = map.getBounds();
    const ids = new Set(
      withLocation.filter(r => bounds.contains(L.latLng(r.location.lat!, r.location.lng!)))
        .map(r => r.id)
    );
    setVisibleIds(ids);
  }, [withLocation]);

  const updateVisibleRef = useRef(updateVisible);
  updateVisibleRef.current = updateVisible;

  useEffect(() => {
    if (!navigator.geolocation) return;
    navigator.geolocation.getCurrentPosition(
      pos => setUserLocation([pos.coords.latitude, pos.coords.longitude]),
      () => {}
    );
  }, []);

  useEffect(() => {
    if (containerRef.current && !mapRef.current) {
      const map = L.map(containerRef.current, {
        center: DEFAULT_LOCATION,
        zoom: 14,
        zoomControl: true,
      });

      L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>',
      }).addTo(map);

      map.on('moveend', () => updateVisibleRef.current());

      mapRef.current = map;
      setMapReady(true);
    }

    return () => {
      if (mapRef.current) {
        mapRef.current.remove();
        mapRef.current = null;
        setMapReady(false);
      }
    };
  }, []);

  useEffect(() => {
    if (!mapReady) return;
    const map = mapRef.current;
    if (!map) return;

    try {
      markersRef.current.forEach(m => map.removeLayer(m));
      if (userMarkerRef.current) {
        map.removeLayer(userMarkerRef.current);
        userMarkerRef.current = null;
      }

      const loc = userLocation ?? DEFAULT_LOCATION;
      const userMarker = L.marker(loc, { icon: userIcon }).addTo(map);
      userMarker.bindPopup(`
        <strong>You are here</strong><br>
        <span style="font-size:12px;color:#666">${userLocation ? 'Your actual location' : 'Fallback location'}</span>
      `);
      userMarkerRef.current = userMarker;

      markersRef.current = withLocation.map(r =>
        L.marker([r.location.lat!, r.location.lng!], { icon: createTypeIcon(r.restaurant_type) })
          .addTo(map)
          .bindPopup(`
            <strong>${r.name}</strong><br>
            <span style="font-size:12px;color:#666">${r.restaurant_type}${r.cuisine_type ? ` · ${r.cuisine_type}` : ''}</span><br>
            <span style="font-size:12px">${r.location.formatted}</span><br>
            <span style="font-size:12px">${r.open_time} - ${r.close_time}${r.open_status ? ' · Open' : ' · Closed'}</span>
          `)
      );

      if (withLocation.length > 0) {
        const locations = withLocation.map(r => L.latLng(r.location.lat!, r.location.lng!));
        locations.push(L.latLng(loc[0], loc[1]));
        const bounds = L.latLngBounds(locations);
        map.fitBounds(bounds, { padding: [50, 50] });
      }

      updateVisibleRef.current();
    } catch (err) {
      console.error('MapView render error:', err);
    }
  }, [restaurants, mapReady, userLocation]);

  const visibleList = withLocation.filter(r => visibleIds.has(r.id));

  return (
    <div style={{ display: 'flex', gap: '16px', height: 'calc(100vh - 80px)' }}>
      <div className="sidebar-panel glass-panel" style={{ width: '320px', flexShrink: 0, overflow: 'auto', position: 'relative' }}>
        <div style={{ padding: '12px', fontWeight: 600, borderBottom: '1px solid var(--panel-border)', position: 'sticky', top: 0, background: 'var(--panel-bg)', zIndex: 1 }}>
          Restaurants on Map ({visibleList.length})
        </div>
        {visibleList.map(r => (
          <RestaurantCard key={r.id} restaurant={r} variant="map" onClick={onSelectRestaurant} />
        ))}
        {visibleList.length === 0 && mapReady && (
          <div className="empty-state" style={{ padding: '20px' }}>
            <span>No restaurants visible on this map area. Pan or zoom to find vendors.</span>
          </div>
        )}
        {!mapReady && (
          <div style={{ padding: '20px', textAlign: 'center', color: 'var(--text-secondary)' }}>
            Loading map...
          </div>
        )}
      </div>

      <div style={{ flex: 1, minWidth: 0, position: 'relative' }}>
        <div
          ref={containerRef}
          style={{
            position: 'absolute',
            inset: 0,
            borderRadius: '12px',
            border: '1px solid rgba(255,255,255,0.1)',
          }}
        />
      </div>
    </div>
  );
}
