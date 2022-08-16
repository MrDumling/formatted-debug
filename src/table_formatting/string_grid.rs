use crate::table_formatting::grid_formatting::GridSizes;
use crate::table_formatting::StringTable;

use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;

impl<K: Display, V: Display> StringTable for HashMap<K, V> {
    /// Formats a BTreeMap into a friendly easy to print format
    /// # Example
    /// ```no_run
    /// use std::collections::HashMap;
    ///
    /// let contacts = HashMap::from([
    ///     ("Daniel", "798-1364"),
    ///     ("Ashley", "645-7689"),
    ///     ("Katie", "545-435-8291"),
    ///     ("Robert", "956-1745")
    /// ]);
    /// ```
    /// Will produce a vector of String's which holds:
    /// ```text
    /// "┏━━━━━━┳━━━━━━━━━━━━┓",
    /// "┃Keys: ┃Values:     ┃",
    /// "┣━━━━━━╋━━━━━━━━━━━━┫",
    /// "┃Robert┃956-1745    ┃",
    /// "┣━━━━━━╋━━━━━━━━━━━━┫",
    /// "┃Ashley┃645-7689    ┃",
    /// "┣━━━━━━╋━━━━━━━━━━━━┫",
    /// "┃Daniel┃798-1364    ┃",
    /// "┣━━━━━━╋━━━━━━━━━━━━┫",
    /// "┃Katie ┃545-435-8291┃",
    /// "┗━━━━━━┻━━━━━━━━━━━━┛",
    /// ```
    ///
    /// ### Note:
    /// Because HashMap is unordered,
    /// the produced table may vary in ouput.
    fn to_table(&self) -> Vec<String> {
        let mut values = vec![[String::from("Keys:"), String::from("Values:")]];

        for (key, value) in self {
            values.push([format!("{}", key), format!("{}", value)]);
        }

        generate_string_grid(&values)
    }
}

impl<K: Display, V: Display> StringTable for BTreeMap<K, V> {
    /// Formats a BTreeMap into a friendly easy to print format
    /// # Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use formatted_debug::table_formatting::StringTable;
    ///
    /// let solar_distance = BTreeMap::from([
    ///     ("Mercury", 0.4),
    ///     ("Venus", 0.7),
    ///     ("Earth", 1.0),
    ///     ("Mars", 1.5),
    /// ]);
    ///
    /// assert_eq!(
    ///     solar_distance.to_table(), vec![
    ///         String::from("┏━━━━━━━┳━━━━━━━┓"),
    ///         String::from("┃Keys:  ┃Values:┃"),
    ///         String::from("┣━━━━━━━╋━━━━━━━┫"),
    ///         String::from("┃Earth  ┃1      ┃"),
    ///         String::from("┣━━━━━━━╋━━━━━━━┫"),
    ///         String::from("┃Mars   ┃1.5    ┃"),
    ///         String::from("┣━━━━━━━╋━━━━━━━┫"),
    ///         String::from("┃Mercury┃0.4    ┃"),
    ///         String::from("┣━━━━━━━╋━━━━━━━┫"),
    ///         String::from("┃Venus  ┃0.7    ┃"),
    ///         String::from("┗━━━━━━━┻━━━━━━━┛"),
    ///     ]
    /// );
    /// ```
    fn to_table(&self) -> Vec<String> {
        let mut values = vec![[String::from("Keys:"), String::from("Values:")]];

        for (key, value) in self {
            values.push([format!("{}", key), format!("{}", value)]);
        }

        generate_string_grid(&values)
    }
}

impl<T: Display> StringTable for Vec<T> {
    /// ```
    /// use formatted_debug::table_formatting::StringTable;
    /// 
    /// let held_contents = vec![0, 1, 2, 3, 4, 5];
    ///
    /// assert_eq!(held_contents.to_table(), vec![
    ///     String::from("┏━┓"),
    ///     String::from("┃0┃"),
    ///     String::from("┣━┫"),
    ///     String::from("┃1┃"),
    ///     String::from("┣━┫"),
    ///     String::from("┃2┃"),
    ///     String::from("┣━┫"),
    ///     String::from("┃3┃"),
    ///     String::from("┣━┫"),
    ///     String::from("┃4┃"),
    ///     String::from("┣━┫"),
    ///     String::from("┃5┃"),
    ///     String::from("┗━┛"),
    /// ]);
    /// ```
    fn to_table(&self) -> Vec<String> {
        let mut values = Vec::new();

        for current_value in self {
            values.push([format!("{}", current_value)]);
        }

        generate_string_grid(&values)
    }
}

impl StringTable for String {
    /// ```
    /// use formatted_debug::table_formatting::StringTable;
    ///
    /// let held_string = String::from(
    ///     "Hello there!\n\
    ///     this is a multilined string\n\
    ///     it should fill up one box\n\
    ///     and only one box"
    /// );
    ///
    /// assert_eq!(held_string.to_table(), vec![
    ///     String::from("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━┓"),
    ///     String::from("┃Hello there!               ┃"),
    ///     String::from("┃this is a multilined string┃"),
    ///     String::from("┃it should fill up one box  ┃"),
    ///     String::from("┃and only one box           ┃"),
    ///     String::from("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"),
    /// ]);
    /// ```
    fn to_table(&self) -> Vec<String> {
        generate_string_grid(&vec![[self.to_string()]])
    }
}

/// Represents a location in a grid,
///
/// A value of
/// ```text
/// GridCoord {
///     x: 3,
///     y: 1,
/// }
/// ```
/// would be found in the following:
/// ```text
/// ┏━┳━━┳━━━━━┓
/// ┃ ┃  ┃here ┃
/// ┣━╋━━╋━━━━━┫
/// ┃ ┃  ┃     ┃
/// ┃ ┃  ┃     ┃
/// ┃ ┃  ┃     ┃
/// ┗━┻━━┻━━━━━┛
/// ```
struct GridCoord {
    x: usize,
    y: usize,
}

/// Generates a formatted grid containing strings
/// # Example
/// ```
/// use formatted_debug::table_formatting::string_grid;
/// 
/// let formatted_strings = vec![
///     [String::from("Operation"), String::from("Values"), String::from("Result")],
///     [String::from("Addition"), String::from("4, 12"), format!("{}", 4 + 12)],
///     [String::from("Division"), String::from("10, 5"), format!("{}", 10 / 5)],
/// ];
/// 
/// assert_eq!(string_grid::generate_string_grid(&formatted_strings), vec![
///     "┏━━━━━━━━━┳━━━━━━┳━━━━━━┓",
///     "┃Operation┃Values┃Result┃",
///     "┣━━━━━━━━━╋━━━━━━╋━━━━━━┫",
///     "┃Addition ┃4, 12 ┃16    ┃",
///     "┣━━━━━━━━━╋━━━━━━╋━━━━━━┫",
///     "┃Division ┃10, 5 ┃2     ┃",
///     "┗━━━━━━━━━┻━━━━━━┻━━━━━━┛"
/// ]);
/// ```
pub fn generate_string_grid<const U: usize>(contents: &Vec<[String; U]>) -> Vec<String> {
    if contents.is_empty() {
        return vec![String::from("┏┓"), String::from("┗┛")];
    }

    let grid = GridSizes {
        widths: get_max_widths(contents),
        heights: get_max_heights(contents),
    };

    let mut result = grid.to_table();

    for (row_index, row) in contents.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate().take(U) {
            result = insert_text(
                column,
                &mut result,
                &grid,
                GridCoord {
                    x: column_index,
                    y: row_index,
                },
            )
        }
    }

    result
}

/// Get the max width of each column
fn get_max_widths<const U: usize>(contents: &Vec<[String; U]>) -> Vec<usize> {
    let mut widths = Vec::new();

    for column_index in 0..U {
        let mut max_width = 0usize;
        for row in contents.iter() {
            let current_width = get_string_width(&row[column_index]) + 2;
            max_width = std::cmp::max(max_width, current_width);
        }
        widths.push(max_width);
    }

    widths
}

/// Get the max height of each row
fn get_max_heights<const U: usize>(contents: &Vec<[String; U]>) -> Vec<usize> {
    let mut heights = Vec::new();

    for row in contents.iter() {
        let mut max_height = 0usize;
        for column in row.iter().take(U) {
            let current_height = get_string_height(column) + 2;
            max_height = std::cmp::max(max_height, current_height);
        }
        heights.push(max_height);
    }

    heights
}

/// Pain and Suffering
/// 
/// Given:
/// * an inserted text,
/// * lines which represent a formatted grid
/// * the grid sizes,
/// * the replacement coordinates
/// 
/// insert the text into the grid, and return the modified lines
fn insert_text(
    inserted_text: &str,
    original_lines: &mut [String],
    grid: &GridSizes,
    replacement_coords: GridCoord,
) -> Vec<String> {
    //get changed y values
    let string_y_start = get_replaced_dimension_index(&grid.heights, replacement_coords.y);
    let string_y_end = string_y_start + get_string_height(inserted_text);
    //find changed x value assuming non-UTF8
    let string_x_start = get_replaced_dimension_index(&grid.widths, replacement_coords.x);

    let changed_lines = &mut original_lines[string_y_start..string_y_end];

    for (line_index, current_line) in changed_lines.iter_mut().enumerate() {
        let inserted_line = inserted_text.split('\n').nth(line_index).unwrap();
        //convert x start and end to be UTF8 friendly
        let string_x_end = map_string_index(
            current_line,
            string_x_start + get_string_width(&String::from(inserted_line)),
        );
        let string_x_start = map_string_index(current_line, string_x_start);

        current_line.replace_range(string_x_start..string_x_end, inserted_line);
    }

    original_lines.to_vec()
}

fn map_string_index(controlling_string: &str, index: usize) -> usize {
    controlling_string.char_indices().nth(index).unwrap().0
}

fn get_replaced_dimension_index(dimension: &[usize], coord: usize) -> usize {
    let mut start_index = 1usize; //starts at 1 to avoid the top padding

    for (current_coord, current_dimension) in dimension.iter().enumerate() {
        if current_coord == coord {
            return start_index;
        }

        start_index += current_dimension - 1; // - 1 to keep only one instance of padding
    }

    panic!("coord value of {} could not be reached", coord)
}

fn get_string_width(s: &str) -> usize {
    let mut max_width = 0usize;

    for current_line in s.split('\n') {
        let current_width = current_line.chars().count();
        if current_width > max_width {
            max_width = current_width
        }
    }

    max_width
}

fn get_string_height(s: &str) -> usize {
    s.matches('\n').count() + 1usize
}
