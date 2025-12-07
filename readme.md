# Rust Playground
## Installation on Mac OS

`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

### Linker
러스트의 컴파일 결과를 하나의 파일로 묶기 위해 사용함
몇가지 흔히 사용되는 러스트 패키지가 C 코드를 이용하기 때문에 C 컴파일러에서 사용하는 Linker를 설치해 해결 가능

`xcode-select --install`

## De-Rust

`rustup self uninstall`

## Start to run
### 1. cargo 생성
- `cargo new '카고이름'`
### 2. cargo 실행 또는 빌드
- `cargo build`
- `cargo run`
### 3. 컴파일 가능성 확인
- `cargo check`

## 빌드 시 버전
- 단순 build:
  - 개발 도중이며 실행 후 동작 테스트 등 빌드가 잦은 경우
- release build:
  - 배포 시 사용하며 잦은 실행보단 정확하고 성능이 보장되어야 하는 경우
  - `cargo build --release`