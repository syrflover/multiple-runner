테스트 함수를 주어진 횟수만큼 실행해주는 `attribute`입니다.

랜덤 값을 생성하여 테스트를 하는 경우에 사용합니다.

## Example

```rust
use multiple_runner::multiple_runner;

#[multiple_runner] // or #[multiple_runner(100)]
#[test]
fn add_one() -> anyhow::Result<()> {
    assert_eq!(1 + 1, 2);

    Ok(())
}

#[multiple_runner]
#[tokio::test]
async fn add_two() -> anyhow::Result<()> {
    assert_eq!(1 + 2, 3);

    Ok(())
}
```