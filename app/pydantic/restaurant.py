from datetime import datetime
from typing import Optional, Annotated
from pydantic import BaseModel, ConfigDict, Field

from app.sqlalchemy.restaurant import RestaurantType, FoodType


class RestaurantInput(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    name: str
    restaurant_type: Optional[RestaurantType] = None
    food_type: Optional[FoodType] = None
    open_time: Optional[datetime] = None
    close_time: Optional[datetime] = None


class RestaurantOutput(RestaurantInput):
    model_config = ConfigDict(from_attributes=True)
    is_open: bool
