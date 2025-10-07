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
