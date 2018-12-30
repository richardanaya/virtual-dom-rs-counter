use crate::actions::AppAction;
use crate::store::Reducer;
use crate::selector::create_selector;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Hash,Eq,PartialEq)]
pub struct AppState {
    pub count: i32,
}

impl AppState {
    pub fn new() -> AppState {
        AppState { count: 0 }
    }
}

impl Reducer<AppAction> for Rc<AppState> {
    fn reduce(&self, a: AppAction) -> Option<Rc<AppState>> {
        match a {
            AppAction::Increment => Some(Rc::new(AppState {
                count: self.count + 1,
            })),
            AppAction::Decrement => Some(Rc::new(AppState {
                count: self.count - 1,
            })),
        }
    }
}

pub struct Selectors{}
impl Selectors {
    pub fn get_count(b:Rc<AppState>) -> i32{
        thread_local!(static c:RefCell<HashMap<Rc<AppState>,i32>> = RefCell::new(HashMap::new()));
        let s = create_selector(&c,|v:Rc<AppState>|{
            v.count
        });
        s(b)
    }
}
