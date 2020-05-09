use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_rust_ownership() {
    // 文字列"101", "102", ... "105"のベクタを作成
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // 以下は参照を使わずにインデックスされた値を取り出す操作
    let fifth = v.pop().unwrap(); // ベクタの最後から値をpopして取り出す
    assert_eq!(fifth, "105");

    let second = v.swap_remove(1); // ベクタの真ん中を抜き出し、最後の要素がそこに代わりに入るように操作
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v[2], "substitute".to_string()); // 3番目の要素を取り出して代わりに別の値を入れる
    assert_eq!(third, "103");

    assert_eq!(v, vec!["101", "104", "substitute"]); // 最後に残ったベクタの中身

    struct Person { name: Option<String>, birth: i32 }
    let mut composers = Vec::new();
    composers.push(Person { name: Some("Palestrina".to_string()), birth: 1525 });

    // nameの所有権をcomposersからローカル変数に移すことはできなくてエラーになる
    // let first_name = composers[0].name;
    // let first_name = std::mem::replace(&mut composers[0].name, None); // nameはOptionのためNoneに置き換えが可能
    let first_name = composers[0].name.take(); // takeメソッドはNoneへのreplaceを行うショートハンド
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);
    assert_eq!(composers[0].birth, 1525);

    // 参照カウント型で所有権を共有する
    let s: Rc<String> = Rc::new("shirataki".to_string()); // Rc型はイミュータブルで作成される
    let t: Rc<String> = s.clone(); // sの参照カウントが+1
    let u: Rc<String> = s.clone(); // sの参照カウントが+1

    // StringのメソッドはRc<String>にも直接実行できる
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);

    // Rc<String>は不変であるためテキストの追加はコンパイラに拒否される
    // s.push_str(" noodles");
}
