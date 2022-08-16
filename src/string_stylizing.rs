/// A struct which holds stylizing options for Strings formatted through ANSI
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StringStyle {
    bold: bool,
    faint: bool,
    italicized: bool,
    underline: bool,
    strikethrough: bool,

    blink: StringBlinkSpeed,
    color: StringColor,
    background_color: StringColor,
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl StringStyle {
    pub fn set_bold(&self, bold: bool) -> StringStyle {
        StringStyle {bold, .. *self}
    }

    pub fn set_italicized(&self, italicized: bool) -> StringStyle {
        StringStyle {italicized, .. *self}
    }

    pub fn set_underline(&self, underline: bool) -> StringStyle {
        StringStyle {underline, .. *self}
    }

    pub fn set_strikethrough(&self, strikethrough: bool) -> StringStyle {
        StringStyle {strikethrough, .. *self}
    }

    pub fn set_faint(&self, faint: bool) -> StringStyle {
        StringStyle {faint, .. *self}
    }

    pub fn set_text_color(&self, text_color: &StringColor) -> StringStyle {
        StringStyle {color: *text_color, .. *self}
    }

    pub fn set_background_color(&self, background_color: &StringColor) -> StringStyle {
        StringStyle {background_color: *background_color, .. *self}
    }

    pub fn set_blink_speed(&self, blink_speed: &StringBlinkSpeed) -> StringStyle {
        StringStyle {blink: *blink_speed, .. *self}
    }
}

impl Default for StringStyle {
    /// Constructs a new, default StringStyle
    /// 
    /// Applying this style directly to a string will not affect it
    /// StringStyle methods must be run to change the state of the StringStyle
    fn default() -> StringStyle {
        StringStyle {
            bold: false,
            faint: false,
            italicized: false,
            underline: false,
            strikethrough: false,

            blink: StringBlinkSpeed::None,
            color: StringColor::None,
            background_color: StringColor::None,
        }
    }
}

/// All basic colors as defined by SGR

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StringColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    Pink,
    Lime,
    BrightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    BrightWhite,
    None,
}

/// All blink speeds as defined by SGR

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StringBlinkSpeed {
    None,
    Slow,
    Fast, // Not widely supported as a formatting option
}

/// Printed strings are stylized using ANSI escape code SGR (Select Graphic Rendition) parameters
/// 
/// As specified [here](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters)
/// 
/// # Examples
/// 
/// ## Default Style
/// If no stylizing options are provided, the function will return the original string
/// ```
/// use formatted_debug::string_stylizing::*;
/// 
/// let result = format_string(
///     &String::from("hello!"),
///     &StringStyle::default()
/// );
/// assert_eq!(result, "hello!");
/// ```
/// 
/// ## Simple Use
/// 
/// ```
/// use formatted_debug::string_stylizing::*;
/// 
/// let result = format_string(
///     &String::from("hello!"), 
///     &StringStyle::default()
///         .set_text_color(&StringColor::Red)
/// );
/// assert_eq!(result, "\x1b[31mhello!\u{1b}[0m");
/// ```
/// 
/// ## Multiple
/// Styles can have multiple changes applied
/// ```
/// use formatted_debug::string_stylizing::*;
/// 
/// let result = format_string(
///     &String::from("hello!"), 
///     &StringStyle::default()
///         .set_text_color(&StringColor::Blue)
///         .set_bold(true)
///         .set_strikethrough(true)
///         .set_blink_speed(&StringBlinkSpeed::Slow)
/// );
/// assert_eq!(result, "\x1b[1;9;5;34mhello!\u{1b}[0m")
/// ```
pub fn format_string(unformatted_string: &String, style: &StringStyle) -> String {    
    if style.eq(&StringStyle::default()) {
        // No formatting required, return input string
        return unformatted_string.to_string();
    }

    //Hex escape codes reqired
    let mut prepended_formatting = String::from("\x1b[");

    if style.bold { prepended_formatting.push_str("1;"); }
    if style.faint { prepended_formatting.push_str("2;"); }
    if style.italicized { prepended_formatting.push_str("3;"); }
    if style.underline { prepended_formatting.push_str("4;"); }
    if style.strikethrough { prepended_formatting.push_str("9;"); }
    
    {
        use StringBlinkSpeed::*;

        prepended_formatting.push_str(match style.blink {
            None => "",
            Slow => "5;",
            Fast => "6;",
        });
    }

    {
        use StringColor::*;

        prepended_formatting.push_str(match style.color {
            Black => "30;",
            Red => "31;",
            Green => "32;",
            Yellow => "33;",
            Blue => "34;",
            Magenta => "35;",
            Cyan => "36;",
            White => "37;",
            Gray => "90;",
            Pink => "91;",
            Lime => "92;",
            BrightYellow => "93;",
            LightBlue => "94;",
            LightMagenta => "95;",
            LightCyan => "96;",
            BrightWhite => "97;",
            None => "",
        });
        
        prepended_formatting.push_str(match style.background_color {
            Black => "40;",
            Red => "41;",
            Green => "42;",
            Yellow => "43;",
            Blue => "44;",
            Magenta => "45;",
            Cyan => "46;",
            White => "47;",
            Gray => "100;",
            Pink => "101;",
            Lime => "102;",
            BrightYellow => "103;",
            LightBlue => "104;",
            LightMagenta => "105;",
            LightCyan => "106;",
            BrightWhite => "107;",
            None => "",
        });
    }

    prepended_formatting.pop(); //remove excess ;'s

    prepended_formatting.push('m');

    let mut formatted_string = unformatted_string.to_string();
    
    formatted_string.insert_str(0, &prepended_formatting[..]);
    formatted_string.push_str("\x1b[0m"); //reset formatting
    
    formatted_string
}