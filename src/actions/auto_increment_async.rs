use crate::actions::AppAction;
use crate::reducers::app::AppState;
use std::rc::Rc;

pub fn auto_increment_async(_state: Rc<AppState>, dispatch: Rc<Fn(AppAction)>) -> Box<Fn()> {
    Box::new(move || {
        dispatch(AppAction::Increment);
    })
}
