import logging
from datetime import time
from typing import List, Optional
from sqlalchemy import or_, and_
from sqlalchemy.orm import Session
from app import models, schemas
import bcrypt

logger = logging.getLogger(__name__)


def hash_password(password: str) -> str:
    return bcrypt.hashpw(password.encode("utf-8"), bcrypt.gensalt()).decode("utf-8")


def verify_password(plain_password: str, hashed_password: str) -> bool:
    return bcrypt.checkpw(
        plain_password.encode("utf-8"), hashed_password.encode("utf-8")
    )


def get_user_by_username(db: Session, username: str) -> Optional[models.User]:
    return db.query(models.User).filter(models.User.username == username).first()


def get_user_by_identifier(db: Session, user_id: int) -> Optional[models.User]:
    return db.query(models.User).filter(models.User.id == user_id).first()


def create_user(db: Session, user: schemas.UserCreate) -> models.User:
    db_user = models.User(
        username=user.username,
        hashed_password=hash_password(user.password),
        role=user.role,
    )
    db.add(db_user)
    db.commit()
    db.refresh(db_user)
    logger.info(
        "User '%s' (id=%d) created with role '%s'",
        db_user.username,
        db_user.id,
        db_user.role,
    )
    return db_user


def reset_user_password(
    db: Session, db_user: models.User, new_password: str
) -> models.User:
    db_user.hashed_password = hash_password(new_password)
    db.commit()
    db.refresh(db_user)
    logger.info("Password reset completed for username: '%s'", db_user.username)
    return db_user


def _create_location(
    db: Session, location_data: schemas.LocationCreate
) -> models.Location:
    db_location = models.Location(**location_data.model_dump())
    db.add(db_location)
    db.flush()
    return db_location


def create_restaurant(
    db: Session, restaurant: schemas.RestaurantCreate
) -> models.Restaurant:
    rest_data = restaurant.model_dump(exclude={"location"})
    db_location = _create_location(db, restaurant.location)
    rest_data["location_id"] = db_location.id
    db_restaurant = models.Restaurant(**rest_data)
    db.add(db_restaurant)
    db.commit()
    db.refresh(db_restaurant)
    logger.info(
        "Restaurant '%s' (id=%d) created by admin",
        db_restaurant.name,
        db_restaurant.id,
    )
    return db_restaurant


def submit_restaurant(
    db: Session, restaurant: schemas.RestaurantSubmit, owner_id: int
) -> models.Restaurant:
    rest_data = restaurant.model_dump(exclude={"location"})
    rest_data["owner_id"] = owner_id
    rest_data["is_approved"] = False
    db_location = _create_location(db, restaurant.location)
    rest_data["location_id"] = db_location.id
    db_restaurant = models.Restaurant(**rest_data)
    db.add(db_restaurant)
    db.commit()
    db.refresh(db_restaurant)
    logger.info(
        "Restaurant '%s' (id=%d) submitted by customer (owner_id=%d)",
        db_restaurant.name,
        db_restaurant.id,
        owner_id,
    )
    return db_restaurant


def get_restaurant(db: Session, restaurant_id: int) -> Optional[models.Restaurant]:
    return (
        db.query(models.Restaurant)
        .filter(models.Restaurant.id == restaurant_id)
        .first()
    )


def get_menu_items(db: Session, restaurant_id: int) -> List[models.MenuItem]:
    return (
        db.query(models.MenuItem)
        .filter(models.MenuItem.restaurant_id == restaurant_id)
        .order_by(models.MenuItem.sort_order)
        .all()
    )


def create_menu_item(
    db: Session, restaurant_id: int, item: schemas.MenuItemCreate
) -> models.MenuItem:
    db_item = models.MenuItem(restaurant_id=restaurant_id, **item.model_dump())
    db.add(db_item)
    db.commit()
    db.refresh(db_item)
    return db_item


def update_menu_item(
    db: Session, db_item: models.MenuItem, item: schemas.MenuItemUpdate
) -> models.MenuItem:
    update_data = item.model_dump(exclude_unset=True)
    for key, value in update_data.items():
        setattr(db_item, key, value)
    db.commit()
    db.refresh(db_item)
    return db_item


def delete_menu_item(db: Session, db_item: models.MenuItem) -> None:
    db.delete(db_item)
    db.commit()


def get_menu_item(db: Session, item_id: int) -> Optional[models.MenuItem]:
    return db.query(models.MenuItem).filter(models.MenuItem.id == item_id).first()


def get_restaurants(
    db: Session,
    name: Optional[str] = None,
    restaurant_type: Optional[models.RestaurantType] = None,
    cuisine_type: Optional[str] = None,
    open_time: Optional[time] = None,
    close_time: Optional[time] = None,
    open_status: Optional[bool] = None,
    is_open_at: Optional[time] = None,
    is_approved: Optional[bool] = None,
    skip: int = 0,
    limit: int = 100,
) -> List[models.Restaurant]:
    query = db.query(models.Restaurant)
    if name:
        query = query.filter(models.Restaurant.name.ilike(f"%{name}%"))
    if restaurant_type:
        query = query.filter(models.Restaurant.restaurant_type == restaurant_type)
    if cuisine_type:
        query = query.filter(models.Restaurant.cuisine_type.ilike(f"%{cuisine_type}%"))
    if open_time:
        query = query.filter(models.Restaurant.open_time == open_time)
    if close_time:
        query = query.filter(models.Restaurant.close_time == close_time)
    if open_status is not None:
        query = query.filter(models.Restaurant.open_status == open_status)
    if is_approved is not None:
        query = query.filter(models.Restaurant.is_approved == is_approved)
    if is_open_at:
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
    logger.debug(
        "Querying restaurants with filters: name=%s, type=%s, cuisine=%s, open_status=%s, approved=%s",
        name,
        restaurant_type,
        cuisine_type,
        open_status,
        is_approved,
    )
    return query.offset(skip).limit(limit).all()


def get_restaurants_by_owner(db: Session, owner_id: int) -> List[models.Restaurant]:
    return (
        db.query(models.Restaurant).filter(models.Restaurant.owner_id == owner_id).all()
    )


def update_restaurant(
    db: Session,
    db_restaurant: models.Restaurant,
    restaurant_update: schemas.RestaurantUpdate,
) -> models.Restaurant:
    update_data = restaurant_update.model_dump(exclude={"location"}, exclude_unset=True)
    changed_fields = list(update_data.keys())
    for key, value in update_data.items():
        setattr(db_restaurant, key, value)
    db.commit()
    db.refresh(db_restaurant)
    logger.info(
        "Restaurant id=%d updated: fields changed=%s",
        db_restaurant.id,
        changed_fields,
    )
    return db_restaurant


def _update_location_in_place(
    db: Session, db_location: models.Location, location_update: schemas.LocationUpdate
) -> models.Location:
    update_data = location_update.model_dump(exclude_unset=True)
    for key, value in update_data.items():
        setattr(db_location, key, value)
    db.flush()
    return db_location


def update_restaurant_status(
    db: Session, db_restaurant: models.Restaurant, open_status: bool
) -> models.Restaurant:
    db_restaurant.open_status = open_status
    db.commit()
    db.refresh(db_restaurant)
    logger.info(
        "Restaurant id=%d status updated to %s",
        db_restaurant.id,
        "open" if open_status else "closed",
    )
    return db_restaurant


def approve_restaurant(
    db: Session, db_restaurant: models.Restaurant
) -> models.Restaurant:
    db_restaurant.is_approved = True
    db.commit()
    db.refresh(db_restaurant)
    logger.info(
        "Restaurant '%s' (id=%d) approved by admin",
        db_restaurant.name,
        db_restaurant.id,
    )
    return db_restaurant


def update_restaurant_location(
    db: Session, db_restaurant: models.Restaurant, location_update: schemas.LocationUpdate
) -> models.Location:
    if not db_restaurant.location:
        db_location = _create_location(db, schemas.LocationCreate(**location_update.model_dump(exclude_unset=True)))
        db_restaurant.location_id = db_location.id
        db.flush()
        return db_location
    return _update_location_in_place(db, db_restaurant.location, location_update)


def delete_restaurant(db: Session, db_restaurant: models.Restaurant) -> None:
    if db_restaurant.location:
        db.delete(db_restaurant.location)
    db.delete(db_restaurant)
    db.commit()
    logger.info("Restaurant id=%d deleted from the database", db_restaurant.id)


def add_favorite(db: Session, consumer_id: int, restaurant_id: int) -> models.Favorite:
    existing = (
        db.query(models.Favorite)
        .filter(
            models.Favorite.consumer_id == consumer_id,
            models.Favorite.restaurant_id == restaurant_id,
        )
        .first()
    )
    if existing:
        logger.debug(
            "Favorite already exists for consumer_id=%d, restaurant_id=%d",
            consumer_id,
            restaurant_id,
        )
        return existing
    db_favorite = models.Favorite(consumer_id=consumer_id, restaurant_id=restaurant_id)
    db.add(db_favorite)
    db.commit()
    db.refresh(db_favorite)
    logger.info(
        "Favorite added: consumer_id=%d, restaurant_id=%d (favorite_id=%d)",
        consumer_id,
        restaurant_id,
        db_favorite.id,
    )
    return db_favorite


def remove_favorite(db: Session, favorite_id: int) -> None:
    db_favorite = (
        db.query(models.Favorite).filter(models.Favorite.id == favorite_id).first()
    )
    if db_favorite:
        db.delete(db_favorite)
        db.commit()
        logger.info("Favorite id=%d removed", favorite_id)


def get_favorite_by_identifier(
    db: Session, favorite_id: int
) -> Optional[models.Favorite]:
    return db.query(models.Favorite).filter(models.Favorite.id == favorite_id).first()


def get_favorites_by_consumer(db: Session, consumer_id: int) -> List[models.Favorite]:
    return (
        db.query(models.Favorite)
        .filter(models.Favorite.consumer_id == consumer_id)
        .all()
    )
