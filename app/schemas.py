from datetime import time
from typing import List, Optional
from pydantic import BaseModel, ConfigDict, computed_field
from app.models import RestaurantType, UserRole, LocationCategory


class LocationCreate(BaseModel):
    location_type: LocationCategory
    description: Optional[str] = None
    lat: Optional[float] = None
    lng: Optional[float] = None
    address: Optional[str] = None
    city: Optional[str] = None
    state: Optional[str] = None
    zip_code: Optional[str] = None
    road_1: Optional[str] = None
    road_2: Optional[str] = None
    venue_name: Optional[str] = None
    stall_number: Optional[str] = None
    lot_name: Optional[str] = None


class LocationUpdate(BaseModel):
    location_type: Optional[LocationCategory] = None
    description: Optional[str] = None
    lat: Optional[float] = None
    lng: Optional[float] = None
    address: Optional[str] = None
    city: Optional[str] = None
    state: Optional[str] = None
    zip_code: Optional[str] = None
    road_1: Optional[str] = None
    road_2: Optional[str] = None
    venue_name: Optional[str] = None
    stall_number: Optional[str] = None
    lot_name: Optional[str] = None


class LocationResponse(BaseModel):
    id: int
    location_type: LocationCategory
    description: Optional[str] = None
    lat: Optional[float] = None
    lng: Optional[float] = None
    address: Optional[str] = None
    city: Optional[str] = None
    state: Optional[str] = None
    zip_code: Optional[str] = None
    road_1: Optional[str] = None
    road_2: Optional[str] = None
    venue_name: Optional[str] = None
    stall_number: Optional[str] = None
    lot_name: Optional[str] = None

    model_config = ConfigDict(from_attributes=True)

    @computed_field
    @property
    def formatted(self) -> str:
        if self.location_type == LocationCategory.STREET_ADDRESS:
            parts = [p for p in [self.address, self.city, self.state] if p]
            joined = ", ".join(parts)
            if self.zip_code:
                joined += f" {self.zip_code}" if joined else self.zip_code
            return joined
        elif self.location_type == LocationCategory.INTERSECTION:
            if self.road_1 and self.road_2:
                return f"{self.road_1} & {self.road_2}"
            return self.description or ""
        elif self.location_type == LocationCategory.GPS_COORDINATES:
            if self.lat is not None and self.lng is not None:
                return f"{self.lat}, {self.lng}"
            return self.description or ""
        elif self.location_type == LocationCategory.FOOD_COURT:
            parts = [p for p in [self.venue_name, self.stall_number] if p]
            return ", ".join(parts) if parts else (self.description or "")
        elif self.location_type == LocationCategory.PARKING_LOT:
            return self.lot_name or self.description or ""
        return self.description or ""


class MenuItemCreate(BaseModel):
    name: str
    description: Optional[str] = None
    price: Optional[float] = None
    sort_order: int = 0


class MenuItemUpdate(BaseModel):
    name: Optional[str] = None
    description: Optional[str] = None
    price: Optional[float] = None
    is_sold_out: Optional[bool] = None
    sort_order: Optional[int] = None


class MenuItemResponse(BaseModel):
    id: int
    restaurant_id: int
    name: str
    description: Optional[str] = None
    price: Optional[float] = None
    is_sold_out: bool
    sort_order: int

    model_config = ConfigDict(from_attributes=True)


class RestaurantBase(BaseModel):
    name: str
    restaurant_type: RestaurantType
    cuisine_type: str = ""
    open_time: time
    close_time: time
    open_status: bool = True
    description: Optional[str] = None


class RestaurantCreate(RestaurantBase):
    location: LocationCreate


class RestaurantSubmit(RestaurantBase):
    location: LocationCreate


class RestaurantUpdate(BaseModel):
    name: Optional[str] = None
    restaurant_type: Optional[RestaurantType] = None
    cuisine_type: Optional[str] = None
    open_time: Optional[time] = None
    close_time: Optional[time] = None
    open_status: Optional[bool] = None
    description: Optional[str] = None
    location: Optional[LocationUpdate] = None


class RestaurantStatusUpdate(BaseModel):
    open_status: bool


class RestaurantApproval(BaseModel):
    is_approved: bool


class RestaurantResponse(RestaurantBase):
    id: int
    is_approved: bool
    owner_id: Optional[int] = None
    location: Optional[LocationResponse] = None
    menu_items: List[MenuItemResponse] = []

    model_config = ConfigDict(from_attributes=True)


class UserCreate(BaseModel):
    username: str
    password: str
    role: UserRole = UserRole.CONSUMER


class UserResponse(BaseModel):
    id: int
    username: str
    role: UserRole

    model_config = ConfigDict(from_attributes=True)


class UserLogin(BaseModel):
    username: str
    password: str


class UserPasswordReset(BaseModel):
    username: str
    new_password: str


class Token(BaseModel):
    access_token: str
    token_type: str
    role: UserRole
    username: str


class TokenData(BaseModel):
    username: Optional[str] = None
    role: Optional[UserRole] = None


class FavoriteCreate(BaseModel):
    restaurant_id: int


class FavoriteResponse(BaseModel):
    id: int
    consumer_id: int
    restaurant_id: int
    restaurant: Optional[RestaurantResponse] = None

    model_config = ConfigDict(from_attributes=True)
