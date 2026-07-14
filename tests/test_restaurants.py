import jwt
from contextlib import contextmanager
from typing import Generator

from fastapi.testclient import TestClient
from sqlalchemy.orm import Session

from app.database import get_db


@contextmanager
def _no_auth_overrides() -> Generator[None, None, None]:
    from app.main import app, get_current_user, get_current_admin

    orig_cu = app.dependency_overrides.pop(get_current_user, None)
    orig_ca = app.dependency_overrides.pop(get_current_admin, None)
    try:
        yield
    finally:
        if orig_cu is not None:
            app.dependency_overrides[get_current_user] = orig_cu
        if orig_ca is not None:
            app.dependency_overrides[get_current_admin] = orig_ca


LOCATION_DEFAULT = {
    "location_type": "street_address",
    "address": "123 Main St",
    "city": "Portland",
    "state": "OR",
}
LOCATION_INTERSECTION = {
    "location_type": "intersection",
    "road_1": "5th Avenue",
    "road_2": "Main Street",
}
LOCATION_STALL = {
    "location_type": "food_court",
    "venue_name": "Market Hall",
    "stall_number": "Stall 3",
}


def test_get_db_coverage() -> None:
    generator = get_db()
    db = next(generator)
    assert isinstance(db, Session)
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
            "location": dict(LOCATION_DEFAULT, address="123 Main St"),
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
    create_resp = client.post(
        "/restaurants",
        json={
            "name": "Taco Stand",
            "restaurant_type": "Food Stall",
            "location": LOCATION_STALL,
            "open_time": "10:00:00",
            "close_time": "18:00:00",
            "open_status": True,
        },
    )
    r_id = create_resp.json()["id"]

    read_resp = client.get(f"/restaurants/{r_id}")
    assert read_resp.status_code == 200
    assert read_resp.json()["name"] == "Taco Stand"

    read_none = client.get("/restaurants/9999")
    assert read_none.status_code == 404
    assert read_none.json()["detail"] == "Restaurant not found"


def test_update_restaurant(client: TestClient) -> None:
    create_resp = client.post(
        "/restaurants",
        json={
            "name": "Pizza Place",
            "restaurant_type": "Brick and mortar Restaurant",
            "location": dict(LOCATION_DEFAULT, address="456 Oak St"),
            "open_time": "11:00:00",
            "close_time": "23:00:00",
            "open_status": True,
        },
    )
    r_id = create_resp.json()["id"]

    update_resp = client.put(
        f"/restaurants/{r_id}",
        json={"name": "Super Pizza Place", "description": "Best pizza"},
    )
    assert update_resp.status_code == 200
    data = update_resp.json()
    assert data["name"] == "Super Pizza Place"
    assert data["description"] == "Best pizza"
    assert data["location"]["formatted"] == "456 Oak St, Portland, OR"

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
            "location": dict(LOCATION_DEFAULT, address="Broad St"),
            "open_time": "06:00:00",
            "close_time": "14:00:00",
            "open_status": True,
        },
    )
    r_id = create_resp.json()["id"]

    patch_resp = client.patch(
        f"/restaurants/{r_id}/status",
        json={"open_status": False},
    )
    assert patch_resp.status_code == 200
    assert patch_resp.json()["open_status"] is False

    patch_resp = client.patch(
        f"/restaurants/{r_id}/status",
        json={"open_status": True},
    )
    assert patch_resp.status_code == 200
    assert patch_resp.json()["open_status"] is True

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
            "location": LOCATION_STALL,
            "open_time": "12:00:00",
            "close_time": "20:00:00",
            "open_status": True,
        },
    )
    r_id = create_resp.json()["id"]

    del_resp = client.delete(f"/restaurants/{r_id}")
    assert del_resp.status_code == 204

    del_none = client.delete(f"/restaurants/{r_id}")
    assert del_none.status_code == 404


def test_filter_restaurants(client: TestClient) -> None:
    client.post(
        "/restaurants",
        json={
            "name": "Downtown Burger Truck",
            "restaurant_type": "Food Truck",
            "location": dict(LOCATION_DEFAULT, address="Downtown"),
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
            "location": dict(LOCATION_DEFAULT, address="West End"),
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
            "location": dict(LOCATION_DEFAULT, address="East Side"),
            "open_time": "12:00:00",
            "close_time": "22:00:00",
            "open_status": True,
        },
    )

    resp = client.get("/restaurants?name=burger")
    assert resp.status_code == 200
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Downtown Burger Truck"

    resp = client.get("/restaurants?name=GRILL")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Midnight Grill Stall"

    resp = client.get("/restaurants?restaurant_type=Food Stall")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Midnight Grill Stall"

    resp = client.get("/restaurants?open_status=false")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Midnight Grill Stall"

    resp = client.get("/restaurants?open_time=12:00:00")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "The Fancy Restaurant"

    resp = client.get("/restaurants?close_time=19:00:00")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Downtown Burger Truck"

    resp = client.get("/restaurants?is_open_at=13:00:00")
    names = [r["name"] for r in resp.json()]
    assert "Downtown Burger Truck" in names
    assert "The Fancy Restaurant" in names
    assert "Midnight Grill Stall" not in names

    resp = client.get("/restaurants?is_open_at=23:30:00")
    names = [r["name"] for r in resp.json()]
    assert "Midnight Grill Stall" in names
    assert "Downtown Burger Truck" not in names
    assert "The Fancy Restaurant" not in names

    resp = client.get("/restaurants?is_open_at=02:00:00")
    names = [r["name"] for r in resp.json()]
    assert "Midnight Grill Stall" in names
    assert "Downtown Burger Truck" not in names

    resp = client.get("/restaurants?is_open_at=05:00:00")
    assert len(resp.json()) == 0


def test_user_authentication_flow(client: TestClient) -> None:
    from app.main import app, get_current_user, get_current_admin

    orig_current_user = app.dependency_overrides.pop(get_current_user, None)
    orig_current_admin = app.dependency_overrides.pop(get_current_admin, None)

    try:
        resp = client.get("/restaurants")
        assert resp.status_code == 401

        signup_resp = client.post(
            "/auth/signup",
            json={"username": "newuser", "password": "newpassword", "role": "Customer"},
        )
        assert signup_resp.status_code == 201
        assert signup_resp.json()["username"] == "newuser"
        assert signup_resp.json()["role"] == "Customer"

        login_resp = client.post(
            "/auth/login",
            json={"username": "newuser", "password": "newpassword"},
        )
        assert login_resp.status_code == 200
        token = login_resp.json()["access_token"]
        assert login_resp.json()["role"] == "Customer"

        headers = {"Authorization": f"Bearer {token}"}
        resp = client.get("/restaurants", headers=headers)
        assert resp.status_code == 200

        create_resp = client.post(
            "/restaurants",
            headers=headers,
            json={
                "name": "Invalid Stand",
                "restaurant_type": "Food Stall",
                "location": LOCATION_STALL,
                "open_time": "10:00:00",
                "close_time": "18:00:00",
            },
        )
        assert create_resp.status_code == 403

        reset_resp = client.post(
            "/auth/reset-password",
            json={"username": "newuser", "new_password": "updatedpassword"},
        )
        assert reset_resp.status_code == 200

        bad_login = client.post(
            "/auth/login",
            json={"username": "newuser", "password": "newpassword"},
        )
        assert bad_login.status_code == 401

        good_login = client.post(
            "/auth/login",
            json={"username": "newuser", "password": "updatedpassword"},
        )
        assert good_login.status_code == 200

    finally:
        if orig_current_user:
            app.dependency_overrides[get_current_user] = orig_current_user
        if orig_current_admin:
            app.dependency_overrides[get_current_admin] = orig_current_admin


def test_default_role_consumer(client: TestClient) -> None:
    with _no_auth_overrides():
        resp = client.post(
            "/auth/signup",
            json={"username": "defrole", "password": "pass"},
        )
        assert resp.status_code == 201
        assert resp.json()["role"] == "Consumer"


def test_signup_duplicate_username(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "dup", "password": "pass1", "role": "Consumer"},
        )
        resp = client.post(
            "/auth/signup",
            json={"username": "dup", "password": "pass2", "role": "Consumer"},
        )
        assert resp.status_code == 400
        assert resp.json()["detail"] == "Username already registered"


def test_reset_password_user_not_found(client: TestClient) -> None:
    with _no_auth_overrides():
        resp = client.post(
            "/auth/reset-password",
            json={"username": "nobody", "new_password": "newpass"},
        )
        assert resp.status_code == 404


def test_get_restaurants_unauthenticated(client: TestClient) -> None:
    with _no_auth_overrides():
        resp = client.get("/restaurants")
        assert resp.status_code == 401


def test_get_restaurants_invalid_token(client: TestClient) -> None:
    with _no_auth_overrides():
        resp = client.get(
            "/restaurants",
            headers={"Authorization": "Bearer invalidtoken"},
        )
        assert resp.status_code == 401


def test_get_current_user_token_without_sub(client: TestClient) -> None:
    token = jwt.encode({"role": "Admin"}, "super-secret-key-for-jwt", algorithm="HS256")
    with _no_auth_overrides():
        resp = client.get(
            "/restaurants",
            headers={"Authorization": f"Bearer {token}"},
        )
        assert resp.status_code == 401


def test_get_current_user_token_user_not_found(client: TestClient) -> None:
    token = jwt.encode({"sub": "nobody"}, "super-secret-key-for-jwt", algorithm="HS256")
    with _no_auth_overrides():
        resp = client.get(
            "/restaurants",
            headers={"Authorization": f"Bearer {token}"},
        )
        assert resp.status_code == 401


def test_customer_submit_restaurant(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "c_sub", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "c_sub", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        resp = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "Submitted Eats",
                "restaurant_type": "Food Stall",
                "cuisine_type": "Thai",
                "location": dict(LOCATION_DEFAULT, address="Downtown"),
                "open_time": "10:00:00",
                "close_time": "22:00:00",
                "menu_items": "Pad Thai, Curry",
            },
        )
        assert resp.status_code == 201
        data = resp.json()
        assert data["name"] == "Submitted Eats"
        assert data["cuisine_type"] == "Thai"
        assert data["menu_items"] == "Pad Thai, Curry"
        assert data["is_approved"] is False
        assert data["owner_id"] is not None
        assert data["location_change_pending"] is False


def test_admin_approve_restaurant(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "app_cust", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "app_cust", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}
        submit = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "Approve Me",
                "restaurant_type": "Food Truck",
                "location": dict(LOCATION_DEFAULT, address="Elm St"),
                "open_time": "08:00:00",
                "close_time": "16:00:00",
            },
        )
        r_id = submit.json()["id"]

    resp = client.patch(f"/restaurants/{r_id}/approve", json={"is_approved": True})
    assert resp.status_code == 200
    assert resp.json()["is_approved"] is True


def test_approve_restaurant_is_approved_false(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "app2", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "app2", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}
        submit = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "No Change",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="Here"),
                "open_time": "09:00:00",
                "close_time": "17:00:00",
            },
        )
        r_id = submit.json()["id"]

    resp = client.patch(f"/restaurants/{r_id}/approve", json={"is_approved": False})
    assert resp.status_code == 200
    assert resp.json()["is_approved"] is False


def test_approve_restaurant_not_found(client: TestClient) -> None:
    resp = client.patch("/restaurants/9999/approve", json={"is_approved": True})
    assert resp.status_code == 404


def test_customer_update_own_restaurant(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "c_upd", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "c_upd", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        sub = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "My Place",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="Somewhere"),
                "open_time": "09:00:00",
                "close_time": "17:00:00",
                "description": "Original",
                "cuisine_type": "American",
            },
        )
        r_id = sub.json()["id"]

        upd = client.put(
            f"/restaurants/{r_id}",
            headers=headers,
            json={"description": "Updated desc", "cuisine_type": "Mexican"},
        )
        assert upd.status_code == 200
        assert upd.json()["description"] == "Updated desc"
        assert upd.json()["cuisine_type"] == "Mexican"
        assert upd.json()["name"] == "My Place"
        assert upd.json()["location"]["formatted"] == "Somewhere, Portland, OR"


def test_customer_update_own_restaurant_name_forbidden(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "c_name", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "c_name", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        sub = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "Original Name",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="Here"),
                "open_time": "10:00:00",
                "close_time": "20:00:00",
            },
        )
        r_id = sub.json()["id"]

        upd = client.put(
            f"/restaurants/{r_id}", headers=headers, json={"name": "New Name"}
        )
        assert upd.status_code == 403
        assert "name" in upd.json()["detail"].lower()


def test_customer_update_other_restaurant_forbidden(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "c_a", "password": "pass", "role": "Customer"},
        )
        login_a = client.post(
            "/auth/login", json={"username": "c_a", "password": "pass"}
        )
        headers_a = {"Authorization": f"Bearer {login_a.json()['access_token']}"}
        sub_a = client.post(
            "/restaurants/submit",
            headers=headers_a,
            json={
                "name": "A's Place",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="Loc A"),
                "open_time": "09:00:00",
                "close_time": "17:00:00",
            },
        )
        r_id_a = sub_a.json()["id"]

        client.post(
            "/auth/signup",
            json={"username": "c_b", "password": "pass", "role": "Customer"},
        )
        login_b = client.post(
            "/auth/login", json={"username": "c_b", "password": "pass"}
        )
        headers_b = {"Authorization": f"Bearer {login_b.json()['access_token']}"}

        upd = client.put(
            f"/restaurants/{r_id_a}", headers=headers_b, json={"description": "Hacked"}
        )
        assert upd.status_code == 403
        assert "own" in upd.json()["detail"].lower()


def test_consumer_cannot_update_restaurant(client: TestClient) -> None:
    create = client.post(
        "/restaurants",
        json={
            "name": "Consumer Target",
            "restaurant_type": "Food Stall",
            "location": dict(LOCATION_DEFAULT, address="Anywhere"),
            "open_time": "09:00:00",
            "close_time": "17:00:00",
        },
    )
    r_id = create.json()["id"]

    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "cons_upd", "password": "pass", "role": "Consumer"},
        )
        login = client.post(
            "/auth/login", json={"username": "cons_upd", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        upd = client.put(
            f"/restaurants/{r_id}", headers=headers, json={"description": "Nope"}
        )
        assert upd.status_code == 403


def test_brick_mortar_location_change_approved(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "bm_own", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "bm_own", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        sub = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "B&M Diner",
                "restaurant_type": "Brick and mortar Restaurant",
                "location": dict(LOCATION_DEFAULT, address="123 Old St"),
                "open_time": "08:00:00",
                "close_time": "22:00:00",
            },
        )
        r_id = sub.json()["id"]

    client.patch(f"/restaurants/{r_id}/approve", json={"is_approved": True})

    with _no_auth_overrides():
        login = client.post(
            "/auth/login", json={"username": "bm_own", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        upd = client.put(
            f"/restaurants/{r_id}",
            headers=headers,
            json={"location": {"address": "456 New St"}, "description": "Moved"},
        )
        assert upd.status_code == 200
        assert upd.json()["location"]["formatted"] == "123 Old St, Portland, OR"
        assert upd.json()["pending_location"]["formatted"] == "456 New St"
        assert upd.json()["location_change_pending"] is True
        assert upd.json()["description"] == "Moved"

    resp = client.patch(f"/restaurants/{r_id}/approve-location", json={"approve": True})
    assert resp.status_code == 200
    assert resp.json()["location"]["formatted"] == "456 New St"
    assert resp.json()["pending_location"] is None
    assert resp.json()["location_change_pending"] is False


def test_brick_mortar_location_change_rejected(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "bm_rej", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "bm_rej", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        sub = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "B&M Reject",
                "restaurant_type": "Brick and mortar Restaurant",
                "location": dict(LOCATION_DEFAULT, address="111 Old St"),
                "open_time": "08:00:00",
                "close_time": "22:00:00",
            },
        )
        r_id = sub.json()["id"]

    client.patch(f"/restaurants/{r_id}/approve", json={"is_approved": True})

    with _no_auth_overrides():
        login = client.post(
            "/auth/login", json={"username": "bm_rej", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        client.put(
            f"/restaurants/{r_id}",
            headers=headers,
            json={"location": {"address": "222 New St"}},
        )

    resp = client.patch(
        f"/restaurants/{r_id}/approve-location", json={"approve": False}
    )
    assert resp.status_code == 200
    assert resp.json()["location"]["formatted"] == "111 Old St, Portland, OR"
    assert resp.json()["pending_location"] is None
    assert resp.json()["location_change_pending"] is False


def test_approve_location_change_not_found(client: TestClient) -> None:
    resp = client.patch("/restaurants/9999/approve-location", json={"approve": True})
    assert resp.status_code == 404


def test_food_truck_update_location_direct(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "ft_own", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "ft_own", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        sub = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "Food Truck",
                "restaurant_type": "Food Truck",
                "location": dict(LOCATION_DEFAULT, address="Corner A"),
                "open_time": "10:00:00",
                "close_time": "18:00:00",
            },
        )
        r_id = sub.json()["id"]

    client.patch(f"/restaurants/{r_id}/approve", json={"is_approved": True})

    with _no_auth_overrides():
        login = client.post(
            "/auth/login", json={"username": "ft_own", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        upd = client.put(
            f"/restaurants/{r_id}", headers=headers, json={"location": {"address": "Corner B"}}
        )
        assert upd.status_code == 200
        assert upd.json()["location"]["formatted"] == "Corner B, Portland, OR"
        assert upd.json()["pending_location"] is None
        assert upd.json()["location_change_pending"] is False


def test_food_cart_update_location_direct(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "fc_own", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "fc_own", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        sub = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "Food Cart",
                "restaurant_type": "Food Cart",
                "location": dict(LOCATION_DEFAULT, address="Park A"),
                "open_time": "11:00:00",
                "close_time": "15:00:00",
            },
        )
        r_id = sub.json()["id"]

    client.patch(f"/restaurants/{r_id}/approve", json={"is_approved": True})

    with _no_auth_overrides():
        login = client.post(
            "/auth/login", json={"username": "fc_own", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        upd = client.put(
            f"/restaurants/{r_id}", headers=headers, json={"location": {"address": "Park B"}}
        )
        assert upd.status_code == 200
        assert upd.json()["location"]["formatted"] == "Park B, Portland, OR"
        assert upd.json()["pending_location"] is None
        assert upd.json()["location_change_pending"] is False


def test_customer_toggle_own_status(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "c_stat", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "c_stat", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        sub = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "Toggle Test",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="Here"),
                "open_time": "09:00:00",
                "close_time": "17:00:00",
                "open_status": True,
            },
        )
        r_id = sub.json()["id"]

        resp = client.patch(
            f"/restaurants/{r_id}/status",
            headers=headers,
            json={"open_status": False},
        )
        assert resp.status_code == 200
        assert resp.json()["open_status"] is False


def test_customer_toggle_other_status_forbidden(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "c_st1", "password": "pass", "role": "Customer"},
        )
        login1 = client.post(
            "/auth/login", json={"username": "c_st1", "password": "pass"}
        )
        headers1 = {"Authorization": f"Bearer {login1.json()['access_token']}"}

        sub = client.post(
            "/restaurants/submit",
            headers=headers1,
            json={
                "name": "Status Target",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="Here"),
                "open_time": "09:00:00",
                "close_time": "17:00:00",
            },
        )
        r_id = sub.json()["id"]

        client.post(
            "/auth/signup",
            json={"username": "c_st2", "password": "pass", "role": "Customer"},
        )
        login2 = client.post(
            "/auth/login", json={"username": "c_st2", "password": "pass"}
        )
        headers2 = {"Authorization": f"Bearer {login2.json()['access_token']}"}

        resp = client.patch(
            f"/restaurants/{r_id}/status",
            headers=headers2,
            json={"open_status": True},
        )
        assert resp.status_code == 403
        assert "own" in resp.json()["detail"].lower()


def test_consumer_cannot_toggle_status(client: TestClient) -> None:
    create = client.post(
        "/restaurants",
        json={
            "name": "Status Test",
            "restaurant_type": "Food Stall",
            "location": dict(LOCATION_DEFAULT, address="Here"),
            "open_time": "09:00:00",
            "close_time": "17:00:00",
        },
    )
    r_id = create.json()["id"]

    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "cs_stat", "password": "pass", "role": "Consumer"},
        )
        login = client.post(
            "/auth/login", json={"username": "cs_stat", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        resp = client.patch(
            f"/restaurants/{r_id}/status",
            headers=headers,
            json={"open_status": True},
        )
        assert resp.status_code == 403


def test_consumer_only_sees_approved_restaurants_list(client: TestClient) -> None:
    client.post(
        "/restaurants",
        json={
            "name": "Approved Only",
            "restaurant_type": "Food Stall",
            "location": dict(LOCATION_DEFAULT, address="Here"),
            "open_time": "09:00:00",
            "close_time": "17:00:00",
        },
    )

    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "vis_cust", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "vis_cust", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}
        client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "Hidden Restaurant",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="There"),
                "open_time": "10:00:00",
                "close_time": "18:00:00",
            },
        )

    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "vis_con", "password": "pass", "role": "Consumer"},
        )
        login = client.post(
            "/auth/login", json={"username": "vis_con", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        resp = client.get("/restaurants", headers=headers)
        assert resp.status_code == 200
        names = [r["name"] for r in resp.json()]
        assert "Approved Only" in names
        assert "Hidden Restaurant" not in names


def test_consumer_only_sees_approved_restaurant_detail(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "det_cust", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "det_cust", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}
        sub = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "Secret Spot",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="Hidden"),
                "open_time": "10:00:00",
                "close_time": "18:00:00",
            },
        )
        r_id = sub.json()["id"]

    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "det_con", "password": "pass", "role": "Consumer"},
        )
        login = client.post(
            "/auth/login", json={"username": "det_con", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        resp = client.get(f"/restaurants/{r_id}", headers=headers)
        assert resp.status_code == 404


def test_consumer_favorites_crud(client: TestClient) -> None:
    rest = client.post(
        "/restaurants",
        json={
            "name": "Fav Restaurant",
            "restaurant_type": "Food Stall",
            "location": dict(LOCATION_DEFAULT, address="Main"),
            "open_time": "09:00:00",
            "close_time": "17:00:00",
        },
    ).json()
    rest_id = rest["id"]

    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "fav_con", "password": "pass", "role": "Consumer"},
        )
        login = client.post(
            "/auth/login", json={"username": "fav_con", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        add = client.post(
            "/favorites", headers=headers, json={"restaurant_id": rest_id}
        )
        assert add.status_code == 201
        fav_id = add.json()["id"]
        assert add.json()["consumer_id"] is not None
        assert add.json()["restaurant_id"] == rest_id

        lst = client.get("/favorites", headers=headers)
        assert lst.status_code == 200
        assert len(lst.json()) == 1
        assert lst.json()[0]["id"] == fav_id
        assert lst.json()[0]["restaurant_id"] == rest_id

        rm = client.delete(f"/favorites/{fav_id}", headers=headers)
        assert rm.status_code == 204

        lst = client.get("/favorites", headers=headers)
        assert len(lst.json()) == 0


def test_consumer_add_favorite_not_approved_restaurant(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "fav_cu", "password": "pass", "role": "Customer"},
        )
        login_cust = client.post(
            "/auth/login", json={"username": "fav_cu", "password": "pass"}
        )
        headers_cust = {"Authorization": f"Bearer {login_cust.json()['access_token']}"}
        sub = client.post(
            "/restaurants/submit",
            headers=headers_cust,
            json={
                "name": "Not Approved",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="Hidden"),
                "open_time": "09:00:00",
                "close_time": "17:00:00",
            },
        )
        r_id = sub.json()["id"]

    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "fav_c2", "password": "pass", "role": "Consumer"},
        )
        login = client.post(
            "/auth/login", json={"username": "fav_c2", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        resp = client.post("/favorites", headers=headers, json={"restaurant_id": r_id})
        assert resp.status_code == 404


def test_consumer_add_favorite_nonexistent_restaurant(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "fav_c3", "password": "pass", "role": "Consumer"},
        )
        login = client.post(
            "/auth/login", json={"username": "fav_c3", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        resp = client.post("/favorites", headers=headers, json={"restaurant_id": 9999})
        assert resp.status_code == 404


def test_consumer_add_favorite_duplicate(client: TestClient) -> None:
    rest = client.post(
        "/restaurants",
        json={
            "name": "Double Fav",
            "restaurant_type": "Food Stall",
            "location": dict(LOCATION_DEFAULT, address="Here"),
            "open_time": "09:00:00",
            "close_time": "17:00:00",
        },
    ).json()

    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "fav_c4", "password": "pass", "role": "Consumer"},
        )
        login = client.post(
            "/auth/login", json={"username": "fav_c4", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        client.post("/favorites", headers=headers, json={"restaurant_id": rest["id"]})
        dup = client.post(
            "/favorites", headers=headers, json={"restaurant_id": rest["id"]}
        )
        assert dup.status_code == 201
        assert dup.json()["restaurant_id"] == rest["id"]


def test_consumer_remove_favorite_not_found(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "rm_con", "password": "pass", "role": "Consumer"},
        )
        login = client.post(
            "/auth/login", json={"username": "rm_con", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        resp = client.delete("/favorites/9999", headers=headers)
        assert resp.status_code == 404


def test_consumer_remove_favorite_not_owner(client: TestClient) -> None:
    rest = client.post(
        "/restaurants",
        json={
            "name": "Shared Restaurant",
            "restaurant_type": "Food Stall",
            "location": dict(LOCATION_DEFAULT, address="Here"),
            "open_time": "09:00:00",
            "close_time": "17:00:00",
        },
    ).json()

    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "fav_a", "password": "pass", "role": "Consumer"},
        )
        login_a = client.post(
            "/auth/login", json={"username": "fav_a", "password": "pass"}
        )
        headers_a = {"Authorization": f"Bearer {login_a.json()['access_token']}"}
        add = client.post(
            "/favorites", headers=headers_a, json={"restaurant_id": rest["id"]}
        )
        fav_id = add.json()["id"]

        client.post(
            "/auth/signup",
            json={"username": "fav_b", "password": "pass", "role": "Consumer"},
        )
        login_b = client.post(
            "/auth/login", json={"username": "fav_b", "password": "pass"}
        )
        headers_b = {"Authorization": f"Bearer {login_b.json()['access_token']}"}

        rm = client.delete(f"/favorites/{fav_id}", headers=headers_b)
        assert rm.status_code == 403
        assert "own" in rm.json()["detail"].lower()


def test_get_restaurants_cuisine_filter(client: TestClient) -> None:
    client.post(
        "/restaurants",
        json={
            "name": "Italian Place",
            "restaurant_type": "Brick and mortar Restaurant",
            "cuisine_type": "Italian",
            "location": dict(LOCATION_DEFAULT, address="Rome Ave"),
            "open_time": "12:00:00",
            "close_time": "22:00:00",
        },
    )
    client.post(
        "/restaurants",
        json={
            "name": "Taco Shop",
            "restaurant_type": "Food Stall",
            "cuisine_type": "Mexican",
            "location": dict(LOCATION_DEFAULT, address="Elm St"),
            "open_time": "10:00:00",
            "close_time": "20:00:00",
        },
    )

    resp = client.get("/restaurants?cuisine_type=Italian")
    assert resp.status_code == 200
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Italian Place"

    resp = client.get("/restaurants?cuisine_type=ita")
    assert len(resp.json()) == 1
    assert resp.json()[0]["cuisine_type"] == "Italian"

    resp = client.get("/restaurants?cuisine_type=American")
    assert len(resp.json()) == 0


def test_get_restaurants_menu_items_filter(client: TestClient) -> None:
    client.post(
        "/restaurants",
        json={
            "name": "Pasta House",
            "restaurant_type": "Brick and mortar Restaurant",
            "cuisine_type": "Italian",
            "location": dict(LOCATION_DEFAULT, address="Main St"),
            "open_time": "12:00:00",
            "close_time": "22:00:00",
            "menu_items": "Spaghetti, Lasagna",
        },
    )
    client.post(
        "/restaurants",
        json={
            "name": "Burger Bar",
            "restaurant_type": "Food Truck",
            "cuisine_type": "American",
            "location": dict(LOCATION_DEFAULT, address="Oak St"),
            "open_time": "11:00:00",
            "close_time": "21:00:00",
            "menu_items": "Burgers, Fries",
        },
    )

    resp = client.get("/restaurants?menu_items=Spaghetti")
    assert resp.status_code == 200
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Pasta House"

    resp = client.get("/restaurants?menu_items=Burger")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Burger Bar"

    resp = client.get("/restaurants?menu_items=Pizza")
    assert len(resp.json()) == 0


def test_get_restaurants_is_approved_filter_admin(client: TestClient) -> None:
    client.post(
        "/restaurants",
        json={
            "name": "Approved Place",
            "restaurant_type": "Food Stall",
            "location": dict(LOCATION_DEFAULT, address="Here"),
            "open_time": "09:00:00",
            "close_time": "17:00:00",
        },
    )

    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "unap", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "unap", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}
        client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "Unapproved Spot",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="There"),
                "open_time": "10:00:00",
                "close_time": "18:00:00",
            },
        )

    resp = client.get("/restaurants?is_approved=true")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Approved Place"

    resp = client.get("/restaurants?is_approved=false")
    assert len(resp.json()) == 1
    assert resp.json()[0]["name"] == "Unapproved Spot"


def test_customer_restaurant_list_sees_own(client: TestClient) -> None:
    client.post(
        "/restaurants",
        json={
            "name": "Approved Place",
            "restaurant_type": "Food Stall",
            "location": dict(LOCATION_DEFAULT, address="Here"),
            "open_time": "09:00:00",
            "close_time": "17:00:00",
        },
    )

    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "ca_cust", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "ca_cust", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "Unapproved Spot",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="There"),
                "open_time": "10:00:00",
                "close_time": "18:00:00",
            },
        )

        resp = client.get("/restaurants", headers=headers)
        assert resp.status_code == 200
        assert len(resp.json()) == 1
        assert resp.json()[0]["name"] == "Unapproved Spot"

        resp = client.get("/restaurants?is_approved=true", headers=headers)
        assert len(resp.json()) == 1
        assert resp.json()[0]["name"] == "Unapproved Spot"


def test_customer_get_my_restaurants(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "myown", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "myown", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "My First",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="A"),
                "open_time": "09:00:00",
                "close_time": "17:00:00",
            },
        )
        client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "My Second",
                "restaurant_type": "Food Truck",
                "location": dict(LOCATION_DEFAULT, address="B"),
                "open_time": "10:00:00",
                "close_time": "18:00:00",
            },
        )

        resp = client.get("/me/restaurants", headers=headers)
        assert resp.status_code == 200
        data = resp.json()
        assert len(data) == 2
        names = [r["name"] for r in data]
        assert "My First" in names
        assert "My Second" in names


def test_admin_can_create_restaurant_with_token(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "newadm", "password": "pass", "role": "Admin"},
        )
        login = client.post(
            "/auth/login", json={"username": "newadm", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        resp = client.post(
            "/restaurants",
            headers=headers,
            json={
                "name": "Admin Created",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="Here"),
                "open_time": "09:00:00",
                "close_time": "17:00:00",
            },
        )
        assert resp.status_code == 201
        assert resp.json()["is_approved"] is True


def test_consumer_endpoint_access_with_customer(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "c_user", "password": "pass", "role": "Customer"},
        )
        login = client.post(
            "/auth/login", json={"username": "c_user", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        resp = client.post("/favorites", headers=headers, json={"restaurant_id": 1})
        assert resp.status_code == 403


def test_customer_endpoint_access_with_consumer(client: TestClient) -> None:
    with _no_auth_overrides():
        client.post(
            "/auth/signup",
            json={"username": "s_user", "password": "pass", "role": "Consumer"},
        )
        login = client.post(
            "/auth/login", json={"username": "s_user", "password": "pass"}
        )
        headers = {"Authorization": f"Bearer {login.json()['access_token']}"}

        resp = client.post(
            "/restaurants/submit",
            headers=headers,
            json={
                "name": "Should Fail",
                "restaurant_type": "Food Stall",
                "location": dict(LOCATION_DEFAULT, address="Nowhere"),
                "open_time": "09:00:00",
                "close_time": "17:00:00",
            },
        )
        assert resp.status_code == 403


def test_admin_create_restaurant_auto_approved(client: TestClient) -> None:
    resp = client.post(
        "/restaurants",
        json={
            "name": "Auto Approved",
            "restaurant_type": "Food Truck",
            "location": dict(LOCATION_DEFAULT, address="Auto St"),
            "open_time": "08:00:00",
            "close_time": "16:00:00",
        },
    )
    assert resp.status_code == 201
    assert resp.json()["is_approved"] is True


def test_food_cart_type(client: TestClient) -> None:
    resp = client.post(
        "/restaurants",
        json={
            "name": "Ice Cream Cart",
            "restaurant_type": "Food Cart",
            "location": dict(LOCATION_DEFAULT, address="Beach"),
            "open_time": "10:00:00",
            "close_time": "18:00:00",
        },
    )
    assert resp.status_code == 201
    data = resp.json()
    assert data["restaurant_type"] == "Food Cart"
    assert data["name"] == "Ice Cream Cart"
