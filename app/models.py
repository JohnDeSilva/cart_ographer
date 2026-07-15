import enum
from datetime import time
from typing import List, Optional
from sqlalchemy import (
    String,
    Time,
    Boolean,
    Enum,
    Integer,
    Float,
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


class LocationCategory(str, enum.Enum):
    STREET_ADDRESS = "street_address"
    INTERSECTION = "intersection"
    GPS_COORDINATES = "gps_coordinates"
    PARKING_LOT = "parking_lot"
    FOOD_COURT = "food_court"
    OTHER = "other"


class Location(Base):
    __tablename__ = "locations"

    id: Mapped[int] = mapped_column(primary_key=True, index=True)
    location_type: Mapped[LocationCategory] = mapped_column(
        Enum(LocationCategory), nullable=False
    )
    description: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    lat: Mapped[Optional[float]] = mapped_column(Float, nullable=True)
    lng: Mapped[Optional[float]] = mapped_column(Float, nullable=True)
    address: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    city: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    state: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    zip_code: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    road_1: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    road_2: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    venue_name: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    stall_number: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    lot_name: Mapped[Optional[str]] = mapped_column(String, nullable=True)

    @property
    def formatted(self) -> str:
        if self.location_type == LocationCategory.STREET_ADDRESS:
            parts = [p for p in [self.address, self.city, self.state] if p]
            joined = ", ".join(parts)
            if self.zip_code:
                joined += f" {self.zip_code}" if joined else self.zip_code
            return joined
        elif self.location_type == LocationCategory.INTERSECTION:
            return f"{self.road_1} & {self.road_2}" if self.road_1 and self.road_2 else (self.description or "")
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


class Restaurant(Base):
    __tablename__ = "restaurants"

    id: Mapped[int] = mapped_column(primary_key=True, index=True)
    name: Mapped[str] = mapped_column(String, index=True, nullable=False)
    restaurant_type: Mapped[RestaurantType] = mapped_column(
        Enum(RestaurantType), nullable=False
    )
    cuisine_type: Mapped[str] = mapped_column(String, default="", nullable=False)
    open_time: Mapped[time] = mapped_column(Time, nullable=False)
    close_time: Mapped[time] = mapped_column(Time, nullable=False)
    open_status: Mapped[bool] = mapped_column(Boolean, default=False, nullable=False)
    description: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    is_approved: Mapped[bool] = mapped_column(Boolean, default=False, nullable=False)
    owner_id: Mapped[Optional[int]] = mapped_column(
        Integer, ForeignKey("users.id"), nullable=True
    )
    location_id: Mapped[Optional[int]] = mapped_column(
        Integer, ForeignKey("locations.id"), nullable=True
    )
    owner: Mapped[Optional["User"]] = relationship(
        "User", back_populates="owned_restaurants", foreign_keys=[owner_id]
    )
    location: Mapped[Optional["Location"]] = relationship(
        "Location", foreign_keys=[location_id], post_update=True
    )
    menu_items: Mapped[List["MenuItem"]] = relationship(
        "MenuItem", back_populates="restaurant", cascade="all, delete-orphan"
    )


class MenuItem(Base):
    __tablename__ = "menu_items"

    id: Mapped[int] = mapped_column(primary_key=True, index=True)
    restaurant_id: Mapped[int] = mapped_column(
        Integer, ForeignKey("restaurants.id", ondelete="CASCADE"), nullable=False
    )
    name: Mapped[str] = mapped_column(String, nullable=False)
    description: Mapped[Optional[str]] = mapped_column(String, nullable=True)
    price: Mapped[Optional[float]] = mapped_column(Float, nullable=True)
    is_sold_out: Mapped[bool] = mapped_column(Boolean, default=False, nullable=False)
    sort_order: Mapped[int] = mapped_column(Integer, default=0, nullable=False)

    restaurant: Mapped["Restaurant"] = relationship(
        "Restaurant", back_populates="menu_items"
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
