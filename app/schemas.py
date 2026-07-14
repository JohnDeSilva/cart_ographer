from datetime import time
from typing import Optional
from pydantic import BaseModel, ConfigDict
from app.models import RestaurantType, UserRole


class RestaurantBase(BaseModel):
    name: str
    restaurant_type: RestaurantType
    location: str
    open_time: time
    close_time: time
    open_status: bool = True
    description: Optional[str] = None


class RestaurantCreate(RestaurantBase):
    pass


class RestaurantUpdate(BaseModel):
    name: Optional[str] = None
    restaurant_type: Optional[RestaurantType] = None
    location: Optional[str] = None
    open_time: Optional[time] = None
    close_time: Optional[time] = None
    open_status: Optional[bool] = None
    description: Optional[str] = None


class RestaurantStatusUpdate(BaseModel):
    open_status: bool


class RestaurantResponse(RestaurantBase):
    id: int

    model_config = ConfigDict(from_attributes=True)


class UserCreate(BaseModel):
    username: str
    password: str
    role: UserRole = UserRole.CUSTOMER


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
