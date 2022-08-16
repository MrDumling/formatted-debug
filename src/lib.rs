//! # Formatted Debug
//!
//! `formatted_debug` is a collection of values to make debugging and
//! printing easier and more convenient.

pub mod table_formatting;
pub mod string_stylizing;

#[cfg(test)]
mod unit_tests {
    mod grid_tests {
        use crate::table_formatting::grid_formatting::GridSizes;
        use crate::table_formatting::StringTable;
    
        #[test]
        fn basic_single_rect() {
            let grid = GridSizes {
                widths: vec![3usize],
                heights: vec![3usize],
            };
    
            assert_eq!(
                grid.to_table(),
                vec![
                    String::from("┏━┓"),
                    String::from("┃ ┃"),
                    String::from("┗━┛"),
                ]
            );
    
            let grid = GridSizes {
                widths: vec![2usize],
                heights: vec![2usize],
            };
    
            assert_eq!(
                grid.to_table(),
                vec![String::from("┏┓"), String::from("┗┛"),]
            );
    
            let grid = GridSizes {
                widths: vec![3usize],
                heights: vec![2usize],
            };
    
            assert_eq!(
                grid.to_table(),
                vec![String::from("┏━┓"), String::from("┗━┛"),]
            );
        }
    
        #[test]
        fn basic_multi_rect() {
            let grid = GridSizes {
                widths: vec![3usize, 4usize],
                heights: vec![3usize],
            };
    
            assert_eq!(
                grid.to_table(), vec![
                    String::from("┏━┳━━┓"),
                    String::from("┃ ┃  ┃"),
                    String::from("┗━┻━━┛"),
                ]
            );
    
            let grid = GridSizes {
                widths: vec![3usize],
                heights: vec![2usize, 3usize],
            };
    
            assert_eq!(
                grid.to_table(), vec![
                    String::from("┏━┓"),
                    String::from("┣━┫"),
                    String::from("┃ ┃"),
                    String::from("┗━┛"),
                ]
            );
        }
    
        #[test]
        fn complex_multi_rec() {
            let grid = GridSizes {
                widths: vec![3usize, 4usize, 2usize],
                heights: vec![3usize, 5usize],
            };
    
            assert_eq!(
                grid.to_table(), vec![
                    String::from("┏━┳━━┳┓"),
                    String::from("┃ ┃  ┃┃"),
                    String::from("┣━╋━━╋┫"),
                    String::from("┃ ┃  ┃┃"),
                    String::from("┃ ┃  ┃┃"),
                    String::from("┃ ┃  ┃┃"),
                    String::from("┗━┻━━┻┛"),
                ]
            );
        }
    }
    
    mod string_grid_tests {
        use crate::table_formatting::StringTable;
        #[test]
        fn test_basic_table() {
            {
                use std::collections::HashMap;
        
                let contacts = HashMap::from([
                    ("Daniel", "798-1364"),
                    ("Ashley", "645-7689"),
                    ("Katie", "545-435-8291"),
                    ("Robert", "956-1745"),
                ]);

                let mut lines = vec![
                    String::from("┏━━━━━━┳━━━━━━━━━━━━┓"), 
                    String::from("┃Keys: ┃Values:     ┃")
                ];

                for (k, v) in contacts.iter() {
                    lines.push(String::from("┣━━━━━━╋━━━━━━━━━━━━┫"));
                    lines.push(format!("┃{: <6}┃{: <12}┃", k, v));
                }
                lines.push(String::from("┗━━━━━━┻━━━━━━━━━━━━┛"));
                
                assert_eq!(contacts.to_table(), lines); 
            }
        }
    
        //also should work for HashMaps, just they are a pain to do tests for in this case
        #[test]
        fn test_nested_tables() {
            use std::collections::BTreeMap;
    
            let inner_a = BTreeMap::from([
                ("InnerA1","ValueA1"),
                ("InnerA2","ValueA2"),
            ]);
    
            let inner_b = BTreeMap::from([
                ("InnerB1","ValueB1"),
                ("InnerB2","ValueB2"),
                ("InnerB3","ValueB3"),
            ]);
    
            let outer = BTreeMap::from([
                ("Outer KeyA", inner_a.to_table().join("\n")),
                ("Outer KeyB", inner_b.to_table().join("\n")),
            ]);
    
            assert_eq!(outer.to_table(), vec![
                String::from("┏━━━━━━━━━━┳━━━━━━━━━━━━━━━━━┓"),
                String::from("┃Keys:     ┃Values:          ┃"),
                String::from("┣━━━━━━━━━━╋━━━━━━━━━━━━━━━━━┫"),
                String::from("┃Outer KeyA┃┏━━━━━━━┳━━━━━━━┓┃"),
                String::from("┃          ┃┃Keys:  ┃Values:┃┃"),
                String::from("┃          ┃┣━━━━━━━╋━━━━━━━┫┃"),
                String::from("┃          ┃┃InnerA1┃ValueA1┃┃"),
                String::from("┃          ┃┣━━━━━━━╋━━━━━━━┫┃"),
                String::from("┃          ┃┃InnerA2┃ValueA2┃┃"),
                String::from("┃          ┃┗━━━━━━━┻━━━━━━━┛┃"),
                String::from("┣━━━━━━━━━━╋━━━━━━━━━━━━━━━━━┫"),
                String::from("┃Outer KeyB┃┏━━━━━━━┳━━━━━━━┓┃"),
                String::from("┃          ┃┃Keys:  ┃Values:┃┃"),
                String::from("┃          ┃┣━━━━━━━╋━━━━━━━┫┃"),
                String::from("┃          ┃┃InnerB1┃ValueB1┃┃"),
                String::from("┃          ┃┣━━━━━━━╋━━━━━━━┫┃"),
                String::from("┃          ┃┃InnerB2┃ValueB2┃┃"),
                String::from("┃          ┃┣━━━━━━━╋━━━━━━━┫┃"),
                String::from("┃          ┃┃InnerB3┃ValueB3┃┃"),
                String::from("┃          ┃┗━━━━━━━┻━━━━━━━┛┃"),
                String::from("┗━━━━━━━━━━┻━━━━━━━━━━━━━━━━━┛"),
            ]);
        }
    
    }
    
    mod string_stylizer_tests {
        #[test]
        fn test_string_formatting() {
            //use crate::string_stylizing::{StringStyle, StringBlinkSpeed, StringColor};
            use crate::string_stylizing::*;

            //order does matter:
            let result = crate::string_stylizing::format_string(
                &String::from("hello!"), 
                &StringStyle::default()
                    .set_text_color(&StringColor::Blue)
                    .set_bold(false)
                    .set_bold(true)
                    .set_italicized(true)
                    .set_strikethrough(true)
                    .set_italicized(false)
                    .set_blink_speed(&StringBlinkSpeed::Slow)
            );
    
            assert_eq!(result, "\x1b[1;9;5;34mhello!\u{1b}[0m")
        }
    }
}