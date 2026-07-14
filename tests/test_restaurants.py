from fastapi.testclient import TestClient
from sqlalchemy.orm import Session
from app.database import get_db


def test_get_db_coverage() -> None:
    # Directly test the get_db generator to ensure coverage
    generator = get_db()
    db = next(generator)
    assert isinstance(db, Session)
    # Trigger finalization/cleanup block
    try:
        next(generator)
    except StopIteration:
        pass


def test_create_restaurant(client: TestClient) -> None:
    response = client.post(
        "/restaurants",
        json={
            "name": "Burger Joint",
            "restaurant_type": "Brick and mortar Restaurant",
            "location": "123 Main St",
            "open_time": "08:00:00",
            "close_time": "22:00:00",
            "open_status": True,
            "description": "Tasty burgers",
        },
    )
    assert response.status_code == 201
    data = response.json()
    assert data["name"] == "Burger Joint"
    assert data["restaurant_type"] == "Brick and mortar Restaurant"
    assert data["id"] is not None


def test_read_restaurant(client: TestClient) -> None:
    # First create
    create_resp = client.post(
        "/restaurants",
        json={
            "name": "Taco Stand",
            "restaurant_type": "Food Stall",
            "location": "Corner of 5th and Elm",
            "open_time": "10:00:00",
            "close_time": "18:00:00",
            "open_status": True,
        },
    )
    r_id = create_resp.json()["id"]

    # Read existing
    read_resp = client.get(f"/restaurants/{r_id}")
    assert read_resp.status_code == 200
    assert read_resp.json()["name"] == "Taco Stand"

    # Read non-existent
    read_none = client.get("/restaurants/9999")
    assert read_none.status_code == 404
    assert read_none.json()["detail"] == "Restaurant not found"


def test_update_restaurant(client: TestClient) -> None:
    create_resp = client.post(
        "/restaurants",
        json={
            "name": "Pizza Place",
            "restaurant_type": "Brick and mortar Restaurant",
            "location": "456 Oak St",
            "open_time": "11:00:00",
            "close_time": "23:00:00",
            "open_status": True,
        },
    )
    r_id = create_resp.json()["id"]

    # Update name and description
    update_resp = client.put(
        f"/restaurants/{r_id}",
        json={"name": "Super Pizza Place", "description": "Best pizza"},
    )
    assert update_resp.status_code == 200
    data = update_resp.json()
    assert data["name"] == "Super Pizza Place"
    assert data["description"] == "Best pizza"
    assert data["location"] == "456 Oak St"  # Unchanged

    # Update non-existent
    update_none = client.put(
        "/restaurants/9999",
        json={"name": "Ghost Restaurant"},
    )
    assert update_none.status_code == 404


def test_patch_restaurant_status(client: TestClient) -> None:
    create_resp = client.post(
        "/restaurants",
        json={
            "name": "Coffee Truck",
            "restaurant_type": "Food Truck",
            "location": "Broad St",
            "open_time": "06:00:00",
            "close_time": "14:00:00",
            "open_status": True,
        },
    )
    r_id = create_resp.json()["id"]

    # Set to closed (False)
    patch_resp = client.patch(
        f"/restaurants/{r_id}/status",
        json={"open_status": False},
    )
    assert patch_resp.status_code == 200
    assert patch_resp.json()["open_status"] is False

    # Set to open (True)
    patch_resp = client.patch(
        f"/restaurants/{r_id}/status",
        json={"open_status": True},
    )
    assert patch_resp.status_code == 200
    assert patch_resp.json()["open_status"] is True

    # Patch non-existent
    patch_none = client.patch(
        "/restaurants/9999/status",
        json={"open_status": False},
    )
    assert patch_none.status_code == 404


def test_delete_restaurant(client: TestClient) -> None:
    create_resp = client.post(
        "/restaurants",
        json={
            "name": "Sushi Cart",
            "restaurant_type": "Food Stall",
            "location": "Market St",
            "open_time": "12:00:00",
            "close_time": "20:00:00",
            "open_status": True,
        },
    )
    r_id = create_resp.json()["id"]

    # Delete existing
    del_resp = client.delete(f"/restaurants/{r_id}")
    assert del_resp.status_code == 204

    # Delete again (should be 404 now)
    del_none = client.delete(f"/restaurants/{r_id}")
    assert del_none.status_code == 404


def test_filter_restaurants(client: TestClient) -> None:
    # Seed data
    client.post(
        "/restaurants",
        json={
            "name": "Downtown Burger Truck",
            "restaurant_type": "Food Truck",
            "location": "Downtown",
            "open_time": "11:00:00",
            "close_time": "19:00:00",
            "open_status": True,
        },
    )
    client.post(
        "/restaurants",
        json={
            "name": "Midnight Grill Stall",
            "restaurant_type": "Food Stall",
            "location": "West End",
            "open_time": "18:00:00",
            "close_time": "03:00:00",
            "open_status": False,
        },
    )
    client.post(
        "/restaurants",
        json={
            "name": "The Fancy Restaurant",
            "restaurant_type": "Brick and mortar Restaurant",
            "location": "East Side",
            "open_time": "12:00:00",
            "close_time": "22:00:00",
            "open_status": True,
        },
    )

    # 1. Filter by Name (Case-insensitive partial match)
    resp = client.get("/restaurants?name=burger")
    assert resp.status_code == 200
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Downtown Burger Truck"

    resp = client.get("/restaurants?name=GRILL")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Midnight Grill Stall"

    # 2. Filter by Type
    resp = client.get("/restaurants?restaurant_type=Food Stall")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Midnight Grill Stall"

    # 3. Filter by Open Status
    resp = client.get("/restaurants?open_status=false")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Midnight Grill Stall"

    # 4. Filter by Exact open/close times
    resp = client.get("/restaurants?open_time=12:00:00")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "The Fancy Restaurant"

    resp = client.get("/restaurants?close_time=19:00:00")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Downtown Burger Truck"

    # 5. Filter by is_open_at (active time)
    # Downtown Burger Truck: 11:00 to 19:00 (Normal)
    # Midnight Grill Stall: 18:00 to 03:00 (Midnight Crossing)
    # The Fancy Restaurant: 12:00 to 22:00 (Normal)

    # Time: 13:00:00 -> Burger Truck and Fancy Restaurant are open
    resp = client.get("/restaurants?is_open_at=13:00:00")
    names = [r["name"] for r in resp.json()]
    assert "Downtown Burger Truck" in names
    assert "The Fancy Restaurant" in names
    assert "Midnight Grill Stall" not in names

    # Time: 23:30:00 -> Midnight Grill Stall is open, others closed
    resp = client.get("/restaurants?is_open_at=23:30:00")
    names = [r["name"] for r in resp.json()]
    assert "Midnight Grill Stall" in names
    assert "Downtown Burger Truck" not in names
    assert "The Fancy Restaurant" not in names

    # Time: 02:00:00 -> Midnight Grill Stall is open, others closed
    resp = client.get("/restaurants?is_open_at=02:00:00")
    names = [r["name"] for r in resp.json()]
    assert "Midnight Grill Stall" in names
    assert "Downtown Burger Truck" not in names

    # Time: 05:00:00 -> All closed
    resp = client.get("/restaurants?is_open_at=05:00:00")
    assert len(resp.json()) == 0


def test_user_authentication_flow(client: TestClient) -> None:
    # Disable dependency override for authentication to test actual auth
    from app.main import app, get_current_user, get_current_admin

    # Temporarily remove dependency overrides for current_user/admin
    orig_current_user = app.dependency_overrides.pop(get_current_user, None)
    orig_current_admin = app.dependency_overrides.pop(get_current_admin, None)

    try:
        # 1. Accessing restaurants without token should fail
        resp = client.get("/restaurants")
        assert resp.status_code == 401

        # 2. Signup new user
        signup_resp = client.post(
            "/auth/signup",
            json={"username": "newuser", "password": "newpassword", "role": "Customer"},
        )
        assert signup_resp.status_code == 201
        assert signup_resp.json()["username"] == "newuser"
        assert signup_resp.json()["role"] == "Customer"

        # 3. Login with credentials
        login_resp = client.post(
            "/auth/login",
            json={"username": "newuser", "password": "newpassword"},
        )
        assert login_resp.status_code == 200
        token = login_resp.json()["access_token"]
        assert login_resp.json()["role"] == "Customer"

        # 4. Access with token
        headers = {"Authorization": f"Bearer {token}"}
        resp = client.get("/restaurants", headers=headers)
        assert resp.status_code == 200

        # 5. Try creating restaurant with Customer token (should fail)
        create_resp = client.post(
            "/restaurants",
            headers=headers,
            json={
                "name": "Invalid Stand",
                "restaurant_type": "Food Stall",
                "location": "Nowhere",
                "open_time": "10:00:00",
                "close_time": "18:00:00",
            },
        )
        assert create_resp.status_code == 403

        # 6. Reset password
        reset_resp = client.post(
            "/auth/reset-password",
            json={"username": "newuser", "new_password": "updatedpassword"},
        )
        assert reset_resp.status_code == 200

        # 7. Login with old password (should fail)
        bad_login = client.post(
            "/auth/login",
            json={"username": "newuser", "password": "newpassword"},
        )
        assert bad_login.status_code == 401

        # 8. Login with new password
        good_login = client.post(
            "/auth/login",
            json={"username": "newuser", "password": "updatedpassword"},
        )
        assert good_login.status_code == 200

    finally:
        # Restore overrides
        if orig_current_user:
            app.dependency_overrides[get_current_user] = orig_current_user
        if orig_current_admin:
            app.dependency_overrides[get_current_admin] = orig_current_admin
