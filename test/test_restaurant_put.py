from sqlalchemy import select

from app.sqlalchemy.restaurant import RestaurantType, Restaurant


def test_update_restaurant(db_session, testclient, single_restaurant):
    response = testclient.put("/restaurant", json={"name": single_restaurant.name, "restaurant_type": RestaurantType.BRICK_AND_MORTAR})
    assert response.status_code == 200
    updated_restaurant = db_session.scalar(select(Restaurant).filter(Restaurant.name == single_restaurant.name))
    assert updated_restaurant.restaurant_type == RestaurantType.BRICK_AND_MORTAR

def test_update_restaurant_does_not_exist(testclient):
    response = testclient.put("/restaurant", json={"name": "Not", "restaurant_type": RestaurantType.BRICK_AND_MORTAR})
    assert response.status_code == 404
