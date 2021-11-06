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

pub trait TableMut<In, Out>: Table<In, Out> {
    fn insert(&mut self, at: usize, cell: Self::Child);

    fn push(&mut self, cell: Self::Child) {
        self.insert(self.len(), cell)
    }
}
