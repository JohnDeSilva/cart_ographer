from sqlalchemy import String
from sqlalchemy.orm import Mapped, mapped_column

from app.sqlalchemy.base import CommonColumns


class Location(CommonColumns):
    __tablename__ = "location"
    name: Mapped[str] = mapped_column(String, unique=True)
    state: Mapped[str] = mapped_column(String)
    city: Mapped[str] = mapped_column(String)
    zipcode: Mapped[str] = mapped_column(String)
    street_name: Mapped[str] = mapped_column(String)
    street_number: Mapped[str] = mapped_column(String)
    site: Mapped[str] = mapped_column(String)
