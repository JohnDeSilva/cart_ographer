import enum
from datetime import time
from typing import Optional
from sqlalchemy import (
    String,
    Time,
    Boolean,
    Enum,
    Integer,
    ForeignKey,
    UniqueConstraint,
)
from sqlalchemy.orm import Mapped, mapped_column, relationship
from app.database import Base


class RestaurantType(str, enum.Enum):
    FOOD_STALL = "Food Stall"
    FOOD_TRUCK = "Food Truck"
    FOOD_CART = "Food Cart"
    BRICK_AND_MORTAR = "Brick and mortar Restaurant"


class UserRole(str, enum.Enum):
    ADMIN = "Admin"
    CUSTOMER = "Customer"
    CONSUMER = "Consumer"


class Restaurant(Base):
    __tablename__ = "restaurants"

    id: Mapped[int] = mapped_column(primary_key=True, index=True)
    name: Mapped[str] = mapped_column(String, index=True, nullable=False)
    restaurant_type: Mapped[RestaurantType] = mapped_column(
        Enum(RestaurantType), nullable=False
    )
    cuisine_type: Mapped[str] = mapped_column(String, default="", nullable=False)
    location: Mapped[str] = mapped_column(String, nullable=False)
    open_time: Mapped[time] = mapped_column(Time, nullable=False)
    close_time: Mapped[time] = mapped_column(Time, nullable=False)
    open_status: Mapped[bool] = mapped_column(Boolean, default=False, nullable=False)
    description: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    menu_items: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    is_approved: Mapped[bool] = mapped_column(Boolean, default=False, nullable=False)
    owner_id: Mapped[Optional[int]] = mapped_column(
        Integer, ForeignKey("users.id"), nullable=True
    )
    pending_location: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    location_change_pending: Mapped[bool] = mapped_column(
        Boolean, default=False, nullable=False
    )

    owner: Mapped[Optional["User"]] = relationship(
        "User", back_populates="owned_restaurants", foreign_keys=[owner_id]
    )


class User(Base):
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(primary_key=True, index=True)
    username: Mapped[str] = mapped_column(
        String, unique=True, index=True, nullable=False
    )
    hashed_password: Mapped[str] = mapped_column(String, nullable=False)
    role: Mapped[UserRole] = mapped_column(
        Enum(UserRole), default=UserRole.CONSUMER, nullable=False
    )

    owned_restaurants: Mapped[list["Restaurant"]] = relationship(
        "Restaurant", back_populates="owner", foreign_keys=[Restaurant.owner_id]
    )
    favorites: Mapped[list["Favorite"]] = relationship(
        "Favorite", back_populates="consumer", foreign_keys="Favorite.consumer_id"
    )


class Favorite(Base):
    __tablename__ = "favorites"

    id: Mapped[int] = mapped_column(primary_key=True, index=True)
    consumer_id: Mapped[int] = mapped_column(
        Integer, ForeignKey("users.id"), nullable=False
    )
    restaurant_id: Mapped[int] = mapped_column(
        Integer, ForeignKey("restaurants.id"), nullable=False
    )

    consumer: Mapped["User"] = relationship(
        "User", back_populates="favorites", foreign_keys=[consumer_id]
    )
    restaurant: Mapped["Restaurant"] = relationship("Restaurant")

    __table_args__ = (
        UniqueConstraint(
            "consumer_id", "restaurant_id", name="unique_consumer_favorite"
        ),
    )
