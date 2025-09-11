from typing import Annotated
from fastapi import APIRouter, Depends, Body, HTTPException
from sqlalchemy import select
from sqlalchemy.exc import IntegrityError
from sqlalchemy.orm import Session

from app.database import get_db
from app.sqlalchemy.restaurant import Restaurant
from app.pydantic.restaurant import RestaurantInput

router = APIRouter(prefix="/restaurant", tags=["restaurant"], dependencies=[Depends(get_db)])


@router.get("")
async def get_restaurants(database_session: Session = Depends(get_db)):
    restaurants = database_session.scalars(select(Restaurant)).all()
    return [RestaurantInput.model_validate(restaurant) for restaurant in restaurants]

@router.post("")
async def create_restaurant(restaurant_input: Annotated[RestaurantInput, Body(description="new restaurant")], database_session: Session = Depends(get_db)):
    new_restaurant = Restaurant(**restaurant_input.model_dump())
    try:
        database_session.add(new_restaurant)
        database_session.commit()
    except IntegrityError:
        raise HTTPException(status_code=400, detail="restaurant already exists")