import uuid

from sqlalchemy import select

from app.sqlalchemy.restaurant import Restaurant


def test_delete_single_restaurant(db_session, testclient, single_restaurant):
    response = testclient.delete(f"/restaurant/{str(single_restaurant.id)}")
    assert response.status_code == 204
    assert None == db_session.scalars(select(Restaurant).where(Restaurant.name == single_restaurant.name)).one_or_none()


def test_delete_single_restaurant_not_found(db_session, testclient):
    response = testclient.delete(f"/restaurant/{uuid.uuid4()}")
    assert response.status_code == 404
