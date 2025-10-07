import uuid
from enum import StrEnum, auto
from datetime import UTC, datetime
from sqlalchemy import ForeignKey, String, DateTime
from sqlalchemy.orm import Mapped, mapped_column

from app.sqlalchemy.base import CommonColumns
from app.sqlalchemy.location import Location


class RestaurantType(StrEnum):
    BRICK_AND_MORTAR = auto()
    FOOD_TRUCK = auto()
    FOOD_CART = auto()


class FoodType(StrEnum):
    FOOD = auto()
    DRINK = auto()


class Restaurant(CommonColumns):
    __tablename__ = "restaurant"
    name: Mapped[str] = mapped_column(String, unique=True)
    restaurant_type: Mapped[str] = mapped_column(String, nullable=False, default=RestaurantType.BRICK_AND_MORTAR)
    food_type: Mapped[str] = mapped_column(String, nullable=False, default=FoodType.FOOD)
    location_id: Mapped[uuid.UUID] = mapped_column(ForeignKey(Location.id), nullable=True)
    open_time: Mapped[datetime] = mapped_column(DateTime(timezone=True), nullable=True)
    close_time: Mapped[datetime] = mapped_column(DateTime(timezone=True), nullable=True)
    is_open: Mapped[bool] = mapped_column(default=False, nullable=False)
