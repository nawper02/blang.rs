pub(crate) enum StackItem {
    Number(f64),
    Array(Vec<Vec<f64>>),
}

impl Clone for StackItem {
    fn clone(&self) -> Self {
        match self {
            StackItem::Number(num) => StackItem::Number(*num),
            StackItem::Array(arr) => StackItem::Array(arr.clone()),
            // Add cases for other variants of StackItem
        }
    }
}
