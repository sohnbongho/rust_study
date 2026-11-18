# rust_study

Rust 학습용 저장소입니다.

## 구성

| 경로 | 설명 |
| --- | --- |
| `main.rs` | `rustc`로 직접 컴파일하는 Hello, world! 예제 |
| `hello_cargo/` | Cargo로 만든 Hello, world! 프로젝트 (edition 2024) |
| `rock-paper-scissors/` | 표준 입력으로 가위/바위/보를 입력받는 예제 (edition 2024) |

## 실행 방법

### rustc로 직접 컴파일

```sh
rustc main.rs
./main        # Windows: .\main.exe
```

### Cargo 프로젝트

```sh
cd hello_cargo
cargo build   # 빌드
cargo run     # 빌드 후 실행
cargo check   # 컴파일 가능 여부만 빠르게 확인
```

## 요구 사항

- [Rust](https://www.rust-lang.org/tools/install) (rustup으로 설치, edition 2024를 지원하는 1.85 이상)
