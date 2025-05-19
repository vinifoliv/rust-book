mod box_smart_pointer;
mod deref;
mod drop;
mod rc;
mod refcell;
mod reference_cycles;

fn main() {
    box_smart_pointer::run();
    deref::run();
    drop::run();
    rc::run();
    refcell::run();
    reference_cycles::run();
}
