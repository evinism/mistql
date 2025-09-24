from os import environ


def is_flag_enabled(flag_name, default=False):
    env_value = environ.get(flag_name)
    if env_value is None:
        return default
    else:
        return env_value.lower() in ("1", "true", "yes")


TYPEGUARD = is_flag_enabled("TYPEGUARD", default=False)
PROFILE = is_flag_enabled("PROFILE", default=False)
