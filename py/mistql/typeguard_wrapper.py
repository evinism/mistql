from typeguard import typechecked as typechecked_decorator
from os import environ
from mistql.env_flags import TYPEGUARD


def typechecked(func):
    if TYPEGUARD:
        return typechecked_decorator(func)
    return func
