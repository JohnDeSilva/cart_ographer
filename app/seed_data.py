import logging
from datetime import time
from typing import Dict, List, Tuple

from sqlalchemy.orm import Session

from app import crud, models, schemas

logger = logging.getLogger(__name__)

DEMO_USER_PASSWORD = "password123"


def _ensure_user(
    database_session: Session, username: str, password: str, role: models.UserRole
) -> models.User:
    existing_user = crud.get_user_by_username(database_session, username)
    if existing_user:
        return existing_user
    user = crud.create_user(
        database_session,
        schemas.UserCreate(username=username, password=password, role=role),
    )
    return user


def _make_location(
    database_session: Session,
    location_type: models.LocationCategory,
    address: str = None,
    city: str = None,
    state: str = None,
    zip_code: str = None,
    lat: float = None,
    lng: float = None,
    road_1: str = None,
    road_2: str = None,
    venue_name: str = None,
    stall_number: str = None,
    lot_name: str = None,
    description: str = None,
) -> models.Location:
    location = models.Location(
        location_type=location_type,
        address=address,
        city=city,
        state=state,
        zip_code=zip_code,
        lat=lat,
        lng=lng,
        road_1=road_1,
        road_2=road_2,
        venue_name=venue_name,
        stall_number=stall_number,
        lot_name=lot_name,
        description=description,
    )
    database_session.add(location)
    database_session.flush()
    return location


def _make_restaurant_data(
    name: str,
    restaurant_type: models.RestaurantType,
    cuisine_type: str,
    open_time: time,
    close_time: time,
    open_status: bool,
    description: str,
    is_approved: bool,
    owner: models.User,
    location: models.Location,
) -> models.Restaurant:
    restaurant = models.Restaurant(
        name=name,
        restaurant_type=restaurant_type,
        cuisine_type=cuisine_type,
        location_id=location.id,
        open_time=open_time,
        close_time=close_time,
        open_status=open_status,
        description=description,
        is_approved=is_approved,
        owner_id=owner.id,
    )
    return restaurant


def _make_menu_item(
    database_session: Session,
    restaurant: models.Restaurant,
    name: str,
    price: float,
    description: str = None,
    is_sold_out: bool = False,
    sort_order: int = 0,
) -> models.MenuItem:
    item = models.MenuItem(
        restaurant_id=restaurant.id,
        name=name,
        description=description,
        price=price,
        is_sold_out=is_sold_out,
        sort_order=sort_order,
    )
    database_session.add(item)
    return item


def _seed_users(database_session: Session) -> Dict[str, models.User]:
    users = {}

    users["admin"] = _ensure_user(
        database_session, "admin", "adminpassword", models.UserRole.ADMIN
    )

    users["consumer"] = _ensure_user(
        database_session, "consumer", "consumerpassword", models.UserRole.CONSUMER
    )

    customer_definitions: List[Tuple[str, str]] = [
        ("cust_cart", "Customer with Food Cart"),
        ("cust_stall", "Customer with Food Stall"),
        ("cust_truck", "Customer with Food Truck"),
        ("cust_bm_truck", "Customer with B&M and Food Truck"),
        ("cust_pending1", "Customer with unapproved restaurant"),
        ("cust_pending2", "Customer with unapproved restaurant"),
    ]
    for username, _ in customer_definitions:
        users[username] = _ensure_user(
            database_session, username, DEMO_USER_PASSWORD, models.UserRole.CUSTOMER
        )

    consumer_definitions: List[Tuple[str, str]] = [
        ("consumer1", "Demo consumer one"),
        ("consumer2", "Demo consumer two"),
        ("consumer3", "Demo consumer three"),
    ]
    for username, _ in consumer_definitions:
        users[username] = _ensure_user(
            database_session, username, DEMO_USER_PASSWORD, models.UserRole.CONSUMER
        )

    return users


def _seed_approved_restaurants(
    database_session: Session, users: Dict[str, models.User]
) -> List[models.Restaurant]:
    restaurants = []

    approved_definitions = [
        (
            "D.O.P.E. Chicken",
            models.RestaurantType.FOOD_TRUCK,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="624 S Las Vegas Blvd, Las Vegas, NV 89101",
                lat=36.1627108,
                lng=-115.1451783,
            ),
            time(7, 0, 0),
            time(21, 0, 0),
            False,
            "Comforting American classics made with quality ingredients.",
            users["cust_cart"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Hotdogs el güero",
            models.RestaurantType.FOOD_CART,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="Stewart Ave, Las Vegas, NV 89110",
                lat=36.1661317,
                lng=-115.0852676,
            ),
            time(8, 0, 0),
            time(21, 0, 0),
            False,
            "Comforting American classics made with quality ingredients.",
            users["cust_stall"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "La Reveliada Tacos and Mariscos",
            models.RestaurantType.FOOD_CART,
            "Mexican",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="1845 N Rancho Dr, Las Vegas, NV 89106",
                lat=36.193145699999995,
                lng=-115.1907256,
            ),
            time(8, 0, 0),
            time(20, 0, 0),
            True,
            "Authentic Mexican cuisine made from scratch with fresh ingredients.",
            users["cust_truck"],
            [
                ("Tacos al pastor", 4.5, "Grilled pork tacos with pineapple", False, 1),
                ("Carnitas burrito", 9.0, "Slow-cooked pork burrito with beans and rice", False, 2),
                ("Quesadillas", 6.0, "Cheese quesadilla with your choice of filling", False, 3),
                ("Guacamole & chips", 5.0, "Fresh tableside guacamole", False, 4),
                ("Agua fresca", 3.0, "Fresh fruit water", False, 5),
            ],
        ),
        (
            "Upland kitchen",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="5704 W Charleston Blvd, Las Vegas, NV 89146",
                lat=36.1598635,
                lng=-115.21970999999999,
            ),
            time(10, 0, 0),
            time(21, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_bm_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Judit's Bistro",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="5675 S Rainbow Blvd suite e, Las Vegas, NV 89113",
                lat=36.085698199999996,
                lng=-115.2434725,
            ),
            time(8, 0, 0),
            time(21, 0, 0),
            False,
            "Comforting American classics made with quality ingredients.",
            users["cust_bm_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Gang Alone Creations",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="3400 Western Ave, Las Vegas, NV 89109",
                lat=36.128257999999995,
                lng=-115.1770451,
            ),
            time(11, 0, 0),
            time(21, 0, 0),
            False,
            "Comforting American classics made with quality ingredients.",
            users["cust_cart"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Tacos LaBonita",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Mexican",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="2051 E Sahara Ave, Las Vegas, NV 89104",
                lat=36.1447731,
                lng=-115.1191697,
            ),
            time(10, 0, 0),
            time(0, 0),
            True,
            "Authentic Mexican cuisine made from scratch with fresh ingredients.",
            users["cust_stall"],
            [
                ("Tacos al pastor", 4.5, "Grilled pork tacos with pineapple", False, 1),
                ("Carnitas burrito", 9.0, "Slow-cooked pork burrito with beans and rice", False, 2),
                ("Quesadillas", 6.0, "Cheese quesadilla with your choice of filling", False, 3),
                ("Guacamole & chips", 5.0, "Fresh tableside guacamole", False, 4),
                ("Agua fresca", 3.0, "Fresh fruit water", False, 5),
            ],
        ),
        (
            "Las Vegas Tea & Coffee",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="3575 S Decatur Blvd #102, Las Vegas, NV 89103",
                lat=36.125059199999995,
                lng=-115.2085311,
            ),
            time(11, 0, 0),
            time(23, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Mr. Paella",
            models.RestaurantType.FOOD_TRUCK,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="Located in Las Vegas, NV",
                lat=36.2655174,
                lng=-115.2343695,
            ),
            time(10, 0, 0),
            time(21, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_bm_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Hunan Masala",
            models.RestaurantType.FOOD_TRUCK,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="Located in Las Vegas, NV",
                lat=36.166129,
                lng=-115.14968859999999,
            ),
            time(8, 0, 0),
            time(20, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_bm_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Smoking Las Vegas Barbecue Catering",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="3125 N Michael Way unit b, Las Vegas, NV 89108",
                lat=36.215814699999996,
                lng=-115.21695249999999,
            ),
            time(12, 0, 0),
            time(23, 0, 0),
            False,
            "Comforting American classics made with quality ingredients.",
            users["cust_cart"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Istanbul Mediterranean Restaurant-2 (HALAL)",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Mediterranean",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="505 Fremont St, Las Vegas, NV 89101",
                lat=36.1690226,
                lng=-115.1405334,
            ),
            time(7, 0, 0),
            time(20, 0, 0),
            True,
            "Fresh Mediterranean fare featuring gyros, falafel, and grilled meats.",
            users["cust_stall"],
            [
                ("Gyro platter", 12.0, "Lamb and beef gyro with tzatziki", False, 1),
                ("Falafel wrap", 9.0, "Crispy chickpea falafel in pita", False, 2),
                ("Hummus", 6.0, "Creamy chickpea hummus with olive oil", False, 3),
                ("Greek salad", 8.0, "Feta, olives, and fresh vegetables", False, 4),
                ("Baklava", 5.0, "Layered phyllo with honey and nuts", False, 5),
            ],
        ),
        (
            "Mim Roll Kabob Mediterranean International Meal",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="6475 W Charleston Blvd Ste 160, Las Vegas, NV 89146",
                lat=36.158841599999995,
                lng=-115.2366706,
            ),
            time(12, 0, 0),
            time(23, 0, 0),
            False,
            "Comforting American classics made with quality ingredients.",
            users["cust_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "The Good Life",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="6121 W Lake Mead Blvd Suite 105, Las Vegas, NV 89108",
                lat=36.1944695,
                lng=-115.2245566,
            ),
            time(12, 0, 0),
            time(21, 0, 0),
            False,
            "Comforting American classics made with quality ingredients.",
            users["cust_bm_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Just Chicken Las Vegas",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="3858 W Sahara Ave, Las Vegas, NV 89102",
                lat=36.1458794,
                lng=-115.19197489999999,
            ),
            time(10, 0, 0),
            time(21, 0, 0),
            False,
            "Comforting American classics made with quality ingredients.",
            users["cust_bm_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Poke Express",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Hawaiian",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="4165 S Grand Canyon Dr Suite 103, Las Vegas, NV 89147",
                lat=36.113949999999996,
                lng=-115.3086,
            ),
            time(12, 0, 0),
            time(22, 0, 0),
            True,
            "Fresh island-style poke bowls and Hawaiian comfort food.",
            users["cust_cart"],
            [
                ("Poke bowl", 13.0, "Fresh ahi tuna over rice with toppings", False, 1),
                ("Hawaiian plate", 14.0, "Kalua pork, rice, and mac salad", False, 2),
                ("Spam musubi", 4.0, "Grilled spam on rice wrapped in nori", False, 3),
                ("Poi", 3.0, "Traditional taro root mash", False, 4),
                ("Haupia", 4.0, "Coconut milk dessert", False, 5),
            ],
        ),
        (
            "Original Greek",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Greek",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="4936 E Tropicana Ave, Las Vegas, NV 89121",
                lat=36.1013221,
                lng=-115.06493549999999,
            ),
            time(8, 0, 0),
            time(23, 0, 0),
            False,
            "Authentic Greek dishes with fresh Mediterranean ingredients.",
            users["cust_stall"],
            [
                ("Gyro platter", 12.0, "Lamb and beef gyro with tzatziki", False, 1),
                ("Moussaka", 14.0, "Layered eggplant and meat casserole", False, 2),
                ("Spanakopita", 8.0, "Spinach and feta in phyllo", False, 3),
                ("Greek salad", 8.0, "Feta, olives, and fresh vegetables", False, 4),
                ("Baklava", 5.0, "Layered phyllo with honey and nuts", False, 5),
            ],
        ),
        (
            "Cafe Zupas",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Healthy",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="5095 S Fort Apache Rd, Las Vegas, NV 89148",
                lat=36.0965711,
                lng=-115.2977368,
            ),
            time(11, 0, 0),
            time(21, 0, 0),
            True,
            "Nutritious and delicious meals made with whole food ingredients.",
            users["cust_truck"],
            [
                ("Harvest bowl", 12.0, "Quinoa with roasted vegetables and tahini", False, 1),
                ("Protein wrap", 10.0, "Grilled chicken with greens in a wrap", False, 2),
                ("Smoothie bowl", 9.0, "Acai blended with fresh fruit and granola", False, 3),
                ("Kale smoothie", 6.0, "Kale, banana, and almond milk", False, 4),
                ("Cold brew", 4.0, "Small-batch cold brew coffee", False, 5),
            ],
        ),
        (
            "Brazilian Jungle Grill",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Brazilian",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="603 S Las Vegas Blvd, Las Vegas, NV 89101",
                lat=36.1631672,
                lng=-115.1443415,
            ),
            time(12, 0, 0),
            time(22, 0, 0),
            True,
            "Brazilian-style grilled meats and traditional South American fare.",
            users["cust_bm_truck"],
            [
                ("Picanha", 18.0, "Grilled top sirloin cap", False, 1),
                ("Coxinha", 5.0, "Chicken croquette", False, 2),
                ("Feijoada", 15.0, "Black bean and pork stew", False, 3),
                ("Pao de queijo", 4.0, "Cheese bread rolls", False, 4),
                ("Brigadeiro", 3.0, "Chocolate truffle", False, 5),
            ],
        ),
        (
            "Izakaya Yagiya",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Japanese",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="7250 S Durango Dr Suite 120, Las Vegas, NV 89113",
                lat=36.0566695,
                lng=-115.27843759999999,
            ),
            time(11, 0, 0),
            time(21, 0, 0),
            False,
            "Traditional Japanese cuisine crafted with precision and fresh seafood.",
            users["cust_bm_truck"],
            [
                ("Spicy tuna roll", 9.0, "Tuna with spicy mayo and cucumber", False, 1),
                ("Salmon nigiri", 12.0, "Fresh Atlantic salmon over rice", False, 2),
                ("Edamame", 5.0, "Steamed soybeans with sea salt", False, 3),
                ("Miso soup", 3.5, "Traditional miso with tofu and seaweed", False, 4),
                ("Green tea ice cream", 4.0, "Creamy matcha flavored ice cream", False, 5),
            ],
        ),
        (
            "Sako Taco Man",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Mexican",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="1531 S Las Vegas Blvd, Las Vegas, NV 89104",
                lat=36.152172799999995,
                lng=-115.1514828,
            ),
            time(11, 0, 0),
            time(23, 0, 0),
            True,
            "Authentic Mexican cuisine made from scratch with fresh ingredients.",
            users["cust_cart"],
            [
                ("Tacos al pastor", 4.5, "Grilled pork tacos with pineapple", False, 1),
                ("Carnitas burrito", 9.0, "Slow-cooked pork burrito with beans and rice", False, 2),
                ("Quesadillas", 6.0, "Cheese quesadilla with your choice of filling", False, 3),
                ("Guacamole & chips", 5.0, "Fresh tableside guacamole", False, 4),
                ("Agua fresca", 3.0, "Fresh fruit water", False, 5),
            ],
        ),
        (
            "Empanada Factory Summerlin",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="9320 W Flamingo Rd #2, Las Vegas, NV 89147",
                lat=36.1151859,
                lng=-115.2939912,
            ),
            time(7, 0, 0),
            time(21, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_stall"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Daikon Vegan Sushi - Lake Mead",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Vegan",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="7210 W Lake Mead Blvd #1, Las Vegas, NV 89128",
                lat=36.1967709,
                lng=-115.2491981,
            ),
            time(10, 0, 0),
            time(22, 0, 0),
            False,
            "Creative plant-based dishes that prove vegan food is anything but boring.",
            users["cust_truck"],
            [
                ("Buddha bowl", 13.0, "Quinoa, roasted vegetables, tahini dressing", False, 1),
                ("Vegan burger", 12.0, "Plant-based patty with all the fixings", False, 2),
                ("Kale Caesar", 10.0, "Massaged kale with cashew Caesar dressing", False, 3),
                ("Sweet potato soup", 7.0, "Creamy vegan sweet potato soup", False, 4),
                ("Cold-pressed juice", 6.0, "Seasonal fruit and vegetable juice", False, 5),
            ],
        ),
        (
            "Smokin' Sammich",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="1740 E Serene Ave #100, Las Vegas, NV 89123",
                lat=36.0212499,
                lng=-115.12829049999999,
            ),
            time(7, 0, 0),
            time(22, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_bm_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "AmeriBrunch Cafe",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="316 E Bridger Ave, Las Vegas, NV 89101",
                lat=36.1677517,
                lng=-115.1436612,
            ),
            time(7, 0, 0),
            time(23, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_bm_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Cafe Mong Sahara",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="9151 W Sahara Ave #110, Las Vegas, NV 89117",
                lat=36.144027699999995,
                lng=-115.29723229999999,
            ),
            time(10, 0, 0),
            time(23, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_cart"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "WaBa Grill",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Asian Fusion",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="7060 S Durango Dr, Las Vegas, NV 89113",
                lat=36.0602114,
                lng=-115.2775099,
            ),
            time(7, 0, 0),
            time(21, 0, 0),
            True,
            "Innovative fusion dishes blending flavors from across Asia.",
            users["cust_stall"],
            [
                ("Ramen bowl", 13.0, "Rich tonkotsu broth with chashu pork", False, 1),
                ("Baos", 8.0, "Steamed buns with pork belly", False, 2),
                ("Kimchi fries", 9.0, "Fries with kimchi and spicy mayo", False, 3),
                ("Gyoza", 7.0, "Pan-fried pork dumplings", False, 4),
                ("Matcha latte", 5.0, "Ceremonial grade matcha", False, 5),
            ],
        ),
        (
            "888 Japanese BBQ Encore",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Barbecue",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="8955 S Eastern Ave, Las Vegas, NV 89123",
                lat=36.026544799999996,
                lng=-115.1193132,
            ),
            time(7, 0, 0),
            time(20, 0, 0),
            False,
            "Slow-smoked meats with house-made barbecue sauces.",
            users["cust_truck"],
            [
                ("Brisket sandwich", 12.0, "Slow-smoked brisket on a brioche bun", False, 1),
                ("Pulled pork plate", 11.0, "Pulled pork with two sides", False, 2),
                ("Smoked ribs", 14.0, "Half rack of hickory-smoked ribs", False, 3),
                ("Cornbread", 3.0, "Buttermilk cornbread with honey butter", False, 4),
                ("Coleslaw", 2.5, "Classic creamy coleslaw", False, 5),
            ],
        ),
        (
            "End Goal Meal Prep + Drive Thru | Healthy & Fresh - Pick Up Today!",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Healthy",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="500 E Windmill Ln #175, Las Vegas, NV 89123",
                lat=36.0429806,
                lng=-115.1512798,
            ),
            time(8, 0, 0),
            time(22, 0, 0),
            True,
            "Nutritious and delicious meals made with whole food ingredients.",
            users["cust_bm_truck"],
            [
                ("Harvest bowl", 12.0, "Quinoa with roasted vegetables and tahini", False, 1),
                ("Protein wrap", 10.0, "Grilled chicken with greens in a wrap", False, 2),
                ("Smoothie bowl", 9.0, "Acai blended with fresh fruit and granola", False, 3),
                ("Kale smoothie", 6.0, "Kale, banana, and almond milk", False, 4),
                ("Cold brew", 4.0, "Small-batch cold brew coffee", False, 5),
            ],
        ),
        (
            "Rutba Indian Kitchen",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Indian",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="5115 Spring Mountain Rd #223A, Las Vegas, NV 89146",
                lat=36.1255844,
                lng=-115.2117535,
            ),
            time(8, 0, 0),
            time(20, 0, 0),
            False,
            "Aromatic Indian curries and tandoori specialties made with traditional spices.",
            users["cust_bm_truck"],
            [
                ("Chicken tikka masala", 14.0, "Creamy tomato-spiced curry", False, 1),
                ("Garlic naan", 4.0, "Tandoor-baked garlic flatbread", False, 2),
                ("Biryani", 12.0, "Fragrant spiced rice with vegetables", False, 3),
                ("Samosas", 6.0, "Crispy pastry filled with spiced potatoes", False, 4),
                ("Mango lassi", 4.0, "Creamy mango yogurt drink", False, 5),
            ],
        ),
        (
            "Novella Italian Kitchen , Pizza",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Italian",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="6825 W Russell Rd suite 115, Las Vegas, NV 89118",
                lat=36.0845571,
                lng=-115.2410096,
            ),
            time(8, 0, 0),
            time(22, 0, 0),
            True,
            "Traditional Italian dishes prepared with imported ingredients and house-made pasta.",
            users["cust_cart"],
            [
                ("Margherita pizza", 14.0, "Wood-fired with fresh mozzarella and basil", False, 1),
                ("Spaghetti carbonara", 16.0, "House-made pasta with pancetta and egg", False, 2),
                ("Caesar salad", 10.0, "Romaine with house-made dressing and croutons", False, 3),
                ("Tiramisu", 8.0, "Classic Italian coffee dessert", False, 4),
                ("Espresso", 3.0, "Single-origin espresso shot", False, 5),
            ],
        ),
        (
            "Manizza's Pizza",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Italian",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="6090 S Rainbow Blvd #2, Las Vegas, NV 89118",
                lat=36.0780835,
                lng=-115.2420377,
            ),
            time(12, 0, 0),
            time(23, 0, 0),
            True,
            "Traditional Italian dishes prepared with imported ingredients and house-made pasta.",
            users["cust_stall"],
            [
                ("Margherita pizza", 14.0, "Wood-fired with fresh mozzarella and basil", False, 1),
                ("Spaghetti carbonara", 16.0, "House-made pasta with pancetta and egg", False, 2),
                ("Caesar salad", 10.0, "Romaine with house-made dressing and croutons", False, 3),
                ("Tiramisu", 8.0, "Classic Italian coffee dessert", False, 4),
                ("Espresso", 3.0, "Single-origin espresso shot", False, 5),
            ],
        ),
        (
            "On The Run Pizza",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Italian",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="333 W St Louis Ave, Las Vegas, NV 89102",
                lat=36.1480919,
                lng=-115.161256,
            ),
            time(8, 0, 0),
            time(21, 0, 0),
            True,
            "Traditional Italian dishes prepared with imported ingredients and house-made pasta.",
            users["cust_truck"],
            [
                ("Margherita pizza", 14.0, "Wood-fired with fresh mozzarella and basil", False, 1),
                ("Spaghetti carbonara", 16.0, "House-made pasta with pancetta and egg", False, 2),
                ("Caesar salad", 10.0, "Romaine with house-made dressing and croutons", False, 3),
                ("Tiramisu", 8.0, "Classic Italian coffee dessert", False, 4),
                ("Espresso", 3.0, "Single-origin espresso shot", False, 5),
            ],
        ),
        (
            "Frank's Pizza Restaurant",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Italian",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="4950 S Rainbow Blvd, Las Vegas, NV 89118",
                lat=36.0986361,
                lng=-115.24244069999999,
            ),
            time(11, 0, 0),
            time(21, 0, 0),
            False,
            "Traditional Italian dishes prepared with imported ingredients and house-made pasta.",
            users["cust_bm_truck"],
            [
                ("Margherita pizza", 14.0, "Wood-fired with fresh mozzarella and basil", False, 1),
                ("Spaghetti carbonara", 16.0, "House-made pasta with pancetta and egg", False, 2),
                ("Caesar salad", 10.0, "Romaine with house-made dressing and croutons", False, 3),
                ("Tiramisu", 8.0, "Classic Italian coffee dessert", False, 4),
                ("Espresso", 3.0, "Single-origin espresso shot", False, 5),
            ],
        ),
        (
            "XoticEats Caribbean kitchen...online & pick-up orders only 🇹🇹",
            models.RestaurantType.FOOD_TRUCK,
            "Caribbean",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="1100 Sierra Vista Dr, Las Vegas, NV 89169",
                lat=36.1271724,
                lng=-115.139456,
            ),
            time(7, 0, 0),
            time(22, 0, 0),
            True,
            "Vibrant Caribbean flavors with island spices and fresh seafood.",
            users["cust_bm_truck"],
            [
                ("Jerk chicken", 13.0, "Spicy grilled chicken with jerk seasoning", False, 1),
                ("Curry goat", 14.0, "Slow-cooked curried goat", False, 2),
                ("Rice and peas", 5.0, "Coconut rice with kidney beans", False, 3),
                ("Plantains", 4.0, "Fried sweet plantains", False, 4),
                ("Rum cake", 6.0, "Traditional Caribbean rum cake", False, 5),
            ],
        ),
        (
            "Kase Sake & Sushi",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Japanese",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="5697 S Jones Blvd #220, Las Vegas, NV 89118",
                lat=36.0856977,
                lng=-115.2257764,
            ),
            time(11, 0, 0),
            time(22, 0, 0),
            True,
            "Traditional Japanese cuisine crafted with precision and fresh seafood.",
            users["cust_cart"],
            [
                ("Spicy tuna roll", 9.0, "Tuna with spicy mayo and cucumber", False, 1),
                ("Salmon nigiri", 12.0, "Fresh Atlantic salmon over rice", False, 2),
                ("Edamame", 5.0, "Steamed soybeans with sea salt", False, 3),
                ("Miso soup", 3.5, "Traditional miso with tofu and seaweed", False, 4),
                ("Green tea ice cream", 4.0, "Creamy matcha flavored ice cream", False, 5),
            ],
        ),
        (
            "Tacos-N-More",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Mexican",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="321 S Casino Center Blvd #130, Las Vegas, NV 89101",
                lat=36.1671556,
                lng=-115.1459405,
            ),
            time(12, 0, 0),
            time(21, 0, 0),
            False,
            "Authentic Mexican cuisine made from scratch with fresh ingredients.",
            users["cust_stall"],
            [
                ("Tacos al pastor", 4.5, "Grilled pork tacos with pineapple", False, 1),
                ("Carnitas burrito", 9.0, "Slow-cooked pork burrito with beans and rice", False, 2),
                ("Quesadillas", 6.0, "Cheese quesadilla with your choice of filling", False, 3),
                ("Guacamole & chips", 5.0, "Fresh tableside guacamole", False, 4),
                ("Agua fresca", 3.0, "Fresh fruit water", False, 5),
            ],
        ),
        (
            "Blvd Bar Bq & Wings Henderson",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="2895 N Green Valley Pkwy, Henderson, NV 89014",
                lat=36.074048399999995,
                lng=-115.08290199999999,
            ),
            time(8, 0, 0),
            time(20, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Chef Kenny's Asian Vegan Cafe",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Vegan",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="6820 Spring Mountain Rd, Las Vegas, NV 89146",
                lat=36.1267827,
                lng=-115.24100179999999,
            ),
            time(11, 0, 0),
            time(21, 0, 0),
            False,
            "Creative plant-based dishes that prove vegan food is anything but boring.",
            users["cust_bm_truck"],
            [
                ("Buddha bowl", 13.0, "Quinoa, roasted vegetables, tahini dressing", False, 1),
                ("Vegan burger", 12.0, "Plant-based patty with all the fixings", False, 2),
                ("Kale Caesar", 10.0, "Massaged kale with cashew Caesar dressing", False, 3),
                ("Sweet potato soup", 7.0, "Creamy vegan sweet potato soup", False, 4),
                ("Cold-pressed juice", 6.0, "Seasonal fruit and vegetable juice", False, 5),
            ],
        ),
        (
            "Locals Hawaiian Style Poke",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Hawaiian",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="4160 S Fort Apache Rd Suite F, Las Vegas, NV 89147",
                lat=36.1139101,
                lng=-115.2960787,
            ),
            time(7, 0, 0),
            time(21, 0, 0),
            True,
            "Fresh island-style poke bowls and Hawaiian comfort food.",
            users["cust_bm_truck"],
            [
                ("Poke bowl", 13.0, "Fresh ahi tuna over rice with toppings", False, 1),
                ("Hawaiian plate", 14.0, "Kalua pork, rice, and mac salad", False, 2),
                ("Spam musubi", 4.0, "Grilled spam on rice wrapped in nori", False, 3),
                ("Poi", 3.0, "Traditional taro root mash", False, 4),
                ("Haupia", 4.0, "Coconut milk dessert", False, 5),
            ],
        ),
        (
            "MIU Japanese BBQ & Sushi",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Japanese",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="3943 Spring Mountain Rd, Las Vegas, NV 89102",
                lat=36.1261662,
                lng=-115.192433,
            ),
            time(9, 0, 0),
            time(22, 0, 0),
            True,
            "Traditional Japanese cuisine crafted with precision and fresh seafood.",
            users["cust_cart"],
            [
                ("Spicy tuna roll", 9.0, "Tuna with spicy mayo and cucumber", False, 1),
                ("Salmon nigiri", 12.0, "Fresh Atlantic salmon over rice", False, 2),
                ("Edamame", 5.0, "Steamed soybeans with sea salt", False, 3),
                ("Miso soup", 3.5, "Traditional miso with tofu and seaweed", False, 4),
                ("Green tea ice cream", 4.0, "Creamy matcha flavored ice cream", False, 5),
            ],
        ),
        (
            "Forke Restaurant",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="2800 W Sahara Ave Suite 5A, Las Vegas, NV 89102",
                lat=36.145310699999996,
                lng=-115.1794417,
            ),
            time(9, 0, 0),
            time(20, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_stall"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Guerrilla Pizza Company",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Italian",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="900 S Las Vegas Blvd #120, Las Vegas, NV 89101",
                lat=36.160037599999995,
                lng=-115.1475706,
            ),
            time(8, 0, 0),
            time(20, 0, 0),
            True,
            "Traditional Italian dishes prepared with imported ingredients and house-made pasta.",
            users["cust_truck"],
            [
                ("Margherita pizza", 14.0, "Wood-fired with fresh mozzarella and basil", False, 1),
                ("Spaghetti carbonara", 16.0, "House-made pasta with pancetta and egg", False, 2),
                ("Caesar salad", 10.0, "Romaine with house-made dressing and croutons", False, 3),
                ("Tiramisu", 8.0, "Classic Italian coffee dessert", False, 4),
                ("Espresso", 3.0, "Single-origin espresso shot", False, 5),
            ],
        ),
        (
            "808 Cafe",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="4011 S Buffalo Dr B106, Las Vegas, NV 89147",
                lat=36.11658,
                lng=-115.26236929999999,
            ),
            time(7, 0, 0),
            time(23, 0, 0),
            False,
            "Comforting American classics made with quality ingredients.",
            users["cust_bm_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Shang Artisan Noodle",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Chinese",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="4983 W Flamingo Rd A, Las Vegas, NV 89103",
                lat=36.114927699999996,
                lng=-115.2099508,
            ),
            time(12, 0, 0),
            time(23, 0, 0),
            True,
            "Classic Chinese dishes wok-fried to perfection with bold flavors.",
            users["cust_bm_truck"],
            [
                ("Kung Pao chicken", 12.0, "Spicy stir-fried chicken with peanuts", False, 1),
                ("Mapo tofu", 10.0, "Silken tofu in spicy Sichuan sauce", False, 2),
                ("Fried rice", 8.0, "Wok-fried rice with vegetables and egg", False, 3),
                ("Spring rolls", 5.0, "Crispy vegetable spring rolls", False, 4),
                ("Hot and sour soup", 4.0, "Traditional hot and sour soup", False, 5),
            ],
        ),
        (
            "Yourway Breakfast + Lunch",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="at Jones, 6121 W Lake Mead Blvd Suite 110, Las Vegas, NV 89108",
                lat=36.194566,
                lng=-115.22469369999999,
            ),
            time(8, 0, 0),
            time(20, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_cart"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Naan & Curry - Indian Food (North Summerlin)",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Indian",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="7660 W Cheyenne Ave #126, Las Vegas, NV 89129",
                lat=36.218656599999996,
                lng=-115.2616788,
            ),
            time(10, 0, 0),
            time(23, 0, 0),
            True,
            "Aromatic Indian curries and tandoori specialties made with traditional spices.",
            users["cust_stall"],
            [
                ("Chicken tikka masala", 14.0, "Creamy tomato-spiced curry", False, 1),
                ("Garlic naan", 4.0, "Tandoor-baked garlic flatbread", False, 2),
                ("Biryani", 12.0, "Fragrant spiced rice with vegetables", False, 3),
                ("Samosas", 6.0, "Crispy pastry filled with spiced potatoes", False, 4),
                ("Mango lassi", 4.0, "Creamy mango yogurt drink", False, 5),
            ],
        ),
        (
            "Squeeze In Breakfast & Lunch",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="5165 S Fort Apache Rd #195, Las Vegas, NV 89148",
                lat=36.0959636,
                lng=-115.2981294,
            ),
            time(10, 0, 0),
            time(21, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
        (
            "Bangkok Street Thai Street Kitchen",
            models.RestaurantType.BRICK_AND_MORTAR,
            "Thai",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="611 E Fremont St #150, Las Vegas, NV 89101",
                lat=36.168289699999995,
                lng=-115.1391215,
            ),
            time(10, 0, 0),
            time(21, 0, 0),
            True,
            "Bold and aromatic Thai street food with fresh herbs and spices.",
            users["cust_bm_truck"],
            [
                ("Pad Thai", 7.0, "Stir-fried rice noodles with tamarind sauce", False, 1),
                ("Green curry", 8.0, "Spicy coconut curry with vegetables", False, 2),
                ("Tom yum soup", 6.0, "Hot and sour Thai soup", False, 3),
                ("Spring rolls", 5.0, "Crispy vegetable spring rolls", False, 4),
                ("Mango sticky rice", 6.0, "Sweet coconut sticky rice with mango", False, 5),
            ],
        ),
        (
            "Durvo",
            models.RestaurantType.BRICK_AND_MORTAR,
            "American",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="5255 S Decatur Blvd #107, Las Vegas, NV 89118",
                lat=36.094299299999996,
                lng=-115.2088546,
            ),
            time(10, 0, 0),
            time(23, 0, 0),
            True,
            "Comforting American classics made with quality ingredients.",
            users["cust_bm_truck"],
            [
                ("Classic cheeseburger", 12.0, "Angus beef with cheddar and house sauce", False, 1),
                ("Crispy chicken sandwich", 11.0, "Buttermilk fried chicken on brioche", False, 2),
                ("Loaded fries", 8.0, "Fries with cheese, bacon, and ranch", False, 3),
                ("House salad", 7.0, "Mixed greens with seasonal vegetables", False, 4),
                ("Milkshake", 5.0, "Hand-spun vanilla, chocolate, or strawberry", False, 5),
            ],
        ),
    ]

    for definition in approved_definitions:
        (
            name,
            restaurant_type,
            cuisine_type,
            location,
            open_time,
            close_time,
            open_status,
            description,
            owner,
            menu_item_defs,
        ) = definition
        restaurant = _make_restaurant_data(
            name=name,
            restaurant_type=restaurant_type,
            cuisine_type=cuisine_type,
            location=location,
            open_time=open_time,
            close_time=close_time,
            open_status=open_status,
            description=description,
            is_approved=True,
            owner=owner,
        )
        database_session.add(restaurant)
        database_session.flush()
        for i, (item_name, price, item_desc, sold_out, sort_order) in enumerate(menu_item_defs):
            _make_menu_item(
                database_session,
                restaurant,
                item_name,
                price,
                description=item_desc,
                is_sold_out=sold_out,
                sort_order=sort_order,
            )
        restaurants.append(restaurant)

    return restaurants


def _seed_unapproved_restaurants(
    database_session: Session, users: Dict[str, models.User]
) -> List[models.Restaurant]:
    restaurants = []

    unapproved_definitions = [
        (
            "Taste of Thai",
            models.RestaurantType.FOOD_CART,
            "Thai",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="Pending location assignment",
                lat=36.1300,
                lng=-115.1940,
            ),
            time(17, 0, 0),
            time(23, 0, 0),
            True,
            "New Thai food cart awaiting approval. Offers authentic street food.",
            users["cust_pending1"],
            [
                ("Pad Thai", 7.00, "Stir-fried rice noodles with tamarind sauce", False, 1),
                ("Green curry", 8.00, "Spicy coconut curry with vegetables", False, 2),
                ("Spring rolls", 5.00, "Crispy vegetable spring rolls", False, 3),
                ("Mango sticky rice", 6.00, "Sweet coconut sticky rice with mango", False, 4),
            ],
        ),
        (
            "Sushi Express",
            models.RestaurantType.FOOD_TRUCK,
            "Japanese",
            _make_location(
                database_session,
                models.LocationCategory.OTHER,
                description="Pending location confirmation",
                lat=36.0400,
                lng=-114.9820,
            ),
            time(12, 0, 0),
            time(21, 0, 0),
            False,
            "Mobile sushi truck waiting for admin approval before launching.",
            users["cust_pending2"],
            [
                ("California roll", 8.00, "Crab, avocado, and cucumber roll", False, 1),
                ("Spicy tuna roll", 9.00, "Tuna with spicy mayo and cucumber", False, 2),
                ("Nigiri selection", 12.00, "Chef's choice of three nigiri", False, 3),
                ("Poke bowls", 11.00, "Marinated fish over rice with toppings", False, 4),
            ],
        ),
    ]

    for definition in unapproved_definitions:
        (
            name,
            restaurant_type,
            cuisine_type,
            location,
            open_time,
            close_time,
            open_status,
            description,
            owner,
            menu_item_defs,
        ) = definition
        restaurant = _make_restaurant_data(
            name=name,
            restaurant_type=restaurant_type,
            cuisine_type=cuisine_type,
            location=location,
            open_time=open_time,
            close_time=close_time,
            open_status=open_status,
            description=description,
            is_approved=False,
            owner=owner,
        )
        database_session.add(restaurant)
        database_session.flush()
        for i, (item_name, price, item_desc, sold_out, sort_order) in enumerate(menu_item_defs):
            _make_menu_item(
                database_session,
                restaurant,
                item_name,
                price,
                description=item_desc,
                is_sold_out=sold_out,
                sort_order=sort_order,
            )
        restaurants.append(restaurant)

    return restaurants


def _seed_favorites(
    database_session: Session, users: Dict[str, models.User], restaurants: List[models.Restaurant]
) -> None:
    approved_restaurants = [r for r in restaurants if r.is_approved]
    restaurant_by_name = {r.name: r for r in approved_restaurants}

    favorite_definitions = [
        ("consumer1", "D.O.P.E. Chicken"),
        ("consumer1", "Hunan Masala"),
        ("consumer2", "Shang Artisan Noodle"),
    ]

    for consumer_username, restaurant_name in favorite_definitions:
        consumer = users.get(consumer_username)
        restaurant = restaurant_by_name.get(restaurant_name)
        if consumer and restaurant:
            existing = (
                database_session.query(models.Favorite)
                .filter(
                    models.Favorite.consumer_id == consumer.id,
                    models.Favorite.restaurant_id == restaurant.id,
                )
                .first()
            )
            if not existing:
                favorite = models.Favorite(
                    consumer_id=consumer.id, restaurant_id=restaurant.id
                )
                database_session.add(favorite)


def seed_demo_data(database_session: Session) -> None:
    existing_restaurant_count = database_session.query(models.Restaurant).count()
    if existing_restaurant_count > 0:
        logger.info(
            "Demo data already exists (%d restaurants found), skipping seed",
            existing_restaurant_count,
        )
        return

    logger.info("Seeding demo data into the database")

    users = _seed_users(database_session)
    database_session.flush()

    approved_restaurants = _seed_approved_restaurants(database_session, users)
    unapproved_restaurants = _seed_unapproved_restaurants(database_session, users)

    all_restaurants = approved_restaurants + unapproved_restaurants
    database_session.flush()

    _seed_favorites(database_session, users, all_restaurants)

    database_session.commit()

    logger.info("Demo data seeded: %d restaurants, %d favorites", len(all_restaurants), 3)


if __name__ == "__main__":
    import os
    logging.basicConfig(level=logging.INFO)
    os.makedirs("run_dev", exist_ok=True)
    from app.database import engine, Base, SessionLocal

    Base.metadata.create_all(bind=engine)
    database_session = SessionLocal()
    try:
        seed_demo_data(database_session)
    finally:
        database_session.close()
