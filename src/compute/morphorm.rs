use crate::compute::{GenericAlgorithm, LayoutAlgorithm};
use crate::geometry::{Point, Rect, Size};
use crate::layout::{Layout, RunMode, SizeAndBaselines, SizingMode};
use crate::math::MaybeMath;
use crate::node::Node;
use crate::prelude::{TaffyMaxContent, TaffyMinContent};
use crate::resolve::{MaybeResolve, ResolveOrZero};
use crate::style::{
    AlignContent, AlignItems, AlignSelf, AvailableSpace, Dimension, Display, FlexWrap, JustifyContent,
    LengthPercentageAuto, Position,
};
use crate::style::{FlexDirection, Style};
use crate::sys::f32_max;
use crate::sys::Vec;
use crate::tree::LayoutTree;

/// The public interface to Taffy's Flexbox algorithm implementation
pub(crate) struct MorphormAlgorithm;
impl LayoutAlgorithm for MorphormAlgorithm {
    const NAME: &'static str = "MORPHORM";

    fn perform_layout(
        tree: &mut impl LayoutTree,
        node: Node,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> SizeAndBaselines {
        compute(tree, node, known_dimensions, parent_size, available_space, RunMode::PeformLayout)
    }

    fn measure_size(
        tree: &mut impl LayoutTree,
        node: Node,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> Size<f32> {
        compute(tree, node, known_dimensions, parent_size, available_space, RunMode::ComputeSize).size
    }
}


#[derive(Debug)]
pub struct ChildNode {
    // A reference to the node.
    node: Node,
    // The index of the node.
    index: usize,

    // The stretch factor of the node.
    stretch_factor: Size<f32>,
    // The minimum constraint of the node.
    min: Size<Option<f32>>,
    // The maximum constraint of the node.
    max: Size<Option<f32>>,

    // Sum of the flex factors on the main axis of the node.
    main_flex_sum: f32,
    // The available free space on the main axis of the node.
    main_non_flex: f32,
    // A remainder used during stretch computation.
    main_remainder: f32,
    // Sum of the cross_before, cross, and cross_after flex factors of the node.
    cross_flex_sum: f32,

    cross_non_flex: f32,
    cross: f32,
    cross_remainder: f32,

    // Computed main-before space of the node.
    main_before: f32,
    // Computed main-after space of the node.
    main_after: f32,
    // Computed cross-before space of the node.
    cross_before: f32,
    // Computed cross-after space of the node.
    cross_after: f32,
}

/// Computes the layout of [`LayoutTree`] according to the flexbox algorithm
pub fn compute(
    tree: &mut impl LayoutTree,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
) -> SizeAndBaselines {

  Size::zero().into()
}