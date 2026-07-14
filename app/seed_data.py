import logging
from datetime import time
from typing import Dict, List, Tuple

from sqlalchemy.orm import Session

from app import crud, models, schemas

logger = logging.getLogger(__name__)

DEMO_USER_PASSWORD = "password123"


def _ensure_user(
    database_session: Session, username: str, password: str, role: models.UserRole
) -> models.User:
    existing_user = crud.get_user_by_username(database_session, username)
    if existing_user:
        return existing_user
    user = crud.create_user(
        database_session,
        schemas.UserCreate(username=username, password=password, role=role),
    )
    return user


def _make_restaurant_data(
    name: str,
    restaurant_type: models.RestaurantType,
    cuisine_type: str,
    location: str,
    open_time: time,
    close_time: time,
    open_status: bool,
    description: str,
    menu_items: str,
    is_approved: bool,
    owner: models.User,
) -> models.Restaurant:
    restaurant = models.Restaurant(
        name=name,
        restaurant_type=restaurant_type,
        cuisine_type=cuisine_type,
        location=location,
        open_time=open_time,
        close_time=close_time,
        open_status=open_status,
        description=description,
        menu_items=menu_items,
        is_approved=is_approved,
        owner_id=owner.id,
    )
    return restaurant


def _seed_users(database_session: Session) -> Dict[str, models.User]:
    users = {}

    users["admin"] = _ensure_user(
        database_session, "admin", "adminpassword", models.UserRole.ADMIN
    )

    users["consumer"] = _ensure_user(
        database_session, "consumer", "consumerpassword", models.UserRole.CONSUMER
    )

    customer_definitions: List[Tuple[str, str]] = [
        ("cust_cart", "Customer with Food Cart"),
        ("cust_stall", "Customer with Food Stall"),
        ("cust_truck", "Customer with Food Truck"),
        ("cust_bm_truck", "Customer with B&M and Food Truck"),
        ("cust_pending1", "Customer with unapproved restaurant"),
        ("cust_pending2", "Customer with unapproved restaurant"),
    ]
    for username, _ in customer_definitions:
        users[username] = _ensure_user(
            database_session, username, DEMO_USER_PASSWORD, models.UserRole.CUSTOMER
        )

    consumer_definitions: List[Tuple[str, str]] = [
        ("consumer1", "Demo consumer one"),
        ("consumer2", "Demo consumer two"),
        ("consumer3", "Demo consumer three"),
    ]
    for username, _ in consumer_definitions:
        users[username] = _ensure_user(
            database_session, username, DEMO_USER_PASSWORD, models.UserRole.CONSUMER
        )

    return users


def _seed_approved_restaurants(
    database_session: Session, users: Dict[str, models.User]
) -> List[models.Restaurant]:
    restaurants = []

    approved_definitions = [
        (
            "Rolling Bites",
            models.RestaurantType.FOOD_CART,
            "Mexican",
            "Downtown, 5th & Main Street",
            time(11, 0, 0),
            time(20, 0, 0),
            True,
            "Authentic Mexican street food cart serving fresh tacos and burritos.",
            "Tacos al pastor, carnitas burrito, quesadillas, elote, agua fresca",
            users["cust_cart"],
        ),
        (
            "Herb & Grain",
            models.RestaurantType.FOOD_STALL,
            "Vegetarian",
            "West End Market, Stall 7",
            time(8, 0, 0),
            time(16, 0, 0),
            True,
            "Farm-fresh vegetarian bowls and seasonal specials at the West End Market.",
            "Harvest bowl, kale Caesar, sweet potato soup, fresh-squeezed juice",
            users["cust_stall"],
        ),
        (
            "Smoke & Wheels",
            models.RestaurantType.FOOD_TRUCK,
            "Barbecue",
            "Industrial District, 42 Warehouse Row",
            time(17, 0, 0),
            time(2, 0, 0),
            False,
            "Late-night BBQ truck serving slow-smoked meats until 2 AM.",
            "Brisket sandwich, pulled pork plate, smoked ribs, cornbread, coleslaw",
            users["cust_truck"],
        ),
        (
            "Cornerstone Bistro",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Italian",
            "45 Main Street, Suite 100",
            time(12, 0, 0),
            time(22, 0, 0),
            True,
            "Upscale Italian bistro with wood-fired pizzas and homemade pasta.",
            "Margherita pizza, spaghetti carbonara, tiramisu, espresso, wine list",
            users["cust_bm_truck"],
        ),
        (
            "Burger Roll-By",
            models.RestaurantType.FOOD_TRUCK,
            "American",
            "Various - check social media for daily location",
            time(10, 0, 0),
            time(21, 0, 0),
            True,
            "Gourmet burger truck roaming the city with creative weekly specials.",
            "Classic cheeseburger, spicy jalapeno burger, loaded fries, milkshakes",
            users["cust_bm_truck"],
        ),
    ]

    for definition in approved_definitions:
        (
            name,
            restaurant_type,
            cuisine_type,
            location,
            open_time,
            close_time,
            open_status,
            description,
            menu_items,
            owner,
        ) = definition
        restaurant = _make_restaurant_data(
            name=name,
            restaurant_type=restaurant_type,
            cuisine_type=cuisine_type,
            location=location,
            open_time=open_time,
            close_time=close_time,
            open_status=open_status,
            description=description,
            menu_items=menu_items,
            is_approved=True,
            owner=owner,
        )
        database_session.add(restaurant)
        restaurants.append(restaurant)

    return restaurants


def _seed_unapproved_restaurants(
    database_session: Session, users: Dict[str, models.User]
) -> List[models.Restaurant]:
    restaurants = []

    unapproved_definitions = [
        (
            "Taste of Thai",
            models.RestaurantType.FOOD_CART,
            "Thai",
            "Pending location assignment",
            time(17, 0, 0),
            time(23, 0, 0),
            True,
            "New Thai food cart awaiting approval. Offers authentic street food.",
            "Pad Thai, green curry, spring rolls, mango sticky rice",
            users["cust_pending1"],
        ),
        (
            "Sushi Express",
            models.RestaurantType.FOOD_TRUCK,
            "Japanese",
            "Pending location confirmation",
            time(12, 0, 0),
            time(21, 0, 0),
            False,
            "Mobile sushi truck waiting for admin approval before launching.",
            "California roll, spicy tuna roll, nigiri selection, poke bowls",
            users["cust_pending2"],
        ),
    ]

    for definition in unapproved_definitions:
        (
            name,
            restaurant_type,
            cuisine_type,
            location,
            open_time,
            close_time,
            open_status,
            description,
            menu_items,
            owner,
        ) = definition
        restaurant = _make_restaurant_data(
            name=name,
            restaurant_type=restaurant_type,
            cuisine_type=cuisine_type,
            location=location,
            open_time=open_time,
            close_time=close_time,
            open_status=open_status,
            description=description,
            menu_items=menu_items,
            is_approved=False,
            owner=owner,
        )
        database_session.add(restaurant)
        restaurants.append(restaurant)

    return restaurants


def _seed_favorites(
    database_session: Session, users: Dict[str, models.User], restaurants: List[models.Restaurant]
) -> None:
    approved_restaurants = [r for r in restaurants if r.is_approved]
    restaurant_by_name = {r.name: r for r in approved_restaurants}

    favorite_definitions = [
        ("consumer1", "Rolling Bites"),
        ("consumer1", "Smoke & Wheels"),
        ("consumer2", "Cornerstone Bistro"),
    ]

    for consumer_username, restaurant_name in favorite_definitions:
        consumer = users.get(consumer_username)
        restaurant = restaurant_by_name.get(restaurant_name)
        if consumer and restaurant:
            existing = (
                database_session.query(models.Favorite)
                .filter(
                    models.Favorite.consumer_id == consumer.id,
                    models.Favorite.restaurant_id == restaurant.id,
                )
                .first()
            )
            if not existing:
                favorite = models.Favorite(
                    consumer_id=consumer.id, restaurant_id=restaurant.id
                )
                database_session.add(favorite)


def seed_demo_data(database_session: Session) -> None:
    existing_restaurant_count = database_session.query(models.Restaurant).count()
    if existing_restaurant_count > 0:
        logger.info(
            "Demo data already exists (%d restaurants found), skipping seed",
            existing_restaurant_count,
        )
        return

    logger.info("Seeding demo data into the database")

    users = _seed_users(database_session)
    database_session.flush()

    approved_restaurants = _seed_approved_restaurants(database_session, users)
    unapproved_restaurants = _seed_unapproved_restaurants(database_session, users)

    all_restaurants = approved_restaurants + unapproved_restaurants
    database_session.flush()

    _seed_favorites(database_session, users, all_restaurants)

    database_session.commit()

    logger.info("Demo data seeded: %d restaurants, %d favorites", len(all_restaurants), 3)


if __name__ == "__main__":
    import os
    logging.basicConfig(level=logging.INFO)
    os.makedirs("run_dev", exist_ok=True)
    from app.database import engine, Base, SessionLocal

    Base.metadata.create_all(bind=engine)
    database_session = SessionLocal()
    try:
        seed_demo_data(database_session)
    finally:
        database_session.close()
