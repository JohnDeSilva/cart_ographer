from typing import Optional, Annotated
from pydantic import BaseModel, ConfigDict, Field

from app.sqlalchemy.restaurant import RestaurantType, FoodType


class RestaurantInput(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    name: str
    restaurant_type: Optional[RestaurantType] = None
    food_type: Optional[FoodType] = None
