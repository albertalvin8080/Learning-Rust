#![allow(warnings)]

mod _box;
mod _graph_cell;
mod _graph_refcell;
mod _graph_rwlock;
mod _tools_rc;
mod _tools_arc_mutex;

fn main() {
    // _box::run();
    // _graph_cell::run();
    // _graph_refcell::run();
    // _graph_rwlock::run();
    // _tools_rc::run();
    _tools_arc_mutex::run();
}
