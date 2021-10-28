pub trait View {
    type Input;
    type Output;

    fn view(&mut self, _: Self::Input) -> Self::Output;
}


pub trait Table<In, Out>: View<Input = In, Output = Out> {
    type Title;

    fn title(&self) -> &Self::Title;

    fn content(&self) -> &[BoxTable<Self::Title, In, Out>];
    fn content_mut(&mut self) -> &mut [BoxTable<Self::Title, In, Out>];


    fn len(&self) -> usize;
}
