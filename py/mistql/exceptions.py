class OpenAnIssueIfYouGetThisError(Exception):
    """Please open an issue if you get this error.

    Issues can be submitted at https://github.com/evinism/mistql/issues/new
    """

    def __init__(self, message: str):
        docstr = self.__doc__ or ""
        self.message = message + "\n\n" + docstr
        super().__init__(message)

    pass


class MistQLException(Exception):
    """
    Base class for all MistQL exceptions.
    """

    pass


class MistQLRuntimeError(MistQLException):
    """
    Raised when the MistQL expression is invalid.
    """

    pass


class MistQLReferenceError(MistQLException):
    """
    Raised when a reference is invalid.
    """

    pass


class MistQLTypeError(MistQLRuntimeError):
    """
    Raised when the MistQL expression is invalid.
    """

    pass
