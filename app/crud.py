from datetime import time
from typing import List, Optional
from sqlalchemy import or_, and_
from sqlalchemy.orm import Session
from app import models, schemas
import bcrypt


def hash_password(password: str) -> str:
    return bcrypt.hashpw(password.encode("utf-8"), bcrypt.gensalt()).decode("utf-8")


def verify_password(plain_password: str, hashed_password: str) -> bool:
    return bcrypt.checkpw(
        plain_password.encode("utf-8"), hashed_password.encode("utf-8")
    )


def get_user_by_username(db: Session, username: str) -> Optional[models.User]:
    return db.query(models.User).filter(models.User.username == username).first()


def create_user(db: Session, user: schemas.UserCreate) -> models.User:
    db_user = models.User(
        username=user.username,
        hashed_password=hash_password(user.password),
        role=user.role,
    )
    db.add(db_user)
    db.commit()
    db.refresh(db_user)
    return db_user


def reset_user_password(
    db: Session, db_user: models.User, new_password: str
) -> models.User:
    db_user.hashed_password = hash_password(new_password)
    db.commit()
    db.refresh(db_user)
    return db_user


def create_restaurant(
    db: Session, restaurant: schemas.RestaurantCreate
) -> models.Restaurant:
    db_restaurant = models.Restaurant(**restaurant.model_dump())
    db.add(db_restaurant)
    db.commit()
    db.refresh(db_restaurant)
    return db_restaurant


def get_restaurant(db: Session, restaurant_id: int) -> Optional[models.Restaurant]:
    return (
        db.query(models.Restaurant)
        .filter(models.Restaurant.id == restaurant_id)
        .first()
    )


def get_restaurants(
    db: Session,
    name: Optional[str] = None,
    restaurant_type: Optional[models.RestaurantType] = None,
    open_time: Optional[time] = None,
    close_time: Optional[time] = None,
    open_status: Optional[bool] = None,
    is_open_at: Optional[time] = None,
    skip: int = 0,
    limit: int = 100,
) -> List[models.Restaurant]:
    query = db.query(models.Restaurant)
    if name:
        query = query.filter(models.Restaurant.name.ilike(f"%{name}%"))
    if restaurant_type:
        query = query.filter(models.Restaurant.restaurant_type == restaurant_type)
    if open_time:
        query = query.filter(models.Restaurant.open_time == open_time)
    if close_time:
        query = query.filter(models.Restaurant.close_time == close_time)
    if open_status is not None:
        query = query.filter(models.Restaurant.open_status == open_status)
    if is_open_at:
        # Check if restaurant is open at target time (taking midnight crossing into account)
        # Normal hours: open_time <= close_time, check open_time <= is_open_at <= close_time
        # Midnight crossing: open_time > close_time, check is_open_at >= open_time OR is_open_at <= close_time
        query = query.filter(
            or_(
                and_(
                    models.Restaurant.open_time <= models.Restaurant.close_time,
                    models.Restaurant.open_time <= is_open_at,
                    is_open_at <= models.Restaurant.close_time,
                ),
                and_(
                    models.Restaurant.open_time > models.Restaurant.close_time,
                    or_(
                        is_open_at >= models.Restaurant.open_time,
                        is_open_at <= models.Restaurant.close_time,
                    ),
                ),
            )
        )
    return query.offset(skip).limit(limit).all()


def update_restaurant(
    db: Session,
    db_restaurant: models.Restaurant,
    restaurant_update: schemas.RestaurantUpdate,
) -> models.Restaurant:
    update_data = restaurant_update.model_dump(exclude_unset=True)
    for key, value in update_data.items():
        setattr(db_restaurant, key, value)
    db.commit()
    db.refresh(db_restaurant)
    return db_restaurant


def update_restaurant_status(
    db: Session, db_restaurant: models.Restaurant, open_status: bool
) -> models.Restaurant:
    db_restaurant.open_status = open_status
    db.commit()
    db.refresh(db_restaurant)
    return db_restaurant


def delete_restaurant(db: Session, db_restaurant: models.Restaurant) -> None:
    db.delete(db_restaurant)
    db.commit()
