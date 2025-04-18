# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/views/text_document.fbs".

from __future__ import annotations

from collections.abc import Iterable, Mapping

from ..._baseclasses import (
    DescribedComponentBatch,
)

__all__ = ["TextDocumentView"]


from ... import datatypes
from ..._baseclasses import AsComponents
from ...datatypes import EntityPathLike, Utf8Like
from ..api import View, ViewContentsLike


class TextDocumentView(View):
    """
    **View**: A view of a single text document, for use with [`archetypes.TextDocument`][rerun.archetypes.TextDocument].

    ⚠️ **This type is _unstable_ and may change significantly in a way that the data won't be backwards compatible.**

    Example
    -------
    ### Use a blueprint to show a text document.:
    ```python
    import rerun as rr
    import rerun.blueprint as rrb

    rr.init("rerun_example_text_document", spawn=True)

    rr.log(
        "markdown",
        rr.TextDocument(
            '''
    # Hello Markdown!
    [Click here to see the raw text](recording://markdown:Text).

    Basic formatting:

    | **Feature**       | **Alternative** |
    | ----------------- | --------------- |
    | Plain             |                 |
    | *italics*         | _italics_       |
    | **bold**          | __bold__        |
    | ~~strikethrough~~ |                 |
    | `inline code`     |                 |

    ----------------------------------

    ## Support
    - [x] [Commonmark](https://commonmark.org/help/) support
    - [x] GitHub-style strikethrough, tables, and checkboxes
    - Basic syntax highlighting for:
      - [x] C and C++
      - [x] Python
      - [x] Rust
      - [ ] Other languages

    ## Links
    You can link to [an entity](recording://markdown),
    a [specific instance of an entity](recording://markdown[#0]),
    or a [specific component](recording://markdown:Text).

    Of course you can also have [normal https links](https://github.com/rerun-io/rerun), e.g. <https://rerun.io>.

    ## Image
    ![A random image](https://picsum.photos/640/480)
    '''.strip(),
            media_type=rr.MediaType.MARKDOWN,
        ),
    )

    # Create a text view that displays the markdown.
    blueprint = rrb.Blueprint(rrb.TextDocumentView(origin="markdown", name="Markdown example"), collapse_panels=True)

    rr.send_blueprint(blueprint)
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/text_log/27f15235fe9639ff42b6ea0d2f0ce580685c021c/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/text_log/27f15235fe9639ff42b6ea0d2f0ce580685c021c/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/text_log/27f15235fe9639ff42b6ea0d2f0ce580685c021c/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/text_log/27f15235fe9639ff42b6ea0d2f0ce580685c021c/1200w.png">
      <img src="https://static.rerun.io/text_log/27f15235fe9639ff42b6ea0d2f0ce580685c021c/full.png" width="640">
    </picture>
    </center>

    """

    def __init__(
        self,
        *,
        origin: EntityPathLike = "/",
        contents: ViewContentsLike = "$origin/**",
        name: Utf8Like | None = None,
        visible: datatypes.BoolLike | None = None,
        defaults: Iterable[AsComponents | Iterable[DescribedComponentBatch]] | None = None,
        overrides: Mapping[
            EntityPathLike,
            AsComponents | Iterable[DescribedComponentBatch | AsComponents | Iterable[DescribedComponentBatch]],
        ]
        | None = None,
    ) -> None:
        """
        Construct a blueprint for a new TextDocumentView view.

        Parameters
        ----------
        origin:
            The `EntityPath` to use as the origin of this view.
            All other entities will be transformed to be displayed relative to this origin.
        contents:
            The contents of the view specified as a query expression.
            This is either a single expression, or a list of multiple expressions.
            See [rerun.blueprint.archetypes.ViewContents][].
        name:
            The display name of the view.
        visible:
            Whether this view is visible.

            Defaults to true if not specified.
        defaults:
            List of archetypes or (described) component batches to add to the view.
            When an archetype in the view is missing a component included in this set,
            the value of default will be used instead of the normal fallback for the visualizer.

            Note that an archetype's required components typically don't have any effect.
            It is recommended to use the archetype's `from_fields` method instead and only specify the fields that you need.
        overrides:
            Dictionary of overrides to apply to the view. The key is the path to the entity where the override
            should be applied. The value is a list of archetypes or (described) component batches to apply to the entity.

            It is recommended to use the archetype's `from_fields` method instead and only specify the fields that you need.

            Important note: the path must be a fully qualified entity path starting at the root. The override paths
            do not yet support `$origin` relative paths or glob expressions.
            This will be addressed in <https://github.com/rerun-io/rerun/issues/6673>.

        """

        properties: dict[str, AsComponents] = {}
        super().__init__(
            class_identifier="TextDocument",
            origin=origin,
            contents=contents,
            name=name,
            visible=visible,
            properties=properties,
            defaults=defaults,
            overrides=overrides,
        )
