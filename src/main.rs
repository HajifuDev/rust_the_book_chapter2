// 標準入出力ライブラリを追加
use std::io;
// Ordering型をスコープに追加
use std::cmp::Ordering;
// 乱数生成クレートを追加
use rand::Rng;

// エントリポイントになる特別な関数名
fn main() {
    // !がついている関数はマクロ？になる
    println!("Guess the number!");          // 数を当ててごらん

    // thread_rngは乱数生成器。
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret number is {}.", secret_number);

    loop {
        println!("Please input your guess.");   // ほら、予想を入力してね
    
        // 可変(mut)の引数 guess を定義
        // new は String型の関連関数。guessの値は、空のStringオブジェクト
        let mut guess = String::new();
    
        // もし冒頭のuseがなければ std::io::stdin()となる
        // read_lineに引数として、ユーザーの入力を格納する文字列を変数として受け取る
        // &は参照を意味する。参照も不変なので mut で可変として扱う必要がある（詳しくは後の章）
        // read_lineメソッドは io::Result 型を返す。Err列挙子にはエラーが起きた際のエラー情報が入っており、
        // それを使ってハンドリングできる。expectを呼び出していないと、エラーの可能性に対処していないということでコンパイルエラーになる。
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");     // 行の読み込みに失敗しました
    
        // 不正な入力値の場合は再入力を促す。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("no, please type number.");
                continue;
            }
        };
    
        println!("You guessed: {}", guess);     // 次のように予想しました: {}
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small !"),
            Ordering::Equal => {
                println!("You Win !");
                break;
            },
            Ordering::Greater => println!("Too Big !"),
        }
    }

}
