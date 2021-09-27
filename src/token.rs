// トークン列
// デバック出力と、オブジェクト同士が等価であることの比較を行うための振る舞いを継承
#[derive(Debug, PartialEq)]
pub enum Token {
    // ポインタをインクリメントする。ポインタをptrとすると、C言語の「ptr++;」に相当する。
    PointerIncrement,
    // ポインタをデクリメントする。C言語の「ptr--;」に相当。
    PointerDecrement,
    // ポインタが指す値をインクリメントする。C言語の「(*ptr)++;」に相当。
    PointerIncrementValue,
    // ポインタが指す値をデクリメントする。C言語の「(*ptr)--;」に相当。
    PointerDecrementValue,
    // ポインタが指す値をデクリメントする。C言語の「(*ptr)--;」に相当。
    Output,
    // ポインタが指す値を出力に書き出す。C言語の「putchar(*ptr);」に相当。
    Input,
    // ポインタが指す値が0なら、対応する ] の直後にジャンプする。C言語の「while(*ptr){」に相当。
    LoopBegin,
    // ポインタが指す値が0でないなら、対応する [ （の直後[注釈 1]）にジャンプする。C言語の「}」に相当
    LoopEnd,
}

// 字句解析器
pub fn lexer(src: &str) -> Vec<Token> {
    // トークンの空配列を作る
    let mut v = vec![];
    // 引数の文字を一文字ずつ見て、Token列挙体の値に変えてトークン配列に追加
    for char in src.chars() {
        v.push(match char {
            '>' => Token::PointerIncrement,
            '<' => Token::PointerDecrement,
            '+' => Token::PointerIncrementValue,
            '-' => Token::PointerDecrementValue,
            '.' => Token::Output,
            ',' => Token::Input, 
            '[' => Token::LoopBegin,
            ']' => Token::LoopEnd,
            _ => continue,
        });
    }
    v
}