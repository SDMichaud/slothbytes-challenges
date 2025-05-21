//! # Remove Virus
//! 
//! ## Challenge Description
//! 
//! Your computer might have been infected by a virus! Create a function that finds the viruses in files and removes them from your computer. 
//! - Bad files will contain "virus" or "malware", but "antivirus" and "notvirus" will not be viruses.
//! - Return "PC Files: Empty" if there are no files left on the computer.

/// Removes virus file names from a string with format "PC Files: comma, separated, names".  
/// Viruses will always contain "virus" or "malware" but "antivirus" and "notvirus" are exceptions.  
/// If all file names are removed, "PC Files: Empty" is returned.
/// # Examples
///
/// ```
/// use slothbytes::remove_virus;
/// 
/// // virus.exe is removed
/// let s = "PC Files: spotifysetup.exe, virus.exe, dog.jpg";
/// let expected = "PC Files: spotifysetup.exe, dog.jpg";
/// assert_eq!(remove_virus(s), expected);
/// 
/// // antivirus.exe is not removed
/// let s = "PC Files: antivirus.exe, cat.pdf, lethalmalware.exe, dangerousvirus.exe";
/// let expected = "PC Files: antivirus.exe, cat.pdf";
/// assert_eq!(remove_virus(s), expected);
/// 
/// // all files removed
/// let s = "PC Files: lethalmalware.exe, dangerousvirus.exe";
/// let expected = "PC Files: Empty";
/// assert_eq!(remove_virus(s), expected);
/// ```
pub fn remove_virus(s: &str) -> String {
    let file_names = s.split("PC Files: ").last().expect("Expected string to be formated properly.").split(",");
    let mut good_files: Vec<&str> = Vec::new();
    for file_name in file_names {
        let file_name = file_name.trim();
        if file_name.contains("antivirus") || file_name.contains("notvirus") {
            good_files.push(file_name);
            // println!("Added file: {file_name}");
        } else if !file_name.contains("virus") && !file_name.contains("malware") {
            good_files.push(file_name);
            // println!("Added file: {file_name}");
        }
    }
    if good_files.len() == 0 {
        return String::from("PC Files: Empty");
    }
    format!("PC Files: {}", good_files.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_virus_exe() {
        let s = "PC Files: spotifysetup.exe, virus.exe, dog.jpg";
        let expected = "PC Files: spotifysetup.exe, dog.jpg";
        assert_eq!(remove_virus(s), expected);
    }

    #[test]
    fn keep_antivirus_exe() {
        let s = "PC Files: antivirus.exe, cat.pdf, lethalmalware.exe, dangerousvirus.exe";
        let expected = "PC Files: antivirus.exe, cat.pdf";
        assert_eq!(remove_virus(s), expected);
    }

    #[test]
    fn remove_all_files() {
        let s = "PC Files: lethalmalware.exe, dangerousvirus.exe";
        let expected = "PC Files: Empty";
        assert_eq!(remove_virus(s), expected);
    }
}