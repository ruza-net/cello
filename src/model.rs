pub trait View<E> {
    fn view(&mut self) -> E;
}

pub type BoxTable<T, E> = Box<dyn Table<E, Title = T>>;

pub trait Table<E>: View<E> {
    type Title;

    fn title(&self) -> &Self::Title;

    fn content(&self) -> &[BoxTable<Self::Title, E>];
    fn content_mut(&mut self) -> &mut [BoxTable<Self::Title, E>];


    fn len(&self) -> usize;
    fn insert(&mut self, at: usize, cell: BoxTable<Self::Title, E>);

    fn push(&mut self, cell: BoxTable<Self::Title, E>) {
        self.insert(self.len() - 1, cell)
    }
}
