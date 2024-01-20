use crate::csv_data_handle::*;

pub struct CsvHolder {
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>,
}

impl CsvHolder {
    pub fn new(headers: Vec<String>, data: Vec<Vec<String>>) -> Option<Self> {
        let header_count = headers.len();
        let rows_of_correct_size = data.iter().all(|row| row.len() == header_count);
        if rows_of_correct_size {
            Some(Self { headers, data })
        } else {
            None
        }
    }

    pub fn row_exists(&self, row: usize) -> bool {
        row > 0 && row <= self.data.len()
    }

    pub fn column_exists(&self, column: usize) -> bool {
        column > 0 && column <= self.headers.len()
    }

    pub fn index_exists(&self, index: Index) -> bool {
        let Index { row, column } = index;
        self.column_exists(column) && self.row_exists(row)
    }
}

impl CsvDataHandle for CsvHolder {
    fn data_at(&self, index: Index) -> CsvResult<&str> {
        if !self.index_exists(index) {
            return Err(CsvError::NoSuchIndex(index));
        }
        let Index { row, column } = index;
        Ok(self.data.index_one_based(row).index_one_based(column))
    }

    fn row(&self, row: usize) -> CsvResult<Vec<&str>> {
        if !self.row_exists(row) {
            return Err(CsvError::NoSuchRow(row));
        }
        Ok(self
            .data
            .index_one_based(row)
            .iter()
            .map(|s| s.as_str())
            .collect())
    }

    fn column(&self, column: usize) -> CsvResult<Vec<&str>> {
        if !self.column_exists(column) {
            return Err(CsvError::NoSuchColumn(column));
        }
        Ok(self
            .data
            .iter()
            .map(|row| row.index_one_based(column).as_str())
            .collect())
    }

    fn headers(&self) -> Vec<&str> {
        self.headers.iter().map(|s| s.as_str()).collect()
    }

    fn property_count(&self) -> usize {
        self.headers.len()
    }

    fn row_count(&self) -> usize {
        self.data.len()
    }

    fn column_count(&self) -> usize {
        self.headers.len()
    }

    fn column_of_field(&self, field: &str) -> CsvResult<usize> {
        self.headers
            .iter()
            .position(|s| s == field)
            .map(|s| s + 1)
            .ok_or(CsvError::NoSuchField(field.to_string()))
    }

    fn replace_data_at(&mut self, index: Index, new_data: String) -> CsvResult<()> {
        if !self.index_exists(index) {
            return Err(CsvError::NoSuchIndex(index));
        }
        let Index { row, column } = index;
        *self
            .data
            .index_one_based_mut(row)
            .index_one_based_mut(column) = new_data;
        Ok(())
    }

    fn replace_column(&mut self, column: usize, new_data: Vec<String>) -> CsvResult<()> {
        if !self.column_exists(column) {
            return Err(CsvError::NoSuchColumn(column));
        }
        if new_data.len() != self.row_count() {
            return Err(CsvError::FailedToReplaceColumn(column));
        }
        for (row, new_value) in self.data.iter_mut().zip(new_data) {
            *row.index_one_based_mut(column) = new_value;
        }
        Ok(())
    }

    fn replace_row(&mut self, row: usize, new_data: Vec<String>) -> CsvResult<()> {
        if !self.row_exists(row) {
            return Err(CsvError::NoSuchRow(row));
        }
        if new_data.len() != self.property_count() {
            return Err(CsvError::FailedToReplaceRow(row));
        }
        *self.data.index_one_based_mut(row) = new_data;
        Ok(())
    }

    fn delete_row(&mut self, row: usize) -> CsvResult<()> {
        if !self.row_exists(row) {
            return Err(CsvError::NoSuchRow(row));
        }
        self.data.remove(row - 1);
        Ok(())
    }

    fn delete_column(&mut self, column: usize) -> CsvResult<()> {
        if !self.column_exists(column) {
            return Err(CsvError::NoSuchColumn(column));
        }
        self.headers.remove(column - 1);
        for row in &mut self.data {
            row.remove(column - 1);
        }
        Ok(())
    }
}

trait IndexOneBased {
    type Item;
    fn index_one_based(&self, index: usize) -> &Self::Item;
    fn index_one_based_mut(&mut self, index: usize) -> &mut Self::Item;
}

impl<T> IndexOneBased for Vec<T> {
    type Item = T;
    fn index_one_based(&self, index: usize) -> &Self::Item {
        &self[index - 1]
    }

    fn index_one_based_mut(&mut self, index: usize) -> &mut Self::Item {
        &mut self[index - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_one_based() {
        let v = vec![1, 2, 3];
        assert_eq!(v.index_one_based(1), &1);
        assert_eq!(v.index_one_based(2), &2);
        assert_eq!(v.index_one_based(3), &3);
    }

    #[test]
    fn test_index_one_based_mut() {
        let mut v = vec![1, 2, 3];
        *v.index_one_based_mut(1) = 4;
        assert_eq!(v, vec![4, 2, 3]);
        *v.index_one_based_mut(2) = 5;
        assert_eq!(v, vec![4, 5, 3]);
        *v.index_one_based_mut(3) = 6;
        assert_eq!(v, vec![4, 5, 6]);
    }

    #[test]
    fn test_new() {
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
        ];
        let csv_holder = CsvHolder::new(headers, data).unwrap();
        assert_eq!(csv_holder.headers, vec!["a", "b", "c"]);
        assert_eq!(
            csv_holder.data,
            vec![vec!["1", "2", "3"], vec!["4", "5", "6"],]
        );
    }

    #[test]
    fn test_new_with_invalid_data() {
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string()],
        ];
        let csv_holder = CsvHolder::new(headers, data);
        assert!(csv_holder.is_none());
    }

    #[test]
    fn test_row_exists() {
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
        ];
        let csv_holder = CsvHolder::new(headers, data).unwrap();
        assert!(csv_holder.row_exists(1));
        assert!(csv_holder.row_exists(2));
        assert!(!csv_holder.row_exists(3));
    }

    #[test]
    fn test_get_row() {
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
        ];
        let csv_holder = CsvHolder::new(headers, data).unwrap();
        assert_eq!(
            csv_holder.row(1).unwrap(),
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        );
        assert_eq!(
            csv_holder.row(2).unwrap(),
            vec!["4".to_string(), "5".to_string(), "6".to_string()]
        );
    }

    #[test]
    fn test_get_row_with_invalid_row() {
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
        ];
        let csv_holder = CsvHolder::new(headers, data).unwrap();
        assert!(csv_holder.row(0).is_err());
        assert!(csv_holder.row(3).is_err());
    }

    #[test]
    fn test_get_column() {
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
        ];
        let csv_holder = CsvHolder::new(headers, data).unwrap();
        assert_eq!(
            csv_holder.column(1).unwrap(),
            vec!["1".to_string(), "4".to_string()]
        );
        assert_eq!(
            csv_holder.column(2).unwrap(),
            vec!["2".to_string(), "5".to_string()]
        );
        assert_eq!(
            csv_holder.column(3).unwrap(),
            vec!["3".to_string(), "6".to_string()]
        );
    }

    #[test]
    fn test_get_column_with_invalid_column() {
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
        ];
        let csv_holder = CsvHolder::new(headers, data).unwrap();
        assert!(csv_holder.column(0).is_err());
        assert!(csv_holder.column(4).is_err());
    }

    #[test]
    fn test_delete_row() {
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let mut data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["7".to_string(), "8".to_string(), "9".to_string()],
        ];
        let mut csv_holder = CsvHolder::new(headers, data.clone()).unwrap();
        csv_holder.delete_row(2).unwrap();
        data.remove(1);
        assert_eq!(csv_holder.data, data);
    }

    #[test]
    fn test_delete_row_with_invalid_row() {
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["7".to_string(), "8".to_string(), "9".to_string()],
        ];
        let mut csv_holder = CsvHolder::new(headers, data.clone()).unwrap();
        assert!(csv_holder.delete_row(0).is_err());
        assert!(csv_holder.delete_row(4).is_err());
        assert_eq!(csv_holder.data, data);
    }

    #[test]
    fn test_delete_column() {
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let mut data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["7".to_string(), "8".to_string(), "9".to_string()],
        ];
        let mut csv_holder = CsvHolder::new(headers, data.clone()).unwrap();
        csv_holder.delete_column(2).unwrap();
        data.iter_mut().for_each(|row| {
            row.remove(1);
        });
        assert_eq!(csv_holder.data, data);
    }

    #[test]
    fn test_delete_column_with_invalid_column() {
        let headers = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let data = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["7".to_string(), "8".to_string(), "9".to_string()],
        ];
        let mut csv_holder = CsvHolder::new(headers, data.clone()).unwrap();
        assert!(csv_holder.delete_column(0).is_err());
        assert!(csv_holder.delete_column(4).is_err());
        assert_eq!(csv_holder.data, data);
    }
}
