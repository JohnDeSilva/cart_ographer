from typing import Optional, Annotated
from pydantic import BaseModel, ConfigDict, Field

from app.sqlalchemy.restaurant import RestaurantType, FoodType


class RestaurantInput(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    name: str
    restaurant_type: Annotated[RestaurantType | None, Field(title="Restaurant Type")]
    food_type: Annotated[FoodType | None, Field(title="Food Type")]
