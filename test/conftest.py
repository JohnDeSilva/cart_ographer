import pytest
from fastapi.testclient import TestClient
from sqlalchemy import create_engine
from sqlalchemy.orm import Session, sessionmaker

from app.cart_ographer import app
from app.database import get_db
from app.sqlalchemy.base import Base


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
