import logging
import os
from datetime import time, timedelta, datetime, timezone
from typing import List, Optional, Any, Dict

import jwt
from fastapi import FastAPI, Depends, HTTPException, Query, status
from fastapi.middleware.cors import CORSMiddleware
from fastapi.security import OAuth2PasswordBearer
from fastapi.staticfiles import StaticFiles
from sqlalchemy.orm import Session

from app import crud, models, schemas
from app.database import engine, get_db, Base

logger = logging.getLogger(__name__)

# Create tables
Base.metadata.create_all(bind=engine)

app = FastAPI(title="Restaurant Directory API", version="0.2.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:5173", "http://127.0.0.1:5173"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

SECRET_KEY = "super-secret-key-for-jwt"
ALGORITHM = "HS256"
ACCESS_TOKEN_EXPIRE_MINUTES = 60

oauth2_scheme = OAuth2PasswordBearer(tokenUrl="/auth/login", auto_error=False)


def create_access_token(
    data: Dict[str, Any], expires_delta: Optional[timedelta] = None
) -> str:
    to_encode = data.copy()
    if expires_delta:
        expire = datetime.now(timezone.utc) + expires_delta
    else:
        expire = datetime.now(timezone.utc) + timedelta(minutes=15)
    to_encode.update({"exp": expire})
    encoded_jwt: str = jwt.encode(to_encode, SECRET_KEY, algorithm=ALGORITHM)
    return encoded_jwt


def get_current_user(
    token: str = Depends(oauth2_scheme), db: Session = Depends(get_db)
) -> models.User:
    credentials_exception = HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Could not validate credentials",
        headers={"WWW-Authenticate": "Bearer"},
    )
    if not token:
        raise credentials_exception
    try:
        payload = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
        username: Optional[str] = payload.get("sub")
        if username is None:
            raise credentials_exception
    except jwt.PyJWTError:
        raise credentials_exception
    user = crud.get_user_by_username(db, username=username)
    if user is None:
        raise credentials_exception
    return user


def get_current_admin(
    current_user: models.User = Depends(get_current_user),
) -> models.User:
    if current_user.role != models.UserRole.ADMIN:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="The user does not have enough privileges",
        )
    return current_user


def get_current_customer(
    current_user: models.User = Depends(get_current_user),
) -> models.User:
    if current_user.role != models.UserRole.CUSTOMER:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Only restaurant owners can perform this action",
        )
    return current_user


def get_current_consumer(
    current_user: models.User = Depends(get_current_user),
) -> models.User:
    if current_user.role != models.UserRole.CONSUMER:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Only consumers can perform this action",
        )
    return current_user


@app.on_event("startup")
def on_startup() -> None:
    db = next(get_db())
    try:
        if not crud.get_user_by_username(db, "admin"):
            crud.create_user(
                db,
                schemas.UserCreate(
                    username="admin",
                    password="adminpassword",
                    role=models.UserRole.ADMIN,
                ),
            )
            logger.info("Seeded default admin user into the database")
        if not crud.get_user_by_username(db, "consumer"):
            crud.create_user(
                db,
                schemas.UserCreate(
                    username="consumer",
                    password="consumerpassword",
                    role=models.UserRole.CONSUMER,
                ),
            )
            logger.info("Seeded default consumer user into the database")
    finally:
        db.close()


# Auth routes
@app.post(
    "/auth/signup",
    response_model=schemas.UserResponse,
    status_code=status.HTTP_201_CREATED,
)
def signup(user: schemas.UserCreate, db: Session = Depends(get_db)) -> models.User:
    db_user = crud.get_user_by_username(db, username=user.username)
    if db_user:
        raise HTTPException(status_code=400, detail="Username already registered")
    return crud.create_user(db=db, user=user)


@app.post("/auth/login", response_model=schemas.Token)
def login(
    user_credentials: schemas.UserLogin, db: Session = Depends(get_db)
) -> Dict[str, Any]:
    user = crud.get_user_by_username(db, username=user_credentials.username)
    if not user or not crud.verify_password(
        user_credentials.password, user.hashed_password
    ):
        logger.warning(
            "Failed login attempt for username: '%s'", user_credentials.username
        )
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Incorrect username or password",
            headers={"WWW-Authenticate": "Bearer"},
        )
    access_token_expires = timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES)
    access_token = create_access_token(
        data={"sub": user.username, "role": user.role},
        expires_delta=access_token_expires,
    )
    logger.debug("Login successful for username: '%s'", user_credentials.username)
    return {
        "access_token": access_token,
        "token_type": "bearer",
        "role": user.role,
        "username": user.username,
    }


@app.post("/auth/reset-password", response_model=schemas.UserResponse)
def reset_password(
    reset_data: schemas.UserPasswordReset, db: Session = Depends(get_db)
) -> models.User:
    user = crud.get_user_by_username(db, username=reset_data.username)
    if not user:
        raise HTTPException(status_code=404, detail="User not found")
    return crud.reset_user_password(
        db=db, db_user=user, new_password=reset_data.new_password
    )


# Admin restaurant routes
@app.post(
    "/restaurants",
    response_model=schemas.RestaurantResponse,
    status_code=status.HTTP_201_CREATED,
)
def create_restaurant(
    restaurant: schemas.RestaurantCreate,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_admin),
) -> models.Restaurant:
    created = crud.create_restaurant(db=db, restaurant=restaurant)
    created.is_approved = True
    db.commit()
    db.refresh(created)
    return created


# Customer restaurant submission
@app.post(
    "/restaurants/submit",
    response_model=schemas.RestaurantResponse,
    status_code=status.HTTP_201_CREATED,
)
def submit_restaurant(
    restaurant: schemas.RestaurantSubmit,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_customer),
) -> models.Restaurant:
    return crud.submit_restaurant(
        db=db, restaurant=restaurant, owner_id=current_user.id
    )


# Read restaurants - with role-based filtering
@app.get("/restaurants", response_model=List[schemas.RestaurantResponse])
def read_restaurants(
    name: Optional[str] = Query(None, description="Filter by partial restaurant name"),
    restaurant_type: Optional[models.RestaurantType] = Query(
        None, alias="restaurant_type", description="Filter by restaurant type"
    ),
    cuisine_type: Optional[str] = Query(None, description="Filter by cuisine type"),
    open_time: Optional[time] = Query(None, description="Filter by exact open time"),
    close_time: Optional[time] = Query(None, description="Filter by exact close time"),
    open_status: Optional[bool] = Query(None, description="Filter by open status"),
    is_open_at: Optional[time] = Query(
        None, description="Filter by open at this specific time"
    ),
    menu_items: Optional[str] = Query(None, description="Filter by menu items"),
    is_approved: Optional[bool] = Query(
        None, description="Filter by approval status (admin only)"
    ),
    skip: int = 0,
    limit: int = 100,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_user),
) -> List[models.Restaurant]:
    if current_user.role == models.UserRole.CUSTOMER:
        return crud.get_restaurants_by_owner(db=db, owner_id=current_user.id)
    effective_approved = is_approved
    if current_user.role == models.UserRole.CONSUMER:
        effective_approved = True
    return crud.get_restaurants(
        db=db,
        name=name,
        restaurant_type=restaurant_type,
        cuisine_type=cuisine_type,
        open_time=open_time,
        close_time=close_time,
        open_status=open_status,
        is_open_at=is_open_at,
        is_approved=effective_approved,
        menu_items=menu_items,
        skip=skip,
        limit=limit,
    )


# Customer's own restaurants
@app.get(
    "/me/restaurants",
    response_model=List[schemas.RestaurantResponse],
)
def read_my_restaurants(
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_customer),
) -> List[models.Restaurant]:
    return crud.get_restaurants_by_owner(db=db, owner_id=current_user.id)


@app.get("/restaurants/{restaurant_id}", response_model=schemas.RestaurantResponse)
def read_restaurant(
    restaurant_id: int,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_user),
) -> models.Restaurant:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    if current_user.role == models.UserRole.CONSUMER and not db_restaurant.is_approved:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    return db_restaurant


@app.put("/restaurants/{restaurant_id}", response_model=schemas.RestaurantResponse)
def update_restaurant(
    restaurant_id: int,
    restaurant_update: schemas.RestaurantUpdate,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_user),
) -> models.Restaurant:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")

    if current_user.role == models.UserRole.ADMIN:
        return crud.update_restaurant(
            db=db, db_restaurant=db_restaurant, restaurant_update=restaurant_update
        )

    if current_user.role == models.UserRole.CUSTOMER:
        if db_restaurant.owner_id != current_user.id:
            raise HTTPException(
                status_code=status.HTTP_403_FORBIDDEN,
                detail="You can only update your own restaurants",
            )
        if restaurant_update.name is not None:
            raise HTTPException(
                status_code=status.HTTP_403_FORBIDDEN,
                detail="Customers cannot change the restaurant name",
            )
        if restaurant_update.location is not None and db_restaurant.restaurant_type == models.RestaurantType.BRICK_AND_MORTAR:
                non_loc_data = restaurant_update.model_dump(exclude={"location"}, exclude_unset=True)
                crud.update_restaurant(
                    db=db,
                    db_restaurant=db_restaurant,
                    restaurant_update=schemas.RestaurantUpdate(**non_loc_data),
                )
                loc_data = restaurant_update.location.model_dump(exclude_unset=True)
                if "location_type" not in loc_data and db_restaurant.location:
                    loc_data["location_type"] = db_restaurant.location.location_type
                return crud.request_location_change(
                    db=db, db_restaurant=db_restaurant,
                    new_location=schemas.LocationCreate(**loc_data)
                )
        if restaurant_update.location is not None:
            crud.update_restaurant_location(
                db=db, db_restaurant=db_restaurant, location_update=restaurant_update.location
            )
        return crud.update_restaurant(
            db=db, db_restaurant=db_restaurant, restaurant_update=restaurant_update
        )

    raise HTTPException(
        status_code=status.HTTP_403_FORBIDDEN,
        detail="You do not have permission to update restaurants",
    )


@app.patch(
    "/restaurants/{restaurant_id}/status",
    response_model=schemas.RestaurantResponse,
)
def update_restaurant_status(
    restaurant_id: int,
    status_update: schemas.RestaurantStatusUpdate,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_user),
) -> models.Restaurant:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")

    if current_user.role == models.UserRole.ADMIN:
        return crud.update_restaurant_status(
            db=db, db_restaurant=db_restaurant, open_status=status_update.open_status
        )

    if current_user.role == models.UserRole.CUSTOMER:
        if db_restaurant.owner_id != current_user.id:
            raise HTTPException(
                status_code=status.HTTP_403_FORBIDDEN,
                detail="You can only toggle status on your own restaurants",
            )
        return crud.update_restaurant_status(
            db=db, db_restaurant=db_restaurant, open_status=status_update.open_status
        )

    raise HTTPException(
        status_code=status.HTTP_403_FORBIDDEN,
        detail="You do not have permission to update restaurant status",
    )


# Admin approval endpoints
@app.patch(
    "/restaurants/{restaurant_id}/approve",
    response_model=schemas.RestaurantResponse,
)
def approve_restaurant(
    restaurant_id: int,
    approval: schemas.RestaurantApproval,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_admin),
) -> models.Restaurant:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    if approval.is_approved:
        return crud.approve_restaurant(db=db, db_restaurant=db_restaurant)
    return db_restaurant


@app.patch(
    "/restaurants/{restaurant_id}/approve-location",
    response_model=schemas.RestaurantResponse,
)
def approve_location_change(
    restaurant_id: int,
    location_approval: schemas.LocationApproval,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_admin),
) -> models.Restaurant:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    if location_approval.approve:
        return crud.approve_location_change(db=db, db_restaurant=db_restaurant)
    else:
        return crud.reject_location_change(db=db, db_restaurant=db_restaurant)


@app.delete("/restaurants/{restaurant_id}", status_code=status.HTTP_204_NO_CONTENT)
def delete_restaurant(
    restaurant_id: int,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_admin),
) -> None:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    crud.delete_restaurant(db=db, db_restaurant=db_restaurant)


# Favorites routes (Consumer only)
@app.post(
    "/favorites",
    response_model=schemas.FavoriteResponse,
    status_code=status.HTTP_201_CREATED,
)
def add_favorite(
    favorite_data: schemas.FavoriteCreate,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_consumer),
) -> models.Favorite:
    db_restaurant = crud.get_restaurant(db, restaurant_id=favorite_data.restaurant_id)
    if db_restaurant is None or not db_restaurant.is_approved:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    return crud.add_favorite(
        db=db, consumer_id=current_user.id, restaurant_id=favorite_data.restaurant_id
    )


@app.delete(
    "/favorites/{favorite_id}",
    status_code=status.HTTP_204_NO_CONTENT,
)
def remove_favorite(
    favorite_id: int,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_consumer),
) -> None:
    db_favorite = crud.get_favorite_by_identifier(db, favorite_id=favorite_id)
    if db_favorite is None:
        raise HTTPException(status_code=404, detail="Favorite not found")
    if db_favorite.consumer_id != current_user.id:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="You can only remove your own favorites",
        )
    crud.remove_favorite(db=db, favorite_id=favorite_id)


@app.get(
    "/favorites",
    response_model=List[schemas.FavoriteResponse],
)
def read_favorites(
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_consumer),
) -> List[models.Favorite]:
    return crud.get_favorites_by_consumer(db=db, consumer_id=current_user.id)


# Mount static web client files at root if built folder is present
if os.path.exists("web_client/dist"):
    app.mount("/", StaticFiles(directory="web_client/dist", html=True), name="static")
