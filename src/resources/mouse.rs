use crate::prelude::*;

pub struct Button {
    index: usize,
    pub is_pressed: bool,
    pub was_pressed: bool,
    pub was_released: bool,
}

#[derive(PartialEq)]
pub enum SelectionState {
    None,
    Drag,
    Released,
}

pub struct Selection {
    pub state: SelectionState,
    pub rect: Rect,
}

impl Selection {
    pub fn get_drawable_rect(&self) -> Rect {
        let x = if self.rect.x1 <= self.rect.x2 {
            self.rect.x1
        } else {
            self.rect.x2
        };
        let y = if self.rect.y1 <= self.rect.y2 {
            self.rect.y1
        } else {
            self.rect.y2
        };

        Rect {
            x1: x,
            y1: y,
            x2: x + self.rect.width(),
            y2: y + self.rect.height(),
        }
    }
}

pub struct Mouse {
    pub point: Point,

    pub left_button: Button,
    pub right_button: Button,

    pub color: ColorPair,
    pub cursor_glyph: FontCharType,

    pub selection: Selection,
}

impl Mouse {
    pub fn new(color: ColorPair, cursor_glyph: FontCharType) -> Self {
        Self {
            point: Point::zero(),
            left_button: Button {
                index: 0,
                is_pressed: false,
                was_pressed: false,
                was_released: false,
            },
            right_button: Button {
                index: 1,
                is_pressed: false,
                was_pressed: false,
                was_released: false,
            },
            color,
            cursor_glyph,
            selection: Selection {
                state: SelectionState::None,
                rect: Rect::zero(),
            },
        }
    }

    pub fn update(&mut self) {
        self.point = INPUT.lock().mouse_tile(0);
        Mouse::update_button(&mut self.left_button);
        Mouse::update_button(&mut self.right_button);
        self.update_selection();
    }

    fn update_button(button: &mut Button) {
        let new_is_pressed = INPUT.lock().is_mouse_button_pressed(button.index);
        let old_is_pressed = button.is_pressed;

        button.is_pressed = new_is_pressed;

        if button.was_pressed {
            button.was_pressed = false;
        } else if !old_is_pressed && new_is_pressed {
            button.was_pressed = true;
        }

        if button.was_released {
            button.was_released = false;
        } else if old_is_pressed && !new_is_pressed {
            button.was_released = true;
        }
    }

    fn update_selection(&mut self) {
        if self.selection.state == SelectionState::Released {
            self.selection.state = SelectionState::None;
            self.selection.rect = Rect::zero();
        } else if self.selection.state == SelectionState::None && self.left_button.was_pressed {
            self.selection.state = SelectionState::Drag;
            self.selection.rect.x1 = self.point.x;
            self.selection.rect.y1 = self.point.y;
            self.selection.rect.x2 = self.point.x;
            self.selection.rect.y2 = self.point.y;
        } else if self.selection.state == SelectionState::Drag && self.left_button.was_released {
            self.selection.state = SelectionState::Released;
            self.selection.rect.x2 = self.point.x;
            self.selection.rect.y2 = self.point.y;
        } else if self.selection.state == SelectionState::Drag && self.left_button.is_pressed {
            self.selection.rect.x2 = self.point.x;
            self.selection.rect.y2 = self.point.y;
        }
    }
}
