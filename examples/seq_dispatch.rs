extern crate shred;
#[macro_use]
extern crate shred_derive;

use shred::{DispatcherBuilder, Read, Resources, System, Write};

#[derive(Debug, Default)]
struct ResA;

#[derive(Debug, Default)]
struct ResB;

#[derive(StaticSystemData)]
struct Data<'a> {
    a: Read<'a, ResA>,
    b: Write<'a, ResB>,
}

struct EmptySystem;

impl<'a> System<'a> for EmptySystem {
    type SystemData = Data<'a>;

    fn run(&mut self, bundle: Data<'a>) {
        println!("{:?}", &*bundle.a);
        println!("{:?}", &*bundle.b);
    }
}

fn main() {
    let mut resources = Resources::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(EmptySystem, "empty", &[])
        .build();
    dispatcher.setup(&mut resources);

    dispatcher.dispatch_seq(&resources);
}
