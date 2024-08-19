use bevy::{
    prelude::*,
    ui::{BorderRadius, FocusPolicy, Interaction, UiRect, Val},
};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use regex::Regex;

use crate::utils::{colors::SubmergeColors, font_size::SubmergeText};

use super::style_bundles::GeneralStyle;

#[derive(Parser)]
#[grammar = "rules.pest"]
struct StyleParser;

pub fn parser<'a>(style: &'a str, bundle: &'a mut GeneralStyle) {
    let pairs =
        StyleParser::parse(Rule::style_definition, style).unwrap_or_else(|e| panic!("{}", e));

    // println!("pair outer: {:?}", pairs);
    for pair in pairs.into_iter() {
        handler(pair, bundle);
    }
}

fn extract_number(value: &str) -> Option<f32> {
    // Define a regex pattern to capture the number part of the string
    let re = Regex::new(r"^(\d+(\.\d+)?)").unwrap();

    // Find the first match for the regex pattern
    if let Some(captures) = re.captures(value) {
        // Extract the captured group and convert it to f32
        if let Some(matched) = captures.get(1) {
            return matched.as_str().parse::<f32>().ok();
        }
    }
    None
}

fn parse_vec(input: &str) -> Vec<f32> {
    // Split the input by underscores
    let parts: Vec<&str> = input.split('_').collect();

    // Convert the string parts to f32 and collect them into a vector
    let vec: Vec<f32> = parts
        .iter()
        .filter_map(|&part| part.parse::<f32>().ok()) // Convert to f32, skipping invalid parts
        .collect();

    vec
}

fn handler<'a>(pair: Pair<'a, Rule>, bundle: &'a mut GeneralStyle) -> Option<&'a str> {
    let rule = pair.as_rule();
    match rule {
        //Property rules -- Rules that adjust base rules in style bundles
        //OR can be reffered to as rules that manipulate styles not in the Styles struct
        Rule::style_definition => None,
        Rule::style => None,
        Rule::border_radius => {
            let mut inner_pairs = pair.clone().into_inner();

            // Get the first inner pair
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.border_radius = BorderRadius::all(bundle.val);

            None
        }
        Rule::background_color => {
            let inner_pair = pair.clone().into_inner().next().unwrap();

            if inner_pair.as_rule() == Rule::COLOR_BASE {
                let color =
                    SubmergeColors::get_color_from_style_text(inner_pair.as_span().as_str());
                bundle.background_color = BackgroundColor(color);
            } else {
                handler(inner_pair, bundle);
                bundle.background_color = BackgroundColor(bundle.color);
            }

            None
        }
        Rule::border_color => {
            let inner_pair = pair.clone().into_inner().next().unwrap();

            if inner_pair.as_rule() == Rule::COLOR_BASE {
                let color =
                    SubmergeColors::get_color_from_style_text(inner_pair.as_span().as_str());
                bundle.border_color = BorderColor(color);
            } else {
                handler(inner_pair, bundle);
                bundle.border_color = BorderColor(bundle.color);
            }

            None
        }
        Rule::text_color => {
            let inner_pair = pair.clone().into_inner().next().unwrap();

            if inner_pair.as_rule() == Rule::COLOR_BASE {
                let color =
                    SubmergeColors::get_color_from_style_text(inner_pair.as_span().as_str());
                bundle.text_color = Some(color);
            } else {
                handler(inner_pair, bundle);
                bundle.text_color = Some(bundle.color);
            }

            None
        }
        Rule::text_size => {
            let mut inner_pairs = pair.clone().into_inner();

            if let Some(value) = inner_pairs.next() {
                if let Some(text_size) = extract_number(value.as_span().as_str()) {
                    bundle.text_size = Some(text_size);
                }
            } else {
                let style = pair.as_span().as_str();
                bundle.text_size = Some(SubmergeText::get_size_from_text_style(style));
            }

            None
        }
        Rule::interaction => {
            let mut inner_pairs = pair.clone().into_inner();

            if let Some(next_pair) = inner_pairs.next() {
                if let Some(value) = handler(next_pair, bundle) {
                    match value {
                        "pressed" => bundle.interaction = Some(Interaction::Pressed),
                        "hovered" => bundle.interaction = Some(Interaction::Hovered),
                        "none" => bundle.interaction = Some(Interaction::None),
                        _ => eprint!("Invalid interaction value. Interaction value can be either 'pressed' | 'hovered' | 'none'")
                    }
                }
            }
            None
        }
        Rule::focus_policy => {
            let mut inner_pairs = pair.clone().into_inner();

            if let Some(next_pair) = inner_pairs.next() {
                if let Some(value) = handler(next_pair, bundle) {
                    match value {
                        "block" => bundle.focus_policy = FocusPolicy::Block,
                        "pass" => bundle.focus_policy = FocusPolicy::Pass,
                        _ => eprint!("Invalid focus_policy value. FocusPolicy value can be either 'block' | 'pass' | 'none'")
                    }
                }
            }

            None
        }

        //todo: handle later
        Rule::image => None,
        //end-todo:
        Rule::transform => {
            let values: Vec<&str> = pair.as_span().as_str().split("-").collect();

            match values[1] {
                "t" => {
                    let val = parse_vec(values[2]);
                    bundle.transform.translation = Vec3::from_slice(&val);
                },
                "r" => {
                    let val = parse_vec(values[2]);
                    bundle.transform.rotation = Quat::from_xyzw(val[0], val[1], val[2], val[3]);
                },
                "s" => {
                    let val = parse_vec(values[2]);
                    bundle.transform.scale = Vec3::from_slice(&val);
                },
                _ => eprintln!("Invalid transform value. Use transform-t (for translate) | transform-r (for rotate) | transform-s (for scale)")
            }

            None
        }

        //todo: handle later
        Rule::global_transform => None,
        //end-todo:

        //cross check everything above
        Rule::visibility => {
            let value: Vec<&str> = pair.as_span().as_str().split("-").collect();
            match value[1] {
                "visible" => bundle.visibility = Visibility::Visible,
                "inherit" => bundle.visibility = Visibility::Inherited,
                "hidden" => bundle.visibility = Visibility::Hidden,
                _ => eprint!("Invalid visibility value. Visibility value can be either 'visible' | 'hidden' | 'inherit'")
            }

            None
        }
        Rule::inherited_visibility => None, //remove later field is private
        Rule::view_visibility => None,      //remove later field is private
        Rule::z_index => {
            let value: Vec<&str> = pair.as_span().as_str().split("-").collect();
            if value.len() == 2 {
                let num = value[1].parse::<i32>();
                if let Ok(number) = num {
                    bundle.z_index = ZIndex::Local(number);
                } else {
                    eprintln!("Invalid value for z_index. Make sure value is not a float and is in format z-10 for local z-index and z-global-10 for global");
                }
            } else {
                let num = value[2].parse::<i32>();
                if let Ok(number) = num {
                    bundle.z_index = ZIndex::Global(number);
                } else {
                    eprintln!("Invalid value for z_index. Make sure value is not a float and is in format z-10 for local z-index and z-global-10 for global");
                }
            }
            None
        }

        //Basic Rules -- Rules used to create other rules
        Rule::VALUE => {
            let value = pair.as_span().as_str();
            if value.contains("px") {
                let number = extract_number(value).unwrap_or_else(|| {
                    eprintln!("Border value '{}' doesn't include a number", value);
                    1.0 // Default value for "px"
                });
                bundle.val = Val::Px(number);
            } else if value.contains("vw") {
                let number = extract_number(value).unwrap_or_else(|| {
                    eprintln!("Border value '{}' doesn't include a number", value);
                    1.0 // Default value for "vw"
                });
                bundle.val = Val::Percent(number);
            } else if value.contains("vh") {
                let number = extract_number(value).unwrap_or_else(|| {
                    eprintln!("Border value '{}' doesn't include a number", value);
                    1.0 // Default value for "vh"
                });
                bundle.val = Val::Percent(number);
            } else if value.contains("%") {
                let number = extract_number(value).unwrap_or_else(|| {
                    eprintln!("Border value '{}' doesn't include a number", value);
                    1.0 // Default value for "%"
                });
                bundle.val = Val::Percent(number);
            } else {
                eprint!("Invalid unit value.");
            }
            None
        }
        Rule::UNIT => None,
        Rule::NUMBER => None,
        Rule::PREDEFINED_VALUES => None,
        Rule::DISPLAY => None,
        Rule::POSITION_TYPE => None,
        Rule::OVERFLOW => None,
        Rule::DIRECTION => None,
        Rule::ALIGN_ITEMS => None,
        Rule::JUSTIFY_ITEMS => None,
        Rule::ALIGN_SELF => None,
        Rule::JUSTIFY_SELF => None,
        Rule::ALIGN_CONTENT => None,
        Rule::JUSTIFY_CONTENT => None,
        Rule::FLEX_DIRECTION => None,
        Rule::FLEX_WRAP => None,
        Rule::GRID_AUTO_FLOW => {
            let mut inner_pairs = pair.clone().into_inner();

            // Get the first inner pair
            if let Some(next_pair) = inner_pairs.next() {
                handler(next_pair, bundle)
            } else {
                // println!("pair: {:?}", pair);
                let value = pair.as_span().as_str();
                match value {
                    "col" => bundle.grid_auto_flow = GridAutoFlow::Column,
                    "col-dense" => bundle.grid_auto_flow = GridAutoFlow::ColumnDense,
                    "row" => bundle.grid_auto_flow = GridAutoFlow::Row,
                    "row-dense" => bundle.grid_auto_flow = GridAutoFlow::RowDense,
                    _ => eprint!("Invalid grid_auto_flow value. GridAutoFlow value can be either 'col' | 'row' | 'col-dense' | 'row-dense'"),
                }
                None
            }
        }
        Rule::COLOR_BASE => None,

        //todo: finish up
        Rule::GRID_TEMPLATE_ROWS => None,
        Rule::GRID_TEMPLATE_COLUMNS => None,
        Rule::GRID_AUTO_ROWS => None,
        Rule::GRID_AUTO_COLUMNS => None,
        Rule::GRID_PLACEMENT => None,
        //end-todo:
        Rule::COLOR => {
            let mut inner_pairs = pair.into_inner();

            let mut color_name = None;
            let mut color_value = None;

            // Loop through inner pairs to get COLOR_NAME and COLOR_VALUE
            while let Some(inner_pair) = inner_pairs.next() {
                match inner_pair.as_rule() {
                    Rule::COLOR_NAME => {
                        color_name = Some(inner_pair.as_span().as_str());
                        // println!("color_name: {:?}", color_name);
                    }
                    Rule::COLOR_VALUE => {
                        color_value = Some(inner_pair.as_span().as_str());
                        // println!("color_value: {:?}", color_value);
                    }
                    _ => {}
                }
            }

            // Handle the combined color logic here (if necessary)
            if let (Some(name), Some(value)) = (color_name, color_value) {
                // Perform any logic you need with the extracted color_name and color_value
                let color =
                    SubmergeColors::get_color_from_style_text(&format!("{}-{}", name, value));
                bundle.color = color;
                // println!("Combined color: {}-{}, Color: {:?}", name, value, color);
            }

            None // Return appropriate value if needed
        }
        Rule::COLOR_NAME => None,
        Rule::COLOR_VALUE => None,
        Rule::DIGIT => None,
        Rule::DECIMAL => None,
        Rule::VEC2 => None,
        Rule::VEC3 => None,
        Rule::VEC4 => None,
        Rule::BVEC4 => {
            let bvec4: Vec<Val> = pair
                .as_span()
                .as_str()
                .split("_")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| {
                    let number = if x.contains("px") {
                        Val::Px(extract_number(*x).unwrap())
                    } else if x.contains("vw") {
                        Val::Vw(extract_number(*x).unwrap())
                    } else if x.contains("vh") {
                        Val::Vh(extract_number(*x).unwrap())
                    } else if x.contains("%") {
                        Val::Percent(extract_number(*x).unwrap())
                    } else {
                        Val::default()
                    };
                    number
                })
                .collect();

            bundle.bvec = bvec4;
            None
        }

        //Style Rules -- Rules that manipulate the Style struct
        Rule::display_style => {
            let value = pair.as_span().as_str();
            match value {
                "display-flex" => {
                    bundle.style.display = Display::Flex;
                }
                "display-grid" => {
                    bundle.style.display = Display::Grid;
                }
                "display-none" => {
                    bundle.style.display = Display::None;
                }
                _ => {
                    eprintln!("Invalid display value. Display value can be either 'flex' | 'grid' | 'none'")
                }
            }

            None
        }
        Rule::position_type_style => {
            let value = pair.as_span().as_str();
            match value {
                    "position-relative" => bundle.style.position_type = PositionType::Relative,
                    "position-absolute" => bundle.style.position_type = PositionType::Absolute,
                    _ => eprint!("Invalid position type value. Position Type value can be either 'relative' | 'absolute' "),
                }
            None
        }
        Rule::overflow_style => {
            let value = pair.as_span().as_str();
            match value {
                    "overflow-visible" => bundle.style.overflow = Overflow::visible(),
                    "overflow-clip" => bundle.style.overflow = Overflow::clip(),
                    "overflow-clip-y" => bundle.style.overflow = Overflow::clip_y(),
                    "overflow-clip-x" => bundle.style.overflow = Overflow::clip_x(),
                    _ => eprint!("Invalid overflow value. Overflow value can be either 'visible' | 'clip' | 'clip-x' | 'clip-y'"),
                }
            None
        }
        Rule::direction_style => {
            let value = pair.as_span().as_str();
            match value {
                    "direction-inherit" => bundle.style.direction = Direction::Inherit,
                    "direction-ltr" => bundle.style.direction = Direction::LeftToRight,
                    "direction-rtl" => bundle.style.direction = Direction::RightToLeft,
                    _ => eprint!("Invalid direction value. Direction value can be either 'inherit' | 'ltr' | 'rtl'"),
                }
            None
        }
        Rule::left_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.left = bundle.val;
            None
        }
        Rule::right_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.right = bundle.val;
            None
        }
        Rule::top_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.top = bundle.val;
            None
        }
        Rule::bottom_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.bottom = bundle.val;
            None
        }
        Rule::width_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.width = bundle.val;
            None
        }
        Rule::height_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.height = bundle.val;
            None
        }
        Rule::min_width_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.min_width = bundle.val;
            None
        }
        Rule::min_height_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.min_height = bundle.val;
            None
        }
        Rule::max_width_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.max_width = bundle.val;
            None
        }
        Rule::max_height_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.max_height = bundle.val;
            None
        }
        Rule::aspect_ratio_style => {
            let value = pair.as_span().as_str().split("-").collect::<Vec<&str>>()[1];
            bundle.style.aspect_ratio = extract_number(value);

            None
        }
        Rule::align_items_style => {
            let value = pair.as_span().as_str();
            match value {
                "align_items-start" => bundle.style.align_items = AlignItems::Start,
                "align_items-end" => bundle.style.align_items = AlignItems::End,
                "align_items-stretch" => bundle.style.align_items = AlignItems::Stretch,
                "align_items-center" => bundle.style.align_items = AlignItems::Center,
                "align_items-flex-start" => bundle.style.align_items = AlignItems::FlexStart,
                "align_items-flex-end" => bundle.style.align_items = AlignItems::FlexEnd,
                "align_items-basline" => bundle.style.align_items = AlignItems::Baseline,
                "align_items-default" => bundle.style.align_items = AlignItems::Default,
                _ => eprint!("Invalid align_items value. AlignItems value can be either 'start' | 'end' | 'stretch' | 'center' | 'flex-start' | 'flex-end' | 'baseline' | 'default'"),
            }

            None
        }
        Rule::justify_items_style => {
            let value = pair.as_span().as_str();
            match value {
                "justify_items-start" => bundle.style.justify_items = JustifyItems::Start,
                "justify_items-end" => bundle.style.justify_items = JustifyItems::End,
                "justify_items-stretch" => bundle.style.justify_items = JustifyItems::Stretch,
                "justify_items-center" => bundle.style.justify_items = JustifyItems::Center,
                "justify_items-baseline" => bundle.style.justify_items = JustifyItems::Baseline,
                "justify_items-default" => bundle.style.justify_items = JustifyItems::Default,
                _ => eprint!("Invalid justify_items value. JustifyItems value can be either 'start' | 'end' | 'stretch' | 'center' | 'baseline' | 'default'"),
                }
            None
        }
        Rule::align_self_style => {
            let value = pair.as_span().as_str();
            match value {
                "align_self-start" => bundle.style.align_self = AlignSelf::Start,
                "align_self-end" => bundle.style.align_self = AlignSelf::End,
                "align_self-stretch" => bundle.style.align_self = AlignSelf::Stretch,
                "align_self-center" => bundle.style.align_self = AlignSelf::Center,
                "align_self-flex-start" => bundle.style.align_self = AlignSelf::FlexStart,
                "align_self-flex-end" => bundle.style.align_self = AlignSelf::FlexEnd,
                "align_self-basline" => bundle.style.align_self = AlignSelf::Baseline,
                "align_self-auto" => bundle.style.align_self = AlignSelf::Auto,
                _ => eprint!("Invalid align_self value. AlignSelf value can be either 'start' | 'end' | 'stretch' | 'center' | 'flex-start' | 'flex-end' | 'baseline' | 'auto'"),
            }
            None
        }
        Rule::justify_self_style => {
            let value = pair.as_span().as_str();
            match value {
                "justify_self-start" => bundle.style.justify_self = JustifySelf::Start,
                "justify_self-end" => bundle.style.justify_self = JustifySelf::End,
                "justify_self-stretch" => bundle.style.justify_self = JustifySelf::Stretch,
                "justify_self-center" => bundle.style.justify_self = JustifySelf::Center,
                "justify_self-baseline" => bundle.style.justify_self = JustifySelf::Baseline,
                "justify_self-default" => bundle.style.justify_self = JustifySelf::Auto,
                _ => eprint!("Invalid justify_self value. JustifySelf value can be either 'start' | 'end' | 'stretch' | 'center' | 'baseline' | 'auto'"),
            }
            None
        }
        Rule::align_content_style => {
            let value = pair.as_span().as_str();
            match value {
                "align_content-start" => bundle.style.align_content = AlignContent::Start,
                "align_content-end" => bundle.style.align_content = AlignContent::End,
                "align_content-stretch" => bundle.style.align_content = AlignContent::Stretch,
                "align_content-center" => bundle.style.align_content = AlignContent::Center,
                "align_content-flex-start" => bundle.style.align_content = AlignContent::FlexStart,
                "align_content-flex-end" => bundle.style.align_content = AlignContent::FlexEnd,
                "align_content-between" => bundle.style.align_content = AlignContent::SpaceBetween,
                "align_content-evenly" => bundle.style.align_content = AlignContent::SpaceEvenly,
                "align_content-around" => bundle.style.align_content = AlignContent::SpaceAround,
                "align_content-default" => bundle.style.align_content = AlignContent::Default,
                _ => eprint!("Invalid align_content value. AlignContent value can be either 'start' | 'end' | 'stretch' | 'center' | 'flex-start' | 'flex-end' | 'between' | 'evenly' | 'around' | 'default'"),
            }
            None
        }
        Rule::justify_content_style => {
            let value = pair.as_span().as_str();
            match value {
                "justify_content-start" => bundle.style.justify_content = JustifyContent::Start,
                "justify_content-end" => bundle.style.justify_content = JustifyContent::End,
                "justify_content-stretch" => bundle.style.justify_content = JustifyContent::Stretch,
                "justify_content-center" => bundle.style.justify_content = JustifyContent::Center,
                "justify_content-flex-start" => bundle.style.justify_content = JustifyContent::FlexStart,
                "justify_content-flex-end" => bundle.style.justify_content = JustifyContent::FlexEnd,
                "justify_content-between" => bundle.style.justify_content = JustifyContent::SpaceBetween,
                "justify_content-evenly" => bundle.style.justify_content = JustifyContent::SpaceEvenly,
                "justify_content-around" => bundle.style.justify_content = JustifyContent::SpaceAround,
                "justify_content-default" => bundle.style.justify_content = JustifyContent::Default,
                _ => eprint!("Invalid justify_content value. JustifyContent value can be either 'start' | 'end' | 'stretch' | 'center' | 'flex-start' | 'flex-end' | 'between' | 'evenly' | 'around' | 'default'"),
            }
            None
        }

        //todo: modify border to accept a vec2 for top/bottom, left/right
        Rule::border_style => {
            let inner_pairs = pair.clone().into_inner().next().unwrap();

            handler(inner_pairs.clone(), bundle);

            if inner_pairs.as_rule() == Rule::BVEC4 {
                bundle.style.border = UiRect::new(
                    bundle.bvec[0],
                    bundle.bvec[1],
                    bundle.bvec[2],
                    bundle.bvec[3],
                );
            } else if inner_pairs.as_rule() == Rule::VALUE {
                bundle.style.border = UiRect::all(bundle.val);
            }

            None
        }
        Rule::margin_style => {
            let inner_pairs = pair.clone().into_inner().next().unwrap();

            handler(inner_pairs.clone(), bundle);

            if inner_pairs.as_rule() == Rule::BVEC4 {
                bundle.style.border = UiRect::new(
                    bundle.bvec[0],
                    bundle.bvec[1],
                    bundle.bvec[2],
                    bundle.bvec[3],
                );
            } else if inner_pairs.as_rule() == Rule::VALUE {
                bundle.style.margin = UiRect::all(bundle.val);
            }

            None
        }
        Rule::padding_style => {
            let inner_pairs = pair.clone().into_inner().next().unwrap();

            handler(inner_pairs.clone(), bundle);

            if inner_pairs.as_rule() == Rule::BVEC4 {
                bundle.style.border = UiRect::new(
                    bundle.bvec[0],
                    bundle.bvec[1],
                    bundle.bvec[2],
                    bundle.bvec[3],
                );
            } else if inner_pairs.as_rule() == Rule::VALUE {
                bundle.style.padding = UiRect::all(bundle.val);
            }

            None
        }
        //end-todo:
        Rule::flex_direction_style => {
            let value = pair.as_span().as_str();
            match value {
                "flex_direction-row" => bundle.style.flex_direction = FlexDirection::Row,
                "flex_direction-row-reverse" => bundle.style.flex_direction = FlexDirection::RowReverse,
                "flex_direction-col" => bundle.style.flex_direction = FlexDirection::Column,
                "flex_direction-col-reverse" => bundle.style.flex_direction = FlexDirection::ColumnReverse,
                _ => eprint!("Invalid flex_direction value. FlexDirection value can be either 'start' | 'row' | 'row-reverse' | 'col' | 'col-reverse' "),
            }
            None
        }
        Rule::flex_wrap_style => {
            let value = pair.as_span().as_str();
            match value {
                "flex_wrap-wrap" => bundle.style.flex_wrap = FlexWrap::Wrap,
                "flex_wrap-wrap-reverse" => bundle.style.flex_wrap = FlexWrap::WrapReverse,
                "flex_wrap-no-wrap" => bundle.style.flex_wrap = FlexWrap::NoWrap,
                _ => eprint!("Invalid flex_wrap value. FlexWrap value can be either 'wrap' | 'wrap-reverse' | 'row-reverse' | 'no-wrap'"),
            }
            None
        }
        Rule::flex_grow_style => {
            let value = pair.as_span().as_str().split("-").collect::<Vec<&str>>()[1];
            bundle.style.flex_grow = extract_number(value).unwrap();

            None
        }
        Rule::flex_shrink_style => {
            let value = pair.as_span().as_str().split("-").collect::<Vec<&str>>()[1];
            bundle.style.flex_shrink = extract_number(value).unwrap();

            None
        }
        Rule::flex_basis_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.flex_basis = bundle.val;
            None
        }
        Rule::row_gap_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.row_gap = bundle.val;
            None
        }
        Rule::column_gap_style => {
            let mut inner_pairs = pair.clone().into_inner();
            handler(inner_pairs.next().unwrap(), bundle);
            bundle.style.column_gap = bundle.val;
            None
        }

        //todo: handle grid propely
        Rule::grid_auto_flow_style => None,
        Rule::grid_template_rows_style => None,
        Rule::grid_template_columns_style => None,
        Rule::grid_auto_rows_style => None,
        Rule::grid_auto_columns_style => None,
        Rule::grid_row_style => None,
        Rule::grid_column_style => None,
        //end-todo:
        Rule::property => None,

        Rule::IDENTIFIER => None,
    }
}
