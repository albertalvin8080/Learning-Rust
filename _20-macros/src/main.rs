use _20_macros::{code_block, header, unordered_list};

fn main() {
    let content = [
        header!("Detestatio Sacrorum"),
        code_block!("std::cout << \"Summa Blasphemia\" << std::endl;"),
        code_block!(
            lang = "c++",
            "std::cout << \"Summa Blasphemia\" << std::endl;"
        ),
        unordered_list!("Franz", "Helmuth", "Jakub"),
    ]
    .join("\n\n");

    println!("{content}");
}
