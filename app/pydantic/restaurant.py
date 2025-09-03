from pydantic import BaseModel, ConfigDict

from app.sqlalchemy.restaurant import RestaurantType, FoodType


class RestaurantInput(BaseModel):
    model_config = ConfigDict(from_attributes=True)
    name: str
    restaurant_type: RestaurantType
    food_type: FoodType
