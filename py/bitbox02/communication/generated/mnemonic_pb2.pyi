# @generated by generate_proto_mypy_stubs.py.  Do not edit!
import sys
from google.protobuf.message import (
    Message as google___protobuf___message___Message,
)

from typing import (
    Optional as typing___Optional,
)

from typing_extensions import (
    Literal as typing_extensions___Literal,
)


class ShowMnemonicRequest(google___protobuf___message___Message):

    def __init__(self,
        ) -> None: ...
    @classmethod
    def FromString(cls, s: bytes) -> ShowMnemonicRequest: ...
    def MergeFrom(self, other_msg: google___protobuf___message___Message) -> None: ...
    def CopyFrom(self, other_msg: google___protobuf___message___Message) -> None: ...

class RestoreFromMnemonicRequest(google___protobuf___message___Message):
    timestamp = ... # type: int
    timezone_offset = ... # type: int

    def __init__(self,
        *,
        timestamp : typing___Optional[int] = None,
        timezone_offset : typing___Optional[int] = None,
        ) -> None: ...
    @classmethod
    def FromString(cls, s: bytes) -> RestoreFromMnemonicRequest: ...
    def MergeFrom(self, other_msg: google___protobuf___message___Message) -> None: ...
    def CopyFrom(self, other_msg: google___protobuf___message___Message) -> None: ...
    if sys.version_info >= (3,):
        def ClearField(self, field_name: typing_extensions___Literal[u"timestamp",u"timezone_offset"]) -> None: ...
    else:
        def ClearField(self, field_name: typing_extensions___Literal[u"timestamp",b"timestamp",u"timezone_offset",b"timezone_offset"]) -> None: ...

class SetMnemonicPassphraseEnabledRequest(google___protobuf___message___Message):
    enabled = ... # type: bool

    def __init__(self,
        *,
        enabled : typing___Optional[bool] = None,
        ) -> None: ...
    @classmethod
    def FromString(cls, s: bytes) -> SetMnemonicPassphraseEnabledRequest: ...
    def MergeFrom(self, other_msg: google___protobuf___message___Message) -> None: ...
    def CopyFrom(self, other_msg: google___protobuf___message___Message) -> None: ...
    if sys.version_info >= (3,):
        def ClearField(self, field_name: typing_extensions___Literal[u"enabled"]) -> None: ...
    else:
        def ClearField(self, field_name: typing_extensions___Literal[u"enabled",b"enabled"]) -> None: ...
