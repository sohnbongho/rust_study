use std::io;

fn main() {
    println!("가위 바위 보 중 하나를 선택하세요: (가위, 바위, 보)");

    let mut decision = String::new();

    io::stdin()
        .read_line(&mut decision)
        .expect("입력 오류 발생");

    println!("선택한 값: {decision}");
}
