use std::{cell::RefCell, rc::Rc};

use crate::{KeyCommand, actors::PointActor};

pub fn process_command(key_command: &KeyCommand, point_actor: &Rc<RefCell<PointActor>>) {
    let old_x: i32;
    let old_y: i32;
    {
        let actor_ref = point_actor.borrow();
        old_x = *(actor_ref.x.get());
        old_y = *(actor_ref.y.get());
    }
    if *key_command.w.get() {
        point_actor
            .borrow_mut()
            .y
            .set(old_y.wrapping_sub(1))
    }
    if *key_command.s.get() {
        point_actor
            .borrow_mut()
            .y
            .set(old_y.wrapping_add(1))
    }
    if *key_command.a.get() {
        point_actor
            .borrow_mut()
            .x
            .set(old_x.wrapping_sub(1))
    }
    if *key_command.d.get() {
        point_actor
            .borrow_mut()
            .x
            .set(old_x.wrapping_add(1))
    }
}