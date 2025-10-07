import pytest
from sqlalchemy import select

from app.sqlalchemy.restaurant import Restaurant, RestaurantType, FoodType


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
