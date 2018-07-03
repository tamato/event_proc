#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::SystemTime;

// component
struct Distance {
    val: i32
}

trait BaseComponent {
    type Return;
    fn show(&self) -> Self::Return;
}

impl BaseComponent for Distance {
    type Return = i32;
    fn show(&self) -> Self::Return {
        println!("from BaseComponent {}", self.val);
        self.val
    }
}

// System
type OffsetType = (i32, i32);
struct OffsetCollection { 
    data: Vec<OffsetType>
}
impl OffsetCollection {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn add(&mut self, target:i32, value:i32) {
        self.data.push( (target, value) );
    }
}

struct OffsetAction {}
impl OffsetAction {
    fn process2(data: OffsetCollection, comps: &mut Vec<Distance>) {
        assert_eq!(data.data.len(), 1);
        for (target, value) in data.data {
            comps[target as usize].val += value;
        }
    }
    fn process(data: Vec<OffsetType>, comps: &mut Vec<Distance>) {
        assert_eq!(data.len(), 1);
        for (target, value) in data {
            comps[target as usize].val += value;
        }
    }
}

// 'BaseComponent' has an assocaited type. Find a way to store it in a list without having to
// decare it's concrete type.
trait AbsComponent {
    fn printme(&self);
}
// implement this for everybody..
// but really only for BaseComponent
impl<T> AbsComponent for T 
where
    T: BaseComponent,
{
    fn printme(&self) {
        println!("Abs + Base Comp");
    }
}

fn main() {
    // To start off have a 1 system that:
    //  Offsets Distance by some number
    let mut comps = vec![Distance{val:0}];
    let mut sys: OffsetCollection;

    // part of test 1
    sys = OffsetCollection::new();

    // new'ing 'sys' evertime is: 483 milli seconds
    // draining out data and filling it back in is: 
    let now = SystemTime::now();
    for i in 0..10000 {

        // test 1 for perf
        sys.add(0, 3);
        let coll = sys.data.drain(..).collect();
        OffsetAction::process(coll, &mut comps);


        // Test 2 for perf
        // sys = OffsetCollection::new();
        // sys.add(0, 3);
        // OffsetAction::process2(sys, &mut comps);
    }

    match now.elapsed() {
        Ok(elapsed) => { println!("Time {}", elapsed.subsec_millis()); }
        Err(e) => { println!("Error: {:?}", e); }
    }

    for c in comps {
        c.show();
    }

    // Systems has an assocaited type
    // other objects can implement Systems and in that impl they specify it
    // But the collections may store a trait that 'Systems' impl.
    let mut cool: Vec<Box<dyn AbsComponent>> = Vec::new();
    cool.push( Box::new(Distance{val:0}) );
    for c in cool {
        c.printme();
    }
}




