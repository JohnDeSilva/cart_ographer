from pydantic import BaseModel


class LocationInput(BaseModel):
    name: str
    state: str
    city: str
    zipcode: str
    street_name: str
    street_number: str
    site: str
