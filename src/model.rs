pub trait View<In, Out> {
    fn view(&mut self, _: In) -> Out;
}


pub trait Table<In, Out>: View<In, Out> {
    type Title;
    type Child: Table<In, Out, Title = Self::Title, Child = Self::Child>;

    fn title(&self) -> &Self::Title;

    fn content(&self) -> &[Self::Child];
    fn content_mut(&mut self) -> &mut [Self::Child];


    fn len(&self) -> usize;
}
