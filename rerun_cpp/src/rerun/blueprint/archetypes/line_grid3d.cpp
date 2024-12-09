// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/line_grid3d.fbs".

#include "line_grid3d.hpp"

#include "../../collection_adapter_builtins.hpp"

namespace rerun::blueprint::archetypes {}

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<blueprint::archetypes::LineGrid3D>::serialize(
        const blueprint::archetypes::LineGrid3D& archetype
    ) {
        using namespace blueprint::archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(6);

        if (archetype.visible.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.visible.value(),
                ComponentDescriptor(
                    "rerun.blueprint.archetypes.LineGrid3D",
                    "visible",
                    "rerun.blueprint.components.Visible"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.spacing.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.spacing.value(),
                ComponentDescriptor(
                    "rerun.blueprint.archetypes.LineGrid3D",
                    "spacing",
                    "rerun.blueprint.components.GridSpacing"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.plane.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.plane.value(),
                ComponentDescriptor(
                    "rerun.blueprint.archetypes.LineGrid3D",
                    "plane",
                    "rerun.components.Plane3D"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.stroke_width.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.stroke_width.value(),
                ComponentDescriptor(
                    "rerun.blueprint.archetypes.LineGrid3D",
                    "stroke_width",
                    "rerun.components.StrokeWidth"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.color.has_value()) {
            auto result = ComponentBatch::from_loggable(
                archetype.color.value(),
                ComponentDescriptor(
                    "rerun.blueprint.archetypes.LineGrid3D",
                    "color",
                    "rerun.components.Color"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = LineGrid3D::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
