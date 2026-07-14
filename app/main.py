from datetime import time, timedelta, datetime, timezone
from typing import List, Optional, Any, Dict
import jwt
from fastapi import FastAPI, Depends, HTTPException, Query, status
from fastapi.security import OAuth2PasswordBearer
from sqlalchemy.orm import Session
from app import crud, models, schemas
from app.database import engine, get_db, Base

# Create tables
Base.metadata.create_all(bind=engine)

app = FastAPI(title="Restaurant Tracker API", version="0.1.0")

SECRET_KEY = "super-secret-key-for-jwt"
ALGORITHM = "HS256"
ACCESS_TOKEN_EXPIRE_MINUTES = 60

oauth2_scheme = OAuth2PasswordBearer(tokenUrl="/auth/login", auto_error=False)


def create_access_token(data: Dict[str, Any], expires_delta: Optional[timedelta] = None) -> str:
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


@app.on_event("startup")
def on_startup() -> None:
    db = next(get_db())
    try:
        # Seed default admin user
        if not crud.get_user_by_username(db, "admin"):
            crud.create_user(
                db,
                schemas.UserCreate(
                    username="admin",
                    password="adminpassword",
                    role=models.UserRole.ADMIN,
                ),
            )
        # Seed default customer user
        if not crud.get_user_by_username(db, "customer"):
            crud.create_user(
                db,
                schemas.UserCreate(
                    username="customer",
                    password="customerpassword",
                    role=models.UserRole.CUSTOMER,
                ),
            )
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
def login(user_credentials: schemas.UserLogin, db: Session = Depends(get_db)) -> Dict[str, Any]:
    user = crud.get_user_by_username(db, username=user_credentials.username)
    if not user or not crud.verify_password(
        user_credentials.password, user.hashed_password
    ):
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



# Restaurant routes
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
    return crud.create_restaurant(db=db, restaurant=restaurant)


@app.get("/restaurants", response_model=List[schemas.RestaurantResponse])
def read_restaurants(
    name: Optional[str] = Query(
        None, description="Filter by partial restaurant name (case-insensitive)"
    ),
    type: Optional[models.RestaurantType] = Query(
        None, alias="restaurant_type", description="Filter by restaurant type"
    ),
    open_time: Optional[time] = Query(
        None, description="Filter by exact open time (HH:MM:SS)"
    ),
    close_time: Optional[time] = Query(
        None, description="Filter by exact close time (HH:MM:SS)"
    ),
    open_status: Optional[bool] = Query(None, description="Filter by open status"),
    is_open_at: Optional[time] = Query(
        None, description="Filter by whether restaurant is open at this specific time"
    ),
    skip: int = 0,
    limit: int = 100,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_user),
) -> List[models.Restaurant]:
    return crud.get_restaurants(
        db=db,
        name=name,
        restaurant_type=type,
        open_time=open_time,
        close_time=close_time,
        open_status=open_status,
        is_open_at=is_open_at,
        skip=skip,
        limit=limit,
    )


@app.get("/restaurants/{restaurant_id}", response_model=schemas.RestaurantResponse)
def read_restaurant(
    restaurant_id: int,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_user),
) -> models.Restaurant:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    return db_restaurant


@app.put("/restaurants/{restaurant_id}", response_model=schemas.RestaurantResponse)
def update_restaurant(
    restaurant_id: int,
    restaurant_update: schemas.RestaurantUpdate,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_admin),
) -> models.Restaurant:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    return crud.update_restaurant(
        db=db, db_restaurant=db_restaurant, restaurant_update=restaurant_update
    )


@app.patch(
    "/restaurants/{restaurant_id}/status", response_model=schemas.RestaurantResponse
)
def update_restaurant_status(
    restaurant_id: int,
    status_update: schemas.RestaurantStatusUpdate,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_admin),
) -> models.Restaurant:
    db_restaurant = crud.get_restaurant(db, restaurant_id=restaurant_id)
    if db_restaurant is None:
        raise HTTPException(status_code=404, detail="Restaurant not found")
    return crud.update_restaurant_status(
        db=db, db_restaurant=db_restaurant, open_status=status_update.open_status
    )


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
