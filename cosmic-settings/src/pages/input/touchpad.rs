use cosmic::iced::Alignment;
use cosmic::widget::{self, row, settings, text};
use cosmic::{Apply, Element};
use cosmic_comp_config::input::{AccelProfile, ClickMethod, ScrollMethod};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;

use super::Message;

crate::cache_dynamic_lazy! {
    static CLICK_BEHAVIOR_CLICK_FINGER: String = fl!("click-behavior", "click-finger");
    static CLICK_BEHAVIOR_BUTTON_AREAS: String = fl!("click-behavior", "button-areas");

    static TAP_TO_CLICK: String = fl!("tap-to-click");
    static TAP_TO_CLICK_DESC: String = fl!("tap-to-click", "desc");

    static TOUCHPAD_ACCELERAION: String = fl!("touchpad", "acceleration");
    static TOUCHPAD_SPEED: String = fl!("touchpad", "speed");

    static OPEN_APPLICATION_LIBRARY: String = fl!("open-application-library");
    static OPEN_WORKSPACES_VIEW: String = fl!("open-workspaces-view");
    static SWIPING_FOUR_FINGER_DOWN: String = fl!("gestures", "four-finger-down");
    static SWIPING_FOUR_FINGER_LEFT: String = fl!("gestures", "four-finger-left");
    static SWIPING_FOUR_FINGER_RIGHT: String = fl!("gestures", "four-finger-right");
    static SWIPING_FOUR_FINGER_UP: String = fl!("gestures", "four-finger-up");
    static SWIPING_THREE_FINGER_ANY: String = fl!("gestures", "three-finger-any");
    static SWITCH_BETWEEN_WINDOWS: String = fl!("switch-between-windows");
    static SWITCH_TO_NEXT_WORKSPACE: String = fl!("switch-to-next-workspace");
    static SWITCH_TO_PREV_WORKSPACE: String = fl!("switch-to-prev-workspace");
}

#[derive(Default)]
pub struct Page;

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(touchpad()),
            sections.insert(click_behavior()),
            sections.insert(scrolling()),
            sections.insert(swiping()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("touchpad", "input-touchpad-symbolic")
            .title(fl!("touchpad"))
            .description(fl!("touchpad", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn touchpad() -> Section<crate::pages::Message> {
    Section::default()
        .descriptions(vec![
            super::PRIMARY_BUTTON.as_str().into(),
            TOUCHPAD_SPEED.as_str().into(),
            TOUCHPAD_ACCELERAION.as_str().into(),
            super::ACCELERATION_DESC.as_str().into(),
            super::DISABLE_WHILE_TYPING.as_str().into(),
        ])
        .view::<Page>(|binder, _page, section| {
            let input = binder.page::<super::Page>().expect("input page not found");
            let theme = cosmic::theme::active();

            settings::view_section(&section.title)
                .add(settings::flex_item(
                    &*super::PRIMARY_BUTTON,
                    cosmic::widget::segmented_control::horizontal(&input.touchpad_primary_button)
                        .minimum_button_width(0)
                        .on_activate(|x| Message::PrimaryButtonSelected(x, true)),
                ))
                .add(settings::item::builder(&*TOUCHPAD_SPEED).flex_control({
                    let value = (input
                        .input_touchpad
                        .acceleration
                        .as_ref()
                        .map_or(0.0, |x| x.speed)
                        + 1.0)
                        * 50.0;

                    let slider = widget::slider(10.0..=80.0, value, |value| {
                        Message::SetMouseSpeed((value / 50.0) - 1.0, true)
                    })
                    .width(250.0)
                    .breakpoints(&[45.0]);

                    row::with_capacity(2)
                        .align_items(Alignment::Center)
                        .spacing(theme.cosmic().space_s())
                        .push(text(format!("{:.0}", value.round())))
                        .push(slider)
                }))
                .add(
                    settings::item::builder(&*TOUCHPAD_ACCELERAION)
                        .description(&*super::ACCELERATION_DESC)
                        .toggler(
                            input
                                .input_touchpad
                                .acceleration
                                .as_ref()
                                .map_or(true, |x| x.profile == Some(AccelProfile::Adaptive)),
                            |x| Message::SetAcceleration(x, true),
                        ),
                )
                .add(
                    settings::item::builder(&*super::DISABLE_WHILE_TYPING).toggler(
                        input.input_touchpad.disable_while_typing.unwrap_or(false),
                        |enabled| Message::DisableWhileTyping(enabled, true),
                    ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn click_behavior() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("click-behavior"))
        .descriptions(vec![
            CLICK_BEHAVIOR_CLICK_FINGER.as_str().into(),
            CLICK_BEHAVIOR_BUTTON_AREAS.as_str().into(),
            TAP_TO_CLICK.as_str().into(),
            TAP_TO_CLICK_DESC.as_str().into(),
        ])
        .view::<Page>(|binder, _page, section| {
            let page = binder
                .page::<super::Page>()
                .expect("input devices page not found");

            settings::view_section(&*section.title)
                // Secondary click via two fingers, and middle-click via three fingers
                .add(settings::item_row(vec![widget::radio(
                    &*CLICK_BEHAVIOR_CLICK_FINGER,
                    ClickMethod::Clickfinger,
                    page.input_touchpad.click_method,
                    |option| Message::SetSecondaryClickBehavior(Some(option), true),
                )
                .into()]))
                // Secondary and middle-click via button areas.
                .add(settings::item_row(vec![widget::radio(
                    &*CLICK_BEHAVIOR_BUTTON_AREAS,
                    ClickMethod::ButtonAreas,
                    page.input_touchpad.click_method,
                    |option| Message::SetSecondaryClickBehavior(Some(option), true),
                )
                .into()]))
                .add(
                    settings::item::builder(&*TAP_TO_CLICK).toggler(
                        page.input_touchpad
                            .tap_config
                            .as_ref()
                            .map_or(false, |x| x.enabled),
                        Message::TapToClick,
                    ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn scrolling() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("scrolling"))
        .descriptions(vec![
            super::SCROLLING_TWO_FINGER.as_str().into(),
            super::SCROLLING_EDGE.as_str().into(),
            super::SCROLLING_SPEED.as_str().into(),
            super::SCROLLING_NATURAL.as_str().into(),
            super::SCROLLING_NATURAL_DESC.as_str().into(),
        ])
        .view::<Page>(|binder, _page, section| {
            let page = binder
                .page::<super::Page>()
                .expect("input devices page not found");
            let theme = cosmic::theme::active();

            settings::view_section(&section.title)
                // Two-finger scrolling toggle
                .add(settings::item_row(vec![widget::radio(
                    &*super::SCROLLING_TWO_FINGER,
                    ScrollMethod::TwoFinger,
                    page.input_touchpad
                        .scroll_config
                        .as_ref()
                        .and_then(|x| x.method),
                    |option| Message::SetScrollMethod(Some(option), true),
                )
                .into()]))
                // Edge scrolling toggle
                .add(settings::item_row(vec![widget::radio(
                    &*super::SCROLLING_EDGE,
                    ScrollMethod::Edge,
                    page.input_touchpad
                        .scroll_config
                        .as_ref()
                        .and_then(|x| x.method),
                    |option| Message::SetScrollMethod(Some(option), true),
                )
                .into()]))
                // Scroll speed slider
                .add(settings::item(&*super::SCROLLING_SPEED, {
                    let value = page
                        .input_touchpad
                        .scroll_config
                        .as_ref()
                        .and_then(|x| x.scroll_factor)
                        .unwrap_or(1.)
                        .log(2.)
                        * 10.0
                        + 50.0;

                    let slider = widget::slider(1.0..=100.0, value, |value| {
                        Message::SetScrollFactor(2f64.powf((value - 50.0) / 10.0), true)
                    })
                    .width(250.0)
                    .breakpoints(&[50.0]);

                    row::with_capacity(2)
                        .align_items(Alignment::Center)
                        .spacing(theme.cosmic().space_s())
                        .push(text(format!("{:.0}", value.round())))
                        .push(slider)
                }))
                // Natural scrolling toggle
                .add(
                    settings::item::builder(&*super::SCROLLING_NATURAL)
                        .description(&*super::SCROLLING_NATURAL_DESC)
                        .toggler(
                            page.input_touchpad
                                .scroll_config
                                .as_ref()
                                .map_or(false, |conf| conf.natural_scroll.unwrap_or(false)),
                            |enabled| Message::SetNaturalScroll(enabled, true),
                        ),
                )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}

fn swiping() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("gestures"))
        .descriptions(vec![
            SWIPING_FOUR_FINGER_DOWN.as_str().into(),
            SWIPING_FOUR_FINGER_LEFT.as_str().into(),
            SWIPING_FOUR_FINGER_RIGHT.as_str().into(),
            SWIPING_FOUR_FINGER_UP.as_str().into(),
            SWIPING_THREE_FINGER_ANY.as_str().into(),
        ])
        .view::<Page>(|_binder, _page, section| {
            settings::view_section(&*section.title)
                // .add(
                //     settings::item::builder(&*SWIPING_THREE_FINGER_ANY)
                //         .flex_control(text(&*SWITCH_BETWEEN_WINDOWS)),
                // )
                .add(
                    settings::item::builder(&*SWIPING_FOUR_FINGER_UP)
                        .flex_control(text(&*SWITCH_TO_PREV_WORKSPACE)),
                )
                .add(
                    settings::item::builder(&*SWIPING_FOUR_FINGER_DOWN)
                        .flex_control(text(&*SWITCH_TO_NEXT_WORKSPACE)),
                )
                // .add(
                //     settings::item::builder(&*SWIPING_FOUR_FINGER_LEFT)
                //         .flex_control(text(&*OPEN_WORKSPACES_VIEW)),
                // )
                // .add(
                //     settings::item::builder(&*SWIPING_FOUR_FINGER_RIGHT)
                //         .flex_control(text(&*OPEN_APPLICATION_LIBRARY)),
                // )
                .apply(Element::from)
                .map(crate::pages::Message::Input)
        })
}
