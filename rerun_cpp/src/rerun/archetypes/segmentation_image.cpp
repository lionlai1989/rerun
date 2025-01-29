// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/segmentation_image.fbs".

#include "segmentation_image.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    SegmentationImage SegmentationImage::clear_fields() {
        auto archetype = SegmentationImage();
        archetype.buffer = ComponentBatch::empty<rerun::components::ImageBuffer>(Descriptor_buffer)
                               .value_or_throw();
        archetype.format = ComponentBatch::empty<rerun::components::ImageFormat>(Descriptor_format)
                               .value_or_throw();
        archetype.opacity =
            ComponentBatch::empty<rerun::components::Opacity>(Descriptor_opacity).value_or_throw();
        archetype.draw_order =
            ComponentBatch::empty<rerun::components::DrawOrder>(Descriptor_draw_order)
                .value_or_throw();
        return archetype;
    }

    Collection<ComponentColumn> SegmentationImage::columns(const Collection<uint32_t>& lengths_) {
        std::vector<ComponentColumn> columns;
        columns.reserve(5);
        if (buffer.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(buffer.value(), lengths_).value_or_throw()
            );
        }
        if (format.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(format.value(), lengths_).value_or_throw()
            );
        }
        if (opacity.has_value()) {
            columns.push_back(
                ComponentColumn::from_batch_with_lengths(opacity.value(), lengths_).value_or_throw()
            );
        }
        if (draw_order.has_value()) {
            columns.push_back(ComponentColumn::from_batch_with_lengths(draw_order.value(), lengths_)
                                  .value_or_throw());
        }
        columns.push_back(ComponentColumn::from_indicators<SegmentationImage>(
                              static_cast<uint32_t>(lengths_.size())
        )
                              .value_or_throw());
        return columns;
    }

    Collection<ComponentColumn> SegmentationImage::columns() {
        if (buffer.has_value()) {
            return columns(std::vector<uint32_t>(buffer.value().length(), 1));
        }
        if (format.has_value()) {
            return columns(std::vector<uint32_t>(format.value().length(), 1));
        }
        if (opacity.has_value()) {
            return columns(std::vector<uint32_t>(opacity.value().length(), 1));
        }
        if (draw_order.has_value()) {
            return columns(std::vector<uint32_t>(draw_order.value().length(), 1));
        }
        return Collection<ComponentColumn>();
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::SegmentationImage>::serialize(
        const archetypes::SegmentationImage& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(5);

        if (archetype.buffer.has_value()) {
            cells.push_back(archetype.buffer.value());
        }
        if (archetype.format.has_value()) {
            cells.push_back(archetype.format.value());
        }
        if (archetype.opacity.has_value()) {
            cells.push_back(archetype.opacity.value());
        }
        if (archetype.draw_order.has_value()) {
            cells.push_back(archetype.draw_order.value());
        }
        {
            auto result = ComponentBatch::from_indicator<SegmentationImage>();
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
