#[derive(Debug, Clone)]
pub enum Message {
    AcceptAll,
    ReviewChange(usize),
    IgnoreChange(usize),
    FinishReview,
    RowClicked(usize),
}