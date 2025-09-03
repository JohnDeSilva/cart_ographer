from fastapi import APIRouter, Depends
from sqlalchemy import select
from sqlalchemy.orm import Session

from app.database import get_db
from app.sqlalchemy.restaurant import Restaurant
from app.pydantic.restaurant import RestaurantInput

router = APIRouter(prefix="/restaurant", tags=["restaurant"], dependencies=[Depends(get_db)])


@router.get("")
async def get_restaurants(database_session: Session = Depends(get_db)):
    restaurants = database_session.scalars(select(Restaurant)).all()
    return [RestaurantInput.model_validate(restaurant) for restaurant in restaurants]

