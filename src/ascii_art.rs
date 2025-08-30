use std::fmt;

pub struct AsciiArt;

impl AsciiArt {
    pub fn display_banner() {
        println!("{}", Self::get_utf_banner());
        println!("{}", Self::get_signature());
        println!();
    }
    
    pub fn display_banner_colored() {
        // ANSI color codes
        let cyan = "\x1b[36m";
        let yellow = "\x1b[33m";
        let green = "\x1b[32m";
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        
        println!("{}{}{}", cyan, Self::get_developer_favorite_logo(), reset);
        println!("{}{}{}", green, Self::get_framework_name(), reset);
        println!("{}{}{}", yellow, Self::get_dev_signature(), reset);
        println!();
    }
    
    pub fn display_banner_pure_signature() {
        // ANSI color codes with different colors for different chars
        let green = "\x1b[32m";   // For $
        let yellow = "\x1b[33m";  // For @
        let red = "\x1b[31m";     // For !
        let blue = "\x1b[34m";    // For m
        let magenta = "\x1b[35m"; // For ₹
        let reset = "\x1b[0m";
        
        println!("{}{}{}", magenta, Self::get_utf_pure_signature(), reset);
        println!();
    }
    
    pub fn display_banner_signature_only() {
        // ANSI color codes
        let magenta = "\x1b[35m";
        let cyan = "\x1b[36m";
        let yellow = "\x1b[33m";
        let reset = "\x1b[0m";
        
        println!("{}{}{}", magenta, Self::get_utf_banner_signature_chars(), reset);
        println!();
    }
    
    fn get_utf_banner() -> &'static str {
        r#"
 @@@@   @@@@@@@@@@@@@@@@@@@
 @@@@   @@@@@@@@@@@@@@@@@@@
 @@@@   @@@@   @@@@@@@@@@@@ 
 @@@@   @@@@   @@@@@@@@@@@@ 
 @@@@@@@@@@@   @@@@@        
  @@@@@@@@@    @@@@@        
        "#
    }
    
    fn get_utf_banner_simple() -> &'static str {
        r#"
 _   _ _____ _____ 
| | | |_   _|  ___|
| | | | | | | |_   
| |_| | | | |  _|  
 \___/  |_| |_|    
        "#
    }
    
    fn get_utf_banner_stylized() -> &'static str {
        r#"
   __  __  ______  ______
  /  \/  \/\__  _\/\  ___\
 /  /\  /\   /\ \/\ \  __\ 
 \  \  \     \ \_\ \ \_\  \
  \__/\__/     \/_/  \/_/  
        "#
    }
    
    fn get_utf_banner_custom() -> &'static str {
        r#"
 ╔═══╗ ╔═══╗ ╔═══╗
 ║ ╔═╝ ║   ║ ║  ═╣
 ║ ╚═╗ ║   ║ ║ ╔═╝
 ╚═══╝ ╚═══╝ ╚═╝  
        "#
    }
    
    fn get_utf_banner_samir_style() -> &'static str {
        r#"
 ██╗   ██╗ ████████╗ ███████╗
 ██║   ██║ ╚══██╔══╝ ██╔════╝
 ██║   ██║    ██║    █████╗  
 ██║   ██║    ██║    ██╔══╝  
 ╚██████╔╝    ██║    ██║     
  ╚═════╝     ╚═╝    ╚═╝     
                             
 ╔══════════════════════════╗
 ║  Unified Test Framework  ║
 ╚══════════════════════════╝
        "#
    }
    
    pub fn get_utf_banner_signature_chars() -> &'static str {
        r#"
 $$$    $$$@@@@@@@@@@@@@@@@@ 
 $$$    $$$@@@@@@@@@@@!!!!!! 
 $$$    $$$   @@@@@@@@!!!    
 $$$    $$$   @@@@@@@@!!!    
 $$$mmm$$$    @@@@@m!!!!     
  $mmmmm$     @@@@@          
                             
  ₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹ 
  ₹ Unified Test Framework ₹ 
  ₹   Created by $@m!₹      ₹ 
  ₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹ 
        "#
    }
    
    pub fn get_utf_pure_signature() -> &'static str {
        r#"
 $   $ @@@@@@@ !!!!!!!
 $   $ @@      !      
 $   $ @@@@@@@ !!!!!! 
 $   $      @@ !      
 $$$$$$ @@@@@@@ !      
                       
  ₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹ 
  ₹   UTF Framework   ₹ 
  ₹    by $@m!₹       ₹ 
  ₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹₹ 
        "#
    }
    
    pub fn get_signature() -> &'static str {
        "                    by $@m!₹"
    }
    
    pub fn get_developer_favorite_logo() -> &'static str {
        r#"
██╗   ██╗████████╗███████╗
██║   ██║╚══██╔══╝██╔════╝
██║   ██║   ██║   █████╗  
██║   ██║   ██║   ██╔══╝  
╚██████╔╝   ██║   ██║     
 ╚═════╝    ╚═╝   ╚═╝     
        "#
    }
    
    pub fn get_framework_name() -> &'static str {
        "    🚀 Unified Test Framework"
    }
    
    pub fn get_dev_signature() -> &'static str {
        "    Created with ❤️  by samirparhi-dev"
    }
    
    pub fn get_tagline() -> &'static str {
        "    Unified Test Framework"
    }
    
    pub fn get_minimal_banner() -> String {
        format!(
            r#"
╭─────────────────────────╮
│      UTF Framework      │
│   by $@m!₹ (Samir)      │
╰─────────────────────────╯
            "#
        )
    }
    
    pub fn get_compact_banner() -> String {
        format!(
            r#"
 ┌─ UTF ─┐  Unified Test Framework
 │  $@m  │  Generate tests with real logic  
 └───!₹──┘  Industry-standard coverage
            "#
        )
    }
    
    pub fn get_stylized_banner() -> String {
        format!(
            r#"
    ██╗   ██╗████████╗███████╗
    ██║   ██║╚══██╔══╝██╔════╝
    ██║   ██║   ██║   █████╗  
    ██║   ██║   ██║   ██╔══╝  
    ╚██████╔╝   ██║   ██║     
     ╚═════╝    ╚═╝   ╚═╝     
                             
    Unified Test Framework   
         by $@m!₹            
            "#
        )
    }
    
    pub fn get_signature_full() -> String {
        format!(
            r#"
╔════════════════════════════════╗
║        UTF Framework           ║
║   Generate • Analyze • Test    ║  
║                                ║
║        Crafted by $@m!₹        ║
║         (Samir Parhi)          ║
╚════════════════════════════════╝
            "#
        )
    }
}

impl fmt::Display for AsciiArt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::get_stylized_banner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_banner_generation() {
        let banner = AsciiArt::get_stylized_banner();
        assert!(banner.contains("UTF"));
        assert!(banner.contains("$@m!₹"));
    }
    
    #[test]
    fn test_signature() {
        let sig = AsciiArt::get_signature();
        assert!(sig.contains("$@m!₹"));
    }
}