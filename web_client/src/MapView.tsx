import { useEffect, useRef, useState } from 'react';
import L from 'leaflet';
import { Restaurant } from './api';

const SIMULATED_USER_LOCATION: [number, number] = [45.5152, -122.6784];

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

export default function MapView({ restaurants }: { restaurants: Restaurant[] }) {
  const containerRef = useRef<HTMLDivElement>(null);
  const mapRef = useRef<L.Map | null>(null);
  const [mapReady, setMapReady] = useState(false);

  useEffect(() => {
    if (containerRef.current && !mapRef.current) {
      const map = L.map(containerRef.current, {
        center: SIMULATED_USER_LOCATION,
        zoom: 14,
        zoomControl: true,
      });

      L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>',
      }).addTo(map);

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
      const markers = restaurants.filter(
        r => r.is_approved && r.location && r.location.lat != null && r.location.lng != null
      );

      const userMarker = L.marker(SIMULATED_USER_LOCATION, { icon: userIcon }).addTo(map);
      userMarker.bindPopup(`
        <strong>You are here</strong><br>
        <span style="font-size:12px;color:#666">Simulated location (Portland, OR)</span>
      `);

      const restaurantMarkers = markers.map(r =>
        L.marker([r.location.lat!, r.location.lng!], { icon: createTypeIcon(r.restaurant_type) })
          .addTo(map)
          .bindPopup(`
            <strong>${r.name}</strong><br>
            <span style="font-size:12px;color:#666">${r.restaurant_type}${r.cuisine_type ? ` · ${r.cuisine_type}` : ''}</span><br>
            <span style="font-size:12px">${r.location.formatted}</span><br>
            <span style="font-size:12px">${r.open_time} - ${r.close_time}${r.open_status ? ' · Open' : ' · Closed'}</span>
          `)
      );

      const locations = markers.map(r => L.latLng(r.location.lat!, r.location.lng!));
      locations.push(L.latLng(SIMULATED_USER_LOCATION[0], SIMULATED_USER_LOCATION[1]));

      if (locations.length > 0) {
        const bounds = L.latLngBounds(locations);
        map.fitBounds(bounds, { padding: [50, 50] });
      }

      return () => {
        map.eachLayer(layer => {
          if (layer instanceof L.Marker) {
            map.removeLayer(layer);
          }
        });
      };
    } catch (err) {
      console.error('MapView render error:', err);
    }
  }, [restaurants, mapReady]);

  const withLocation = restaurants.filter(r => r.location?.lat != null && r.location?.lng != null);

  return (
    <>
      {!mapReady && (
        <div style={{ padding: '40px', textAlign: 'center', color: 'var(--text-secondary)' }}>
          Initializing map...
        </div>
      )}
      <div
        ref={containerRef}
        style={{
          width: '100%',
          minHeight: '500px',
          height: 'calc(100vh - 180px)',
          borderRadius: '12px',
          border: '1px solid rgba(255,255,255,0.1)',
        }}
      />
      {mapReady && withLocation.length === 0 && restaurants.length > 0 && (
        <div style={{ padding: '12px', textAlign: 'center', color: 'var(--text-secondary)', fontSize: '14px' }}>
          {restaurants.length} restaurant(s) loaded, but none have GPS coordinates to pin on the map.
        </div>
      )}
    </>
  );
}
