from datetime import time
from typing import Optional
from pydantic import BaseModel, ConfigDict
from app.models import RestaurantType, UserRole


class RestaurantBase(BaseModel):
    name: str
    restaurant_type: RestaurantType
    cuisine_type: str = ""
    location: str
    open_time: time
    close_time: time
    open_status: bool = True
    description: Optional[str] = None
    menu_items: Optional[str] = None


class RestaurantCreate(RestaurantBase):
    pass


class RestaurantSubmit(RestaurantBase):
    pass


class RestaurantUpdate(BaseModel):
    name: Optional[str] = None
    restaurant_type: Optional[RestaurantType] = None
    cuisine_type: Optional[str] = None
    location: Optional[str] = None
    open_time: Optional[time] = None
    close_time: Optional[time] = None
    open_status: Optional[bool] = None
    description: Optional[str] = None
    menu_items: Optional[str] = None


class RestaurantStatusUpdate(BaseModel):
    open_status: bool


class RestaurantApproval(BaseModel):
    is_approved: bool


class LocationApproval(BaseModel):
    approve: bool


class RestaurantResponse(RestaurantBase):
    id: int
    is_approved: bool
    owner_id: Optional[int] = None
    pending_location: Optional[str] = None
    location_change_pending: bool = False

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
