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

impl std::ops::Mul for StackItem {
    type Output = Result<StackItem, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StackItem::Number(a), StackItem::Number(b)) => Ok(StackItem::Number(a * b)),
            (StackItem::Number(scalar), StackItem::Array(arr)) | (StackItem::Array(arr), StackItem::Number(scalar)) => {
                let result = arr.into_iter()
                    .map(|row| row.into_iter().map(|x| x * scalar).collect())
                    .collect();
                Ok(StackItem::Array(result))
            },
            (StackItem::Array(a), StackItem::Array(b)) => {
                if a.is_empty() || b.is_empty() {
                    return Err("Cannot multiply empty arrays".to_string());
                }
                if a[0].len() != b.len() {
                    return Err("Incompatible array dimensions for multiplication".to_string());
                }
                let result: Vec<Vec<f64>> = a.into_iter()
                    .map(|row_a| {
                        b[0].iter().enumerate()
                            .map(|(i, _)| {
                                row_a.iter()
                                    .zip(b.iter().map(|row_b| row_b[i]))
                                    .map(|(&x, y)| x * y)
                                    .sum()
                            })
                            .collect()
                    })
                    .collect();
                Ok(StackItem::Array(result))
            },
        }
    }
}
