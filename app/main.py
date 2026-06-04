from datetime import time
from typing import List, Optional
from fastapi import FastAPI, Depends, HTTPException, Query, status
from sqlalchemy.orm import Session
from app import crud, models, schemas
from app.database import engine, get_db, Base

# Create tables
Base.metadata.create_all(bind=engine)

app = FastAPI(title="Restaurant Tracker API", version="0.1.0")

@app.post("/restaurants", response_model=schemas.RestaurantResponse, status_code=status.HTTP_201_CREATED)
def create_restaurant(restaurant: schemas.RestaurantCreate, db: Session = Depends(get_db)) -> models.Restaurant:
    return crud.create_restaurant(db=db, restaurant=restaurant)

@app.get("/restaurants", response_model=List[schemas.RestaurantResponse])
def read_restaurants(
    name: Optional[str] = Query(None, description="Filter by partial restaurant name (case-insensitive)"),
    type: Optional[models.RestaurantType] = Query(None, alias="restaurant_type", description="Filter by restaurant type"),
    open_time: Optional[time] = Query(None, description="Filter by exact open time (HH:MM:SS)"),
    close_time: Optional[time] = Query(None, description="Filter by exact close time (HH:MM:SS)"),
    open_status: Optional[bool] = Query(None, description="Filter by open status"),
    is_open_at: Optional[time] = Query(None, description="Filter by whether restaurant is open at this specific time"),
    skip: int = 0,
    limit: int = 100,
    db: Session = Depends(get_db)
) -> List[models.Restaurant]:
    return crud.get_restaurants(
        db=db,
        name=name,
        restaurant_type=type,
        open_time=open_time,
        close_time=close_time,
        open_status=open_status,
        is_open_at=is_open_at,
        skip=skip,
        limit=limit
    )

@app.get("/restaurants/{restaurant_id}", response_model=schemas.RestaurantResponse)
def read_restaurant(restaurant_id: int, db: Session = Depends(get_db)) -> models.Restaurant:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    return db_restaurant

@app.put("/restaurants/{restaurant_id}", response_model=schemas.RestaurantResponse)
def update_restaurant(
    restaurant_id: int, restaurant_update: schemas.RestaurantUpdate, db: Session = Depends(get_db)
) -> models.Restaurant:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    return crud.update_restaurant(db=db, db_restaurant=db_restaurant, restaurant_update=restaurant_update)

@app.patch("/restaurants/{restaurant_id}/status", response_model=schemas.RestaurantResponse)
def update_restaurant_status(
    restaurant_id: int, status_update: schemas.RestaurantStatusUpdate, db: Session = Depends(get_db)
) -> models.Restaurant:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    return crud.update_restaurant_status(db=db, db_restaurant=db_restaurant, open_status=status_update.open_status)

@app.delete("/restaurants/{restaurant_id}", status_code=status.HTTP_204_NO_CONTENT)
def delete_restaurant(restaurant_id: int, db: Session = Depends(get_db)) -> None:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    crud.delete_restaurant(db=db, db_restaurant=db_restaurant)
