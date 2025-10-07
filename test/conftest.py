from datetime import datetime, timezone

import pytest
from fastapi.testclient import TestClient
from sqlalchemy import create_engine, select
from sqlalchemy.orm import Session, sessionmaker

from app.cart_ographer import app
from app.database import get_db
from app.sqlalchemy.base import Base
from app.sqlalchemy.restaurant import Restaurant, RestaurantType, FoodType


@pytest.fixture(scope="session")
def db_engine():
    engine = create_engine("sqlite:///test.db", echo=False, future=True, connect_args={"check_same_thread": False})
    Base.metadata.create_all(engine)
    yield engine


@pytest.fixture(scope="function")
def db_session(db_engine):
    connection = db_engine.connect()
    transaction = connection.begin()
    TestSession = sessionmaker(bind=connection)
    session = TestSession()

    yield session
    session.close()
    connection.close()


def override_db_session(db_session: Session):
    yield db_session


@pytest.fixture()
def testclient(db_session: Session):
    app.dependency_overrides[get_db] = lambda: db_session

    with TestClient(app) as client:
        yield client


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
