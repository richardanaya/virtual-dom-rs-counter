use crate::actions::AppAction;
use crate::selector::create_selector;
use crate::store::Reducer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Hash, Eq, PartialEq)]
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

pub struct Selectors {}
impl Selectors {
    pub fn get_count(b: Rc<AppState>) -> i32 {
        // create the selector one time
        thread_local! {
            static cache:RefCell<HashMap<Rc<AppState>,i32>> = RefCell::new(HashMap::new());
            static s:Box<Fn(Rc<AppState>)->i32> = create_selector(&cache,
                |v:Rc<AppState>|{
                    v.count
                }
            );
        };
        // call it each subsequent time
        s.with(|sel| sel(b))
    }
}
