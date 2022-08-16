use crate::table_formatting::StringTable;

/// A struct which holds widths of rectangles that form a grid
/// # Example
/// 
/// ```
/// use formatted_debug::table_formatting::grid_formatting::GridSizes;
/// use formatted_debug::table_formatting::StringTable;
/// 
/// let grid = GridSizes {
///     widths: vec![3usize, 4usize, 2usize],
///     heights: vec![3usize, 5usize],
/// };
///
/// assert_eq!(
///     grid.to_table(), vec![
///         String::from("┏━┳━━┳┓"),
///         String::from("┃ ┃  ┃┃"),
///         String::from("┣━╋━━╋┫"),
///         String::from("┃ ┃  ┃┃"),
///         String::from("┃ ┃  ┃┃"),
///         String::from("┃ ┃  ┃┃"),
///         String::from("┗━┻━━┻┛"),
///     ]
/// );
/// ```
/// 
/// You are not able to provide a size of less than 2:
/// ```should_panic
/// use formatted_debug::table_formatting::grid_formatting::GridSizes;
/// use formatted_debug::table_formatting::StringTable;
/// 
/// let grid = GridSizes {
///     widths: vec![1usize, 4usize, 2usize],
///     heights: vec![3usize, 5usize],
/// };
/// 
/// grid.to_table();
/// ```
pub struct GridSizes {
    pub widths: Vec<usize>,
    pub heights: Vec<usize>,
}

impl StringTable for GridSizes {
    fn to_table(&self) -> Vec<String> {
        let mut result = vec![self.generate_top_string()];

        let max_height_index = self.heights.len() - 1;
        let column_seperator = self.get_column_seperator();

        for height_index in 0..=max_height_index {
            let current_height = self.heights[height_index];

            result.append(&mut self.generate_columns(current_height));
            if height_index != max_height_index {
                result.push((&column_seperator).to_string());
            }
        }
        result.push(self.generate_bottom_string());

        result
    }
}

impl GridSizes {
    /// retrieves top layer
    /// output looks like: "┏━━━━━━━━━━┳━━━━━┳━━━━━━━┓"
    fn generate_top_string(&self) -> String {
        let mut top_layer = format!("┏{}", "━".repeat(self.widths[0] - 2));

        for i in 1..=self.widths.len() - 1 {
            top_layer += "┳";
            top_layer += &"━".repeat(self.widths[i] - 2);
        }

        top_layer += "┓";
        top_layer
    }

    /// retrieves bottom layer
    /// output looks like: "┗━━━━━━━━━━┻━━━━━┻━━━━━━━┛"
    fn generate_bottom_string(&self) -> String {
        let mut top_layer = format!("┗{}", "━".repeat(self.widths[0] - 2));

        for i in 1..=self.widths.len() - 1 {
            top_layer += "┻";
            top_layer += &"━".repeat(self.widths[i] - 2);
        }

        top_layer += "┛";
        top_layer
    }

    fn generate_columns(&self, columns_height: usize) -> Vec<String> {
        vec![self.get_unit_columns(); columns_height - 2]
    }

    fn get_unit_columns(&self) -> String {
        let mut unit_column = String::from("┃");

        for current_width in self.widths.iter() {
            unit_column += &" ".repeat(current_width - 2);
            unit_column += "┃";
        }

        unit_column
    }

    /// sits in between table entries
    /// output looks like: "┣━━━━━━━━━━╋━━━━━╋━━━━━━━┫" 
    fn get_column_seperator(&self) -> String {
        let mut column_seperator = String::from("┣");
        let max_width_index = self.widths.len() - 1;

        for width_index in 0..=max_width_index {
            let current_width = self.widths[width_index];
            column_seperator += &"━".repeat(current_width - 2);

            if width_index != max_width_index {
                column_seperator += "╋";
            }
        }

        column_seperator += "┫";
        column_seperator
    }
}