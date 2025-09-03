import uvicorn
from fastapi import FastAPI
from contextlib import asynccontextmanager
from app.database import engine
from app.sqlalchemy.base import Base
from app.router.restaurant import router as restaurant_router

app = FastAPI()


app.include_router(restaurant_router)


@asynccontextmanager
async def lifespan(app: FastAPI):
    print("Application startup...")
    print("Creating database tables...")
    Base.metadata.create_all(bind=engine)
    print("Database tables created successfully.")
    yield
    print("Application shutdown...")


@app.get("/")
def root():
    return {"message": "Hello World"}


if __name__ == "__main__":
    # The `uvicorn.run()` function is the standard way to run an ASGI application.
    # The first argument is the application instance, formatted as "module:variable".
    # `reload=True` is a key flag for development, as it restarts the server on code changes.
    uvicorn.run(
        "cart_ographer:app",  # The format is 'module_name:app_instance_name'
        host="127.0.0.1",
        port=8000,
        reload=True,  # This is crucial for development
    )
