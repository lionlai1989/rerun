// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/ellipsoids3d.fbs".

#include "ellipsoids3d.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    Ellipsoids3D Ellipsoids3D::clear_fields() {
        auto archetype = Ellipsoids3D();
        archetype.half_sizes =
            ComponentBatch::empty<rerun::components::HalfSize3D>(Descriptor_half_sizes)
                .value_or_throw();
        archetype.centers =
            ComponentBatch::empty<rerun::components::PoseTranslation3D>(Descriptor_centers)
                .value_or_throw();
        archetype.rotation_axis_angles =
            ComponentBatch::empty<rerun::components::PoseRotationAxisAngle>(
                Descriptor_rotation_axis_angles
            )
                .value_or_throw();
        archetype.quaternions =
            ComponentBatch::empty<rerun::components::PoseRotationQuat>(Descriptor_quaternions)
                .value_or_throw();
        archetype.colors =
            ComponentBatch::empty<rerun::components::Color>(Descriptor_colors).value_or_throw();
        archetype.line_radii =
            ComponentBatch::empty<rerun::components::Radius>(Descriptor_line_radii)
                .value_or_throw();
        archetype.fill_mode =
            ComponentBatch::empty<rerun::components::FillMode>(Descriptor_fill_mode)
                .value_or_throw();
        archetype.labels =
            ComponentBatch::empty<rerun::components::Text>(Descriptor_labels).value_or_throw();
        archetype.show_labels =
            ComponentBatch::empty<rerun::components::ShowLabels>(Descriptor_show_labels)
                .value_or_throw();
        archetype.class_ids =
            ComponentBatch::empty<rerun::components::ClassId>(Descriptor_class_ids)
                .value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> Ellipsoids3D::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(11);
        if (half_sizes.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(half_sizes.value(), lengths_)
                                  .value_or_throw());
        }
        if (centers.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(centers.value(), lengths_).value_or_throw()
            );
        }
        if (rotation_axis_angles.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(rotation_axis_angles.value(), lengths_)
                    .value_or_throw()
            );
        }
        if (quaternions.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(quaternions.value(), lengths_)
                    .value_or_throw()
            );
        }
        if (colors.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(colors.value(), lengths_).value_or_throw()
            );
        }
        if (line_radii.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(line_radii.value(), lengths_)
                                  .value_or_throw());
        }
        if (fill_mode.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(fill_mode.value(), lengths_)
                                  .value_or_throw());
        }
        if (labels.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(labels.value(), lengths_).value_or_throw()
            );
        }
        if (show_labels.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(show_labels.value(), lengths_)
                    .value_or_throw()
            );
        }
        if (class_ids.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(class_ids.value(), lengths_)
                                  .value_or_throw());
        }
        columns.push_back(
            ComponentColumn::from_indicators<Ellipsoids3D>(static_cast<uint32_t>(lengths_.size()))
                .value_or_throw()
        );
        return columns;
    }

    Collection<ComponentColumn> Ellipsoids3D::columns() {
        if (half_sizes.has_value()) {
            return columns(std::vector<uint32_t>(half_sizes.value().length(), 1));
        }
        if (centers.has_value()) {
            return columns(std::vector<uint32_t>(centers.value().length(), 1));
        }
        if (rotation_axis_angles.has_value()) {
            return columns(std::vector<uint32_t>(rotation_axis_angles.value().length(), 1));
        }
        if (quaternions.has_value()) {
            return columns(std::vector<uint32_t>(quaternions.value().length(), 1));
        }
        if (colors.has_value()) {
            return columns(std::vector<uint32_t>(colors.value().length(), 1));
        }
        if (line_radii.has_value()) {
            return columns(std::vector<uint32_t>(line_radii.value().length(), 1));
        }
        if (fill_mode.has_value()) {
            return columns(std::vector<uint32_t>(fill_mode.value().length(), 1));
        }
        if (labels.has_value()) {
            return columns(std::vector<uint32_t>(labels.value().length(), 1));
        }
        if (show_labels.has_value()) {
            return columns(std::vector<uint32_t>(show_labels.value().length(), 1));
        }
        if (class_ids.has_value()) {
            return columns(std::vector<uint32_t>(class_ids.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::Ellipsoids3D>::serialize(
        const archetypes::Ellipsoids3D& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(11);

        if (archetype.half_sizes.has_value()) {
            cells.push_back(archetype.half_sizes.value());
        }
        if (archetype.centers.has_value()) {
            cells.push_back(archetype.centers.value());
        }
        if (archetype.rotation_axis_angles.has_value()) {
            cells.push_back(archetype.rotation_axis_angles.value());
        }
        if (archetype.quaternions.has_value()) {
            cells.push_back(archetype.quaternions.value());
        }
        if (archetype.colors.has_value()) {
            cells.push_back(archetype.colors.value());
        }
        if (archetype.line_radii.has_value()) {
            cells.push_back(archetype.line_radii.value());
        }
        if (archetype.fill_mode.has_value()) {
            cells.push_back(archetype.fill_mode.value());
        }
        if (archetype.labels.has_value()) {
            cells.push_back(archetype.labels.value());
        }
        if (archetype.show_labels.has_value()) {
            cells.push_back(archetype.show_labels.value());
        }
        if (archetype.class_ids.has_value()) {
            cells.push_back(archetype.class_ids.value());
        }
        {
            auto result = ComponentBatch::from_indicator<Ellipsoids3D>();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
