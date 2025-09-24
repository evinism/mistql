from typeguard import typechecked as typechecked_decorator
from os import environ

typeguard_enabled = environ.get("TYPEGUARD_ENABLED", "").lower() in ("1", "true", "yes")


def typechecked(func):
    if typeguard_enabled:
        return typechecked_decorator(func)
    return func
