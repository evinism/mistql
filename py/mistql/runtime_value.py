from enum import Enum
from typing import Callable, List


class RuntimeValueType(Enum):
    """
    Enumeration of the different types of runtime values available in MistQL
    """

    Null = "null"
    Boolean = "boolean"
    Number = "number"
    String = "string"
    Object = "object"
    Array = "array"
    Function = "function"
    Regex = "regex"


class RuntimeValue:
    @staticmethod
    def of(value):
        """
        Convert a Python value into a MistQL RuntimeValue
        """
        if isinstance(value, RuntimeValue):
            return value
        elif value is None:
            return RuntimeValue(RuntimeValueType.Null)
        elif isinstance(value, bool):
            return RuntimeValue(RuntimeValueType.Boolean, value)
        elif isinstance(value, int):
            return RuntimeValue(RuntimeValueType.Number, float(value))
        elif isinstance(value, float):
            return RuntimeValue(RuntimeValueType.Number, value)
        elif isinstance(value, str):
            return RuntimeValue(RuntimeValueType.String, value)
        elif isinstance(value, list):
            return RuntimeValue(
                RuntimeValueType.Array,
                [RuntimeValue.of(item) for item in value],
            )
        elif isinstance(value, dict):
            return RuntimeValue(
                RuntimeValueType.Object,
                {key: RuntimeValue.of(value[key]) for key in value},
            )
        else:
            raise ValueError(
                "Cannot convert external type to MistQL type: " + str(type(value))
            )

    @staticmethod
    def create_function(definition: Callable):
        """
        Create a new function that can be used in MistQL expressions
        """
        return RuntimeValue(
            RuntimeValueType.Function,
            definition,
        )

    def __init__(self, type, value=None):
        self.type = type
        self.value = value

    def to_python(self):
        """
        Convert a MistQL RuntimeValue into a Python value
        """
        if self.type == RuntimeValueType.Null:
            return None
        elif self.type == RuntimeValueType.Boolean:
            return self.value
        elif self.type == RuntimeValueType.Number:
            return self.value
        elif self.type == RuntimeValueType.String:
            return self.value
        elif self.type == RuntimeValueType.Array:
            return [item.to_python() for item in self.value]
        elif self.type == RuntimeValueType.Object:
            return {key: value.to_python() for key, value in self.value.items()}
        else:
            raise ValueError(
                "Cannot convert MistQL value type to Python: " + str(self.type)
            )
