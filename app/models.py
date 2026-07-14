import enum
from datetime import time
from typing import Optional
from sqlalchemy import String, Time, Boolean, Enum
from sqlalchemy.orm import Mapped, mapped_column
from app.database import Base


class RestaurantType(str, enum.Enum):
    FOOD_STALL = "Food Stall"
    FOOD_TRUCK = "Food Truck"
    BRICK_AND_MORTAR = "Brick and mortar Restaurant"


class UserRole(str, enum.Enum):
    ADMIN = "Admin"
    CUSTOMER = "Customer"


class Restaurant(Base):
    __tablename__ = "restaurants"

    id: Mapped[int] = mapped_column(primary_key=True, index=True)
    name: Mapped[str] = mapped_column(String, index=True, nullable=False)
    restaurant_type: Mapped[RestaurantType] = mapped_column(
        Enum(RestaurantType), nullable=False
    )
    location: Mapped[str] = mapped_column(String, nullable=False)
    open_time: Mapped[time] = mapped_column(Time, nullable=False)
    close_time: Mapped[time] = mapped_column(Time, nullable=False)
    open_status: Mapped[bool] = mapped_column(Boolean, default=False, nullable=False)
    description: Mapped[Optional[str]] = mapped_column(String, nullable=True)


class User(Base):
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(primary_key=True, index=True)
    username: Mapped[str] = mapped_column(
        String, unique=True, index=True, nullable=False
    )
    hashed_password: Mapped[str] = mapped_column(String, nullable=False)
    role: Mapped[UserRole] = mapped_column(
        Enum(UserRole), default=UserRole.CUSTOMER, nullable=False
    )
