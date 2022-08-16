use std::{cell::RefCell, cell::RefMut, collections::HashMap, rc::Rc};

fn main() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));

    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
    }

    let total: i32 = shared_map.borrow().values().sum();
    println!("{}", total);
}
