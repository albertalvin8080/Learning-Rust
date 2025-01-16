use std::{marker::PhantomData, ptr::NonNull};

trait Speak {
    fn speak(&self) -> ();
}

struct Cat;
impl Speak for Cat {
    fn speak(&self) -> () {
        println!("meow");
    }
}

struct Dog;
impl Speak for Dog {
    fn speak(&self) -> () {
        println!("woof");
    }
}

struct VTable {
    speak_thunk: unsafe fn(NonNull<()>),
}

struct AnySpeak<'a> {
    _p: PhantomData<&'a ()>,
    data: NonNull<()>,
    vtable: &'static VTable,
}

impl<'a> AnySpeak<'a> {
    pub fn new<T: Speak>(t: &'a T) -> Self {
        Self {
            _p: PhantomData,
            data: NonNull::from(t).cast(), // <- cast to unit type ()
            vtable: &VTable { // <- constant promotion
                speak_thunk: |data| unsafe {data.cast::<T>().as_ref()}.speak()
            }
        }
    }

    pub fn speak(&self) {
        unsafe { (self.vtable.speak_thunk)(self.data) }
    }
}

pub fn run() {
    AnySpeak::new(&Cat).speak();
    AnySpeak::new(&Dog).speak();
    // Yes, with some modifications we can make a cat woof and a dog meow.
    //  ... pub fn new<T: Speak, U: Speak>(t: &'a T) ...
    //  ... unsafe {data.cast::<U>().as_ref()}.speak() ...
    // AnySpeak::new::<_, Dog>(&Cat).speak(); // woof from a Cat
}