#[macro_export]
macro_rules! header {
    ($val:literal) => {{
        concat!("# ", $val)
    }};
}

#[macro_export]
macro_rules! code_block {
    ($val:literal) => {{
        concat!("```\n", $val, "\n```")
    }};
    (lang = $lang:literal, $val:literal) => {{
        concat!("```", $lang, "\n", $val, "\n```")
    }};
}

#[macro_export]
macro_rules! unordered_list {
    ($($val:literal),+) => {{
        concat!($(" - ", $val, "\n",)+).trim_end()
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_header() {
        let h = header!("My Title");
        assert_eq!(h, "# My Title");
    }

    #[test]
    fn test_code_block() {
        let c = code_block!("std::cout << \"Summa Blasphemia\" << std::endl;");
        assert_eq!(
            "```\nstd::cout << \"Summa Blasphemia\" << std::endl;\n```",
            c
        );
    }

    #[test]
    fn test_code_block_lang() {
        let c = code_block!(
            lang = "c++",
            "std::cout << \"Summa Blasphemia\" << std::endl;"
        );
        assert_eq!(
            "```c++\nstd::cout << \"Summa Blasphemia\" << std::endl;\n```",
            c
        );
    }

    #[test]
    fn test_unordered_list() {
        let ul = unordered_list!("Franz", "Helmuth", "Jakub");
        assert_eq!(" - Franz\n - Helmuth\n - Jakub", ul);
    }
}
