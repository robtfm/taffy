#[test]
#[allow(non_snake_case)]
fn bevy_issue_16304__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node0000 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0001 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0002 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0003 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0004 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0005 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0006 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0007 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0008 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0009 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node000 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                ..Default::default()
            },
            &[node0000, node0001, node0002, node0003, node0004, node0005, node0006, node0007, node0008, node0009],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Length(400f32), height: auto() },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(400f32), height: auto() },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(400f32),
                    height: taffy::style::Dimension::Length(110f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 110f32, "height of node {:?}. Expected {}. Actual {}", node, 110f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node0, 400f32, size.width);
    assert_eq!(size.height, 110f32, "height of node {:?}. Expected {}. Actual {}", node0, 110f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node00, 400f32, size.width);
    assert_eq!(size.height, 16f32, "height of node {:?}. Expected {}. Actual {}", node00, 16f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node000, 160f32, size.width);
    assert_eq!(size.height, 16f32, "height of node {:?}. Expected {}. Actual {}", node000, 16f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let layout = taffy.layout(node0000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0000, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0000, 10f32, size.height);
    assert_eq!(location.x, 3f32, "x of node {:?}. Expected {}. Actual {}", node0000, 3f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0000, 3f32, location.y);
    let layout = taffy.layout(node0001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0001, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0001, 10f32, size.height);
    assert_eq!(location.x, 19f32, "x of node {:?}. Expected {}. Actual {}", node0001, 19f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0001, 3f32, location.y);
    let layout = taffy.layout(node0002).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0002, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0002, 10f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node0002, 35f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0002, 3f32, location.y);
    let layout = taffy.layout(node0003).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0003, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0003, 10f32, size.height);
    assert_eq!(location.x, 51f32, "x of node {:?}. Expected {}. Actual {}", node0003, 51f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0003, 3f32, location.y);
    let layout = taffy.layout(node0004).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0004, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0004, 10f32, size.height);
    assert_eq!(location.x, 67f32, "x of node {:?}. Expected {}. Actual {}", node0004, 67f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0004, 3f32, location.y);
    let layout = taffy.layout(node0005).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0005, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0005, 10f32, size.height);
    assert_eq!(location.x, 83f32, "x of node {:?}. Expected {}. Actual {}", node0005, 83f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0005, 3f32, location.y);
    let layout = taffy.layout(node0006).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0006, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0006, 10f32, size.height);
    assert_eq!(location.x, 99f32, "x of node {:?}. Expected {}. Actual {}", node0006, 99f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0006, 3f32, location.y);
    let layout = taffy.layout(node0007).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0007, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0007, 10f32, size.height);
    assert_eq!(location.x, 115f32, "x of node {:?}. Expected {}. Actual {}", node0007, 115f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0007, 3f32, location.y);
    let layout = taffy.layout(node0008).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0008, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0008, 10f32, size.height);
    assert_eq!(location.x, 131f32, "x of node {:?}. Expected {}. Actual {}", node0008, 131f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0008, 3f32, location.y);
    let layout = taffy.layout(node0009).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0009, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0009, 10f32, size.height);
    assert_eq!(location.x, 147f32, "x of node {:?}. Expected {}. Actual {}", node0009, 147f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0009, 3f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn bevy_issue_16304__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout, TaffyTree};
    let mut taffy: TaffyTree<crate::TextMeasure> = TaffyTree::new();
    let node0000 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0001 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0002 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0003 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0004 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0005 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0006 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0007 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0008 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0009 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(10f32),
                height: taffy::style::Dimension::Length(10f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(3f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(3f32),
                bottom: taffy::style::LengthPercentageAuto::Length(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node000 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                ..Default::default()
            },
            &[node0000, node0001, node0002, node0003, node0004, node0005, node0006, node0007, node0008, node0009],
        )
        .unwrap();
    let node00 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Length(400f32), height: auto() },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(400f32), height: auto() },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(400f32),
                    height: taffy::style::Dimension::Length(110f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 110f32, "height of node {:?}. Expected {}. Actual {}", node, 110f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node0, 400f32, size.width);
    assert_eq!(size.height, 110f32, "height of node {:?}. Expected {}. Actual {}", node0, 110f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node00, 400f32, size.width);
    assert_eq!(size.height, 16f32, "height of node {:?}. Expected {}. Actual {}", node00, 16f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let layout = taffy.layout(node000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node000, 160f32, size.width);
    assert_eq!(size.height, 16f32, "height of node {:?}. Expected {}. Actual {}", node000, 16f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node000, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node000, 0f32, location.y);
    let layout = taffy.layout(node0000).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0000, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0000, 10f32, size.height);
    assert_eq!(location.x, 3f32, "x of node {:?}. Expected {}. Actual {}", node0000, 3f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0000, 3f32, location.y);
    let layout = taffy.layout(node0001).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0001, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0001, 10f32, size.height);
    assert_eq!(location.x, 19f32, "x of node {:?}. Expected {}. Actual {}", node0001, 19f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0001, 3f32, location.y);
    let layout = taffy.layout(node0002).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0002, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0002, 10f32, size.height);
    assert_eq!(location.x, 35f32, "x of node {:?}. Expected {}. Actual {}", node0002, 35f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0002, 3f32, location.y);
    let layout = taffy.layout(node0003).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0003, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0003, 10f32, size.height);
    assert_eq!(location.x, 51f32, "x of node {:?}. Expected {}. Actual {}", node0003, 51f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0003, 3f32, location.y);
    let layout = taffy.layout(node0004).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0004, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0004, 10f32, size.height);
    assert_eq!(location.x, 67f32, "x of node {:?}. Expected {}. Actual {}", node0004, 67f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0004, 3f32, location.y);
    let layout = taffy.layout(node0005).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0005, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0005, 10f32, size.height);
    assert_eq!(location.x, 83f32, "x of node {:?}. Expected {}. Actual {}", node0005, 83f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0005, 3f32, location.y);
    let layout = taffy.layout(node0006).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0006, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0006, 10f32, size.height);
    assert_eq!(location.x, 99f32, "x of node {:?}. Expected {}. Actual {}", node0006, 99f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0006, 3f32, location.y);
    let layout = taffy.layout(node0007).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0007, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0007, 10f32, size.height);
    assert_eq!(location.x, 115f32, "x of node {:?}. Expected {}. Actual {}", node0007, 115f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0007, 3f32, location.y);
    let layout = taffy.layout(node0008).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0008, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0008, 10f32, size.height);
    assert_eq!(location.x, 131f32, "x of node {:?}. Expected {}. Actual {}", node0008, 131f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0008, 3f32, location.y);
    let layout = taffy.layout(node0009).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0009, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0009, 10f32, size.height);
    assert_eq!(location.x, 147f32, "x of node {:?}. Expected {}. Actual {}", node0009, 147f32, location.x);
    assert_eq!(location.y, 3f32, "y of node {:?}. Expected {}. Actual {}", node0009, 3f32, location.y);
}
