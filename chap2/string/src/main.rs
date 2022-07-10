fn main() {
    // let s = String::from("hello world!");
    
    // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("{}, {}", hello, world);

    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear();

    // println!("the first word is: {}", word);

    // let s1 = String::from("hello");
    // let h = &s1[..2];
    // println!("{}", h);

    // let mut s = String::from("hello ");
    // s.push('r');
    // println!("追加字符串 push() -> {}", s);

    // s.push_str("ust!");
    // println!("追加字符串 push_str() -> {}", s);

    // let mut s = String::from("Hello rust!");
    // s.insert(5, ',');
    // println!("插入字符 insert() -> {}", s);
    // s.insert_str(6, " I like");
    // println!("插入字符串 insert_str() -> {}", s);

    // let string_replace = String::from("I like rust, Learning rust is my favourite!");
    // let new_string_replace = string_replace.replace("rust", "Rust");
    // dbg!(new_string_replace);

    // let mut string_pop = String::from("rust pop 中文!");
    // let p1 = string_pop.pop();
    // let p2 = string_pop.pop();
    // let p3 = string_pop.pop();
    // dbg!(p1);
    // dbg!(p2);
    // dbg!(string_pop);

    // let string_append = String::from("hello ");
    // let string_rust = String::from("rust ");
    // &string_rust 会自动解引用为&rust
    // let result = string_append + &string_rust;
    // let mut result = result + "!";
    // result += "!!!";

    // println!("连接字符串 + -> {}", result);

        // 通过 \ + 字符的十六进制表示，转义输出一个字符
        // let byte_escape = "I'm writing \x52\x75\x73\x74!";
        // println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    
        // // \u 可以输出一个 unicode 字符
        // let unicode_codepoint = "\u{211D}";
        // let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    
        // println!(
        //     "Unicode character {} (U+211D) is called {}",
        //     unicode_codepoint, character_name
        // );
    
        // // 换行了也会保持之前的字符串格式
        // let long_string = "String literals
        //                     can span multiple lines.
        //                     The linebreak and indentation here ->\
        //                     <- can be escaped too!";
        // println!("{}", long_string);

    // for c in "中国人".chars() {
    //     println!("{}", c);
    // }

        // 通过 \ + 字符的十六进制表示，转义输出一个字符
        let byte_escape = "I'm writing \x52\x75\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    
        // \u 可以输出一个 unicode 字符
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    
        println!(
            "Unicode character {} (U+211D) is called {}",
            unicode_codepoint, character_name
        );
    
        // 换行了也会保持之前的字符串格式
        let long_string = "String literals
                            can span multiple lines.
                            The linebreak and indentation here ->\
                            <- can be escaped too!";
        println!("{}", long_string);
}

// fn first_word(s: &String) -> &str {
//     &s[..1]
// }
