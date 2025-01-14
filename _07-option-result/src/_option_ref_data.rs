pub fn run() {
    let w = Widget(Some(Data(34u8)));
    let v1: &Option<Data> = w.get_data_v1();
    let v2: Option<&Data> = w.get_data_v2();

    // We need to use as_ref() anyway because we cannot consume v1 in map(self) (due to it being a reference).
    v1.as_ref().map(|data| println!("{:?}", data));
    v2.map(|data| println!("{:?}", data));
}

#[derive(Debug)]
struct Data(u8);

#[derive(Debug)]
struct Widget(Option<Data>);

impl Widget {
    fn get_data_v1(&self) -> &Option<Data> {
        &self.0
    }
    fn get_data_v2(&self) -> Option<&Data> {
        self.0.as_ref()
    }
}
