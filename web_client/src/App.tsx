import { AlertCircle } from 'lucide-react';
import { useAppState } from './hooks/useAppState';
import Header from './components/Header';
import AuthForms from './components/AuthForms';
import DashboardView from './components/DashboardView';
import SubmissionsView from './components/SubmissionsView';
import ApprovalsView from './components/ApprovalsView';
import FavoritesView from './components/FavoritesView';
import RestaurantForm from './components/RestaurantForm';
import MapView from './MapView';
import ListView from './components/ListView';

export default function App() {
  const s = useAppState();

  return (
    <div className="container">
      <Header
        authName={s.authName}
        authRole={s.authRole}
        menuOpen={s.menuOpen}
        setMenuOpen={s.setMenuOpen}
        closeMenu={s.closeMenu}
        setSelectedId={s.setSelectedId}
        setView={s.setView}
        handleLogout={s.handleLogout}
        setDetailOnly={s.setDetailOnly}
      />

      {s.statusMsg && (
        <div className={`status-alert ${s.statusMsg.type}`}>
          <AlertCircle size={16} />
          {s.statusMsg.text}
        </div>
      )}

      {s.menuOpen && <div style={{ position: 'fixed', inset: 0, zIndex: 99 }} onClick={() => s.setMenuOpen(false)} />}

      {(s.view === 'login' || s.view === 'signup' || s.view === 'reset') && (
        <AuthForms
          view={s.view}
          username={s.username}
          setUsername={s.setUsername}
          password={s.password}
          setPassword={s.setPassword}
          role={s.role}
          setRole={s.setRole}
          setView={s.setView}
          handleLogin={s.handleLogin}
          handleSignup={s.handleSignup}
          handleResetPassword={s.handleResetPassword}
        />
      )}

      {s.view === 'list' && (
        <ListView restaurants={s.restaurants} navigateToDetail={s.navigateToDetail} />
      )}

      {s.view === 'dashboard' && (
        <DashboardView
          restaurants={s.restaurants}
          setRestaurants={s.setRestaurants}
          searchQuery={s.searchQuery}
          setSearchQuery={s.setSearchQuery}
          selectedId={s.selectedId}
          setSelectedId={s.setSelectedId}
          authRole={s.authRole}
          authName={s.authName}
          selectedRestaurant={s.selectedRestaurant}
          isSelectedFavorited={s.isSelectedFavorited}
          openEditForm={s.openEditForm}
          toggleRestaurantStatus={s.toggleRestaurantStatus}
          handleDeleteRestaurant={s.handleDeleteRestaurant}
          handleToggleFavorite={s.handleToggleFavorite}
          openAddForm={s.openAddForm}
          showMsg={s.showMsg}
          setEditingMenuId={s.setEditingMenuId}
          setFormNewMenuName={s.setFormNewMenuName}
          setFormNewMenuPrice={s.setFormNewMenuPrice}
          setFormNewMenuDesc={s.setFormNewMenuDesc}
          sidebar={!s.detailOnly}
          setDetailOnly={s.setDetailOnly}
          setView={s.setView}
        />
      )}

      {s.view === 'customer-submissions' && (
        <SubmissionsView
          myRestaurants={s.myRestaurants}
          openEditForm={s.openEditForm}
          toggleRestaurantStatus={s.toggleRestaurantStatus}
          openAddForm={s.openAddForm}
          navigateToDetail={s.navigateToDetail}
        />
      )}

      {s.view === 'admin-approvals' && (
        <ApprovalsView
          approvalRestaurants={s.approvalRestaurants}
          handleApproveRestaurant={s.handleApproveRestaurant}
        />
      )}

      {s.view === 'consumer-favorites' && (
        <FavoritesView
          consumerFavorites={s.consumerFavorites}
          handleRemoveFavorite={s.handleRemoveFavorite}
          navigateToDetail={s.navigateToDetail}
        />
      )}

      {s.view === 'map' && <MapView restaurants={s.restaurants} onSelectRestaurant={s.navigateToDetail} />}

      {(s.view === 'form-add' || s.view === 'form-edit') && (
        <RestaurantForm
          view={s.view}
          formName={s.formName}
          setFormName={s.setFormName}
          formType={s.formType}
          setFormType={s.setFormType}
          formCuisineType={s.formCuisineType}
          setFormCuisineType={s.setFormCuisineType}
          formLocation={s.formLocation}
          setFormLocation={s.setFormLocation}
          formOpenTime={s.formOpenTime}
          setFormOpenTime={s.setFormOpenTime}
          formCloseTime={s.formCloseTime}
          setFormCloseTime={s.setFormCloseTime}
          formOpenStatus={s.formOpenStatus}
          setFormOpenStatus={s.setFormOpenStatus}
          formDescription={s.formDescription}
          setFormDescription={s.setFormDescription}
          formMenuItems={s.formMenuItems}
          setFormMenuItems={s.setFormMenuItems}
          formNewMenuName={s.formNewMenuName}
          setFormNewMenuName={s.setFormNewMenuName}
          formNewMenuPrice={s.formNewMenuPrice}
          setFormNewMenuPrice={s.setFormNewMenuPrice}
          formNewMenuDesc={s.formNewMenuDesc}
          setFormNewMenuDesc={s.setFormNewMenuDesc}
          editingMenuId={s.editingMenuId}
          setEditingMenuId={s.setEditingMenuId}
          formLocDescription={s.formLocDescription}
          setFormLocDescription={s.setFormLocDescription}
          formLocLat={s.formLocLat}
          setFormLocLat={s.setFormLocLat}
          formLocLng={s.formLocLng}
          setFormLocLng={s.setFormLocLng}
          formLocAddress={s.formLocAddress}
          setFormLocAddress={s.setFormLocAddress}
          formLocCity={s.formLocCity}
          setFormLocCity={s.setFormLocCity}
          formLocState={s.formLocState}
          setFormLocState={s.setFormLocState}
          formLocZip={s.formLocZip}
          setFormLocZip={s.setFormLocZip}
          formLocRoad1={s.formLocRoad1}
          setFormLocRoad1={s.setFormLocRoad1}
          formLocRoad2={s.formLocRoad2}
          setFormLocRoad2={s.setFormLocRoad2}
          formLocVenue={s.formLocVenue}
          setFormLocVenue={s.setFormLocVenue}
          formLocStall={s.formLocStall}
          setFormLocStall={s.setFormLocStall}
          formLocLot={s.formLocLot}
          setFormLocLot={s.setFormLocLot}
          selectedId={s.selectedId}
          authRole={s.authRole}
          handleSaveRestaurant={s.handleSaveRestaurant}
          setView={s.setView}
          showMsg={s.showMsg}
        />
      )}
    </div>
  );
}
