import uuid
from datetime import datetime, timezone
import pytest
from sqlalchemy import select
from app.sqlalchemy.restaurant import Restaurant, RestaurantType, FoodType


@pytest.fixture
def open_time() -> datetime:
    return datetime(2025, 10, 14, 7, 30, tzinfo=timezone.utc)


@pytest.fixture
def close_time() -> datetime:
    return datetime(2025, 10, 14, 18, 00, tzinfo=timezone.utc)


@pytest.fixture
def single_restaurant(db_session, open_time, close_time) -> Restaurant:
    test_restaurant = Restaurant(
        name="Test Restaurant", restaurant_type=RestaurantType.FOOD_CART, food_type=FoodType.FOOD, open_time=open_time, close_time=close_time
    )
    db_session.add(test_restaurant)
    db_session.commit()
    return db_session.scalars(select(Restaurant).where(Restaurant.name == test_restaurant.name)).one()


@pytest.fixture
def multiple_restaurants(db_session) -> list[Restaurant]:
    test_restaurants = [Restaurant(name=f"test{i}", restaurant_type=RestaurantType.FOOD_TRUCK, food_type=FoodType.FOOD) for i in range(4)]
    db_session.add_all(test_restaurants)
    db_session.commit()
    return db_session.scalars(select(Restaurant).order_by(Restaurant.name)).all()


def test_get_restaurants_single_item(db_session, testclient, single_restaurant):
    response = testclient.get("/restaurant")
    assert response.status_code == 200
    assert len(response.json()) == 1
    assert response.json() == [
        {
            "close_time": "2025-10-14T18:00:00",
            "food_type": "food",
            "is_open": False,
            "name": "Test Restaurant",
            "open_time": "2025-10-14T07:30:00",
            "restaurant_type": "food_cart",
        }
    ]


def test_get_restaurants_multiple_items(db_session, testclient, multiple_restaurants):
    response = testclient.get("/restaurant")
    assert response.status_code == 200
    assert len(response.json()) == 4


def test_get_restaurants_none_returned(db_session, testclient):
    response = testclient.get("/restaurant")
    assert response.status_code == 200
    assert len(response.json()) == 0
    assert response.json() == []


def test_post_single_restaurant(db_session, testclient):
    response = testclient.post("/restaurant", json={"name": "test"})
    assert response.status_code == 200
    created_restaurant = db_session.scalar(select(Restaurant).filter(Restaurant.name == "test"))
    assert created_restaurant.name == "test"
    assert created_restaurant.restaurant_type == RestaurantType.BRICK_AND_MORTAR
    assert created_restaurant.food_type == FoodType.FOOD


@pytest.mark.parametrize("r_type", list(RestaurantType))
@pytest.mark.parametrize("f_type", list(FoodType))
def test_post_single_restaurant_type_matrix(r_type, f_type, db_session, testclient):
    response = testclient.post("/restaurant", json={"name": "test", "restaurant_type": r_type, "food_type": f_type})
    assert response.status_code == 200
    created_restaurant = db_session.scalar(select(Restaurant).filter(Restaurant.name == "test"))
    assert created_restaurant.name == "test"
    assert created_restaurant.restaurant_type == r_type
    assert created_restaurant.food_type == f_type


def test_post_single_restaurant_same_name(db_session, testclient):
    response1 = testclient.post("/restaurant", json={"name": "test"})
    assert response1.status_code == 200
    response2 = testclient.post("/restaurant", json={"name": "test"})
    assert response2.status_code == 400


def test_delete_single_restaurant(db_session, testclient, single_restaurant):
    response = testclient.delete(f"/restaurant/{str(single_restaurant.id)}")
    assert response.status_code == 204
    assert None == db_session.scalars(select(Restaurant).where(Restaurant.name == single_restaurant.name)).one_or_none()


def test_delete_single_restaurant_not_found(db_session, testclient):
    response = testclient.delete(f"/restaurant/{uuid.uuid4()}")
    assert response.status_code == 404
