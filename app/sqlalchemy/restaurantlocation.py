import uuid
from datetime import datetime

from sqlalchemy import ForeignKey, DateTime
from sqlalchemy.orm import Mapped, mapped_column
from app.sqlalchemy.base import DateFields


class RestaurantLocation(DateFields):
    __tablename__ = "restaurant_location"

    location_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("location.id", ondelete="CASCADE"), primary_key=True)
    restaurant_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("restaurant.id", ondelete="CASCADE"), primary_key=True)
    end_date: Mapped[datetime] = mapped_column(DateTime(timezone=True), nullable=True)
