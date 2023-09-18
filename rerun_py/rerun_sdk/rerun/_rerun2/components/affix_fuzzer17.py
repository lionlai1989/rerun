# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer17Ext" class in "affix_fuzzer17_ext.py".

from __future__ import annotations

from typing import Sequence, Union

import pyarrow as pa
from attrs import define, field

from .. import datatypes
from .._baseclasses import (
    BaseExtensionArray,
    BaseExtensionType,
)

__all__ = ["AffixFuzzer17", "AffixFuzzer17Array", "AffixFuzzer17ArrayLike", "AffixFuzzer17Like", "AffixFuzzer17Type"]


@define
class AffixFuzzer17:
    # You can define your own __init__ function as a member of AffixFuzzer17Ext in affix_fuzzer17_ext.py

    many_optional_unions: list[datatypes.AffixFuzzer3] | None = field(default=None)


AffixFuzzer17Like = AffixFuzzer17
AffixFuzzer17ArrayLike = Union[
    AffixFuzzer17,
    Sequence[AffixFuzzer17Like],
]


# --- Arrow support ---


class AffixFuzzer17Type(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.list_(
                pa.field(
                    "item",
                    pa.dense_union(
                        [
                            pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                            pa.field("degrees", pa.float32(), nullable=False, metadata={}),
                            pa.field("radians", pa.float32(), nullable=False, metadata={}),
                            pa.field(
                                "craziness",
                                pa.list_(
                                    pa.field(
                                        "item",
                                        pa.struct(
                                            [
                                                pa.field(
                                                    "single_float_optional", pa.float32(), nullable=True, metadata={}
                                                ),
                                                pa.field(
                                                    "single_string_required", pa.utf8(), nullable=False, metadata={}
                                                ),
                                                pa.field(
                                                    "single_string_optional", pa.utf8(), nullable=True, metadata={}
                                                ),
                                                pa.field(
                                                    "many_floats_optional",
                                                    pa.list_(
                                                        pa.field("item", pa.float32(), nullable=False, metadata={})
                                                    ),
                                                    nullable=True,
                                                    metadata={},
                                                ),
                                                pa.field(
                                                    "many_strings_required",
                                                    pa.list_(pa.field("item", pa.utf8(), nullable=False, metadata={})),
                                                    nullable=False,
                                                    metadata={},
                                                ),
                                                pa.field(
                                                    "many_strings_optional",
                                                    pa.list_(pa.field("item", pa.utf8(), nullable=False, metadata={})),
                                                    nullable=True,
                                                    metadata={},
                                                ),
                                                pa.field("flattened_scalar", pa.float32(), nullable=False, metadata={}),
                                                pa.field(
                                                    "almost_flattened_scalar",
                                                    pa.struct(
                                                        [pa.field("value", pa.float32(), nullable=False, metadata={})]
                                                    ),
                                                    nullable=False,
                                                    metadata={},
                                                ),
                                                pa.field("from_parent", pa.bool_(), nullable=True, metadata={}),
                                            ]
                                        ),
                                        nullable=False,
                                        metadata={},
                                    )
                                ),
                                nullable=False,
                                metadata={},
                            ),
                            pa.field(
                                "fixed_size_shenanigans",
                                pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 3),
                                nullable=False,
                                metadata={},
                            ),
                        ]
                    ),
                    nullable=False,
                    metadata={},
                )
            ),
            "rerun.testing.components.AffixFuzzer17",
        )


class AffixFuzzer17Array(BaseExtensionArray[AffixFuzzer17ArrayLike]):
    _EXTENSION_NAME = "rerun.testing.components.AffixFuzzer17"
    _EXTENSION_TYPE = AffixFuzzer17Type

    @staticmethod
    def _native_to_pa_array(data: AffixFuzzer17ArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement native_to_pa_array_override in affix_fuzzer17_ext.py


AffixFuzzer17Type._ARRAY_TYPE = AffixFuzzer17Array

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(AffixFuzzer17Type())
