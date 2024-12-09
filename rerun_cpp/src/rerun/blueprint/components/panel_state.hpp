// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/components/panel_state.fbs".

#pragma once

#include "../../component_descriptor.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    /// \private
    template <typename T>
    class NumericBuilder;

    class Array;
    class DataType;
    class UInt8Type;
    using UInt8Builder = NumericBuilder<UInt8Type>;
} // namespace arrow

namespace rerun::blueprint::components {
    /// **Component**: Tri-state for panel controls.
    enum class PanelState : uint8_t {

        /// Completely hidden.
        Hidden = 1,

        /// Visible, but as small as possible on its shorter axis.
        Collapsed = 2,

        /// Fully expanded.
        Expanded = 3,
    };
} // namespace rerun::blueprint::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<blueprint::components::PanelState> {
        static constexpr ComponentDescriptor Descriptor = "rerun.blueprint.components.PanelState";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::blueprint:: components::PanelState` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const blueprint::components::PanelState* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::UInt8Builder* builder, const blueprint::components::PanelState* elements,
            size_t num_elements
        );
    };
} // namespace rerun
