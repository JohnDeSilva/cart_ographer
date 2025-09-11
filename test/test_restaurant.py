from sqlalchemy import select
from app.sqlalchemy.restaurant import Restaurant, RestaurantType, FoodType


def test_get_restaurants_single_item(db_session, testclient):
    db_session.add(Restaurant(name="test", restaurant_type=RestaurantType.FOOD_TRUCK, food_type=FoodType.FOOD))
    db_session.commit()
    response = testclient.get("/restaurant")
    assert response.status_code == 200
    assert len(response.json()) == 1
    assert response.json() == [{'name': 'test', 'restaurant_type': 'food_truck', 'food_type': 'food'}]


def test_get_restaurants_multiple_items(db_session, testclient):
    test_restaurants = [Restaurant(name=f"test{i}", restaurant_type=RestaurantType.FOOD_TRUCK, food_type=FoodType.FOOD) for i in range(4)]
    db_session.add_all(test_restaurants)
    db_session.commit()
    response = testclient.get("/restaurant")
    assert response.status_code == 200
    assert len(response.json()) == 4


def test_get_restaurants_none_returned(db_session, testclient):
    response = testclient.get("/restaurant")
    assert response.status_code == 200
    assert len(response.json()) == 0
    assert response.json() == []

def test_post_restaurant(db_session, testclient):
    response = testclient.post("/restaurant", json={"name": "test", "restaurant_type": None, "food_type": None})
    assert response.status_code == 200
    created_restaurant = db_session.scalar(
        select(Restaurant).filter(Restaurant.name == "test")
    )
    assert created_restaurant.name == "test"
    assert created_restaurant.restaurant_type == RestaurantType.BRICK_AND_MORTAR
    assert created_restaurant.food_type == FoodType.FOOD

def test_post_restaurant_same_name(db_session, testclient):
    response = testclient.post("/restaurant", json={"name": "test", "restaurant_type": None, "food_type": None})
    response = testclient.post("/restaurant", json={"name": "test", "restaurant_type": None, "food_type": None})
    assert response.status_code == 400
