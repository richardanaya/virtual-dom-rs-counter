use crate::store::Reducer;
use crate::actions::AppAction;

pub struct AppState {
    pub count: i32,
}

impl AppState {
    pub fn new() -> AppState {
        AppState { count: 0 }
    }
}

impl Reducer<AppAction> for AppState {
    fn reduce(&self, a: AppAction) -> Option<AppState> {
        match a {
            AppAction::Increment => Some(AppState {
                count: self.count + 1,
            }),
            AppAction::Decrement => Some(AppState {
                count: self.count - 1,
            }),
        }
    }
}
