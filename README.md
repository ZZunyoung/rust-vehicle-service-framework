# Rust Vehicle Service Framework

Linux 환경에서 여러 독립 프로세스가 이벤트 기반으로 통신하는 Rust 시스템 프레임워크 실험 프로젝트입니다.

이 프로젝트의 목표는 자동차 도메인 서비스를 깊게 구현하는 것이 아니라, 차량용 시스템 프레임워크에서 요구되는 Runtime, IPC, Event Bus, Service Registry, Heartbeat, Health Monitor 같은 기반 구조를 직접 설계하고 구현하는 것입니다.

## 현재 구현 상태

- Cargo workspace 구성
- Unix Domain Socket 기반 IPC
- Runtime 프로세스
- 서비스 등록 및 해제
- Event publish/dispatch
- Service Registry
- Heartbeat
- Health check scheduler
- ServiceDown 감지
- Graceful shutdown
- 예제 서비스
  - `door`
  - `display`
  - `logger`

현재 동작하는 핵심 흐름은 다음과 같습니다.

```text
door
  -> Publish(DoorOpened)
  -> launcher Runtime
  -> EventBus
  -> display Dispatch(DoorOpened)
  -> logger Dispatch(DoorOpened)
```

## Workspace 구조

```text
.
├── framework/
│   └── src/
│       ├── config.rs
│       ├── event.rs
│       ├── event_bus.rs
│       ├── ipc.rs
│       ├── message.rs
│       ├── registry.rs
│       ├── runtime.rs
│       ├── scheduler.rs
│       └── service_api.rs
├── launcher/
│   └── src/main.rs
├── services/
│   ├── door/
│   ├── display/
│   └── logger/
├── Cargo.toml
└── Rust_Vehicle_Service_Framework_Updated.md
```

## 주요 모듈

### `framework::event`

서비스 간에 주고받는 이벤트 타입을 정의합니다.

현재 이벤트:

- `DoorOpened`
- `EngineStarted`
- `FuelLow`
- `ReverseGear`
- `ServiceDown { service_name }`
- `Heartbeat`
- `Shutdown`

`ServiceDown`은 payload를 포함하므로 어떤 서비스가 죽었는지 logger가 기록할 수 있습니다.

### `framework::message`

IPC로 전달되는 메시지 타입입니다.

- `Register`: 서비스 등록
- `Publish`: 서비스가 Runtime에 이벤트 발행
- `Dispatch`: Runtime이 구독 서비스에 이벤트 전달
- `Heartbeat`: 서비스 생존 신호
- `Shutdown`: 정상 종료 요청

### `framework::ipc`

Unix Domain Socket 기반 메시지 송수신을 담당합니다.

메시지는 `serde_json`으로 직렬화되고, 한 줄 단위로 전송됩니다.

```text
Runtime socket:
/tmp/vehicle-framework.sock

Service socket:
/tmp/vehicle-framework-{service_name}.sock
```

### `framework::registry`

서비스 상태와 구독 정보를 관리합니다.

저장하는 정보:

- 서비스 이름
- 구독 이벤트 목록
- 서비스 socket path
- 마지막 heartbeat 시각
- 서비스 상태
  - `Alive`
  - `Down`

### `framework::runtime`

Runtime의 핵심 로직입니다.

처리하는 작업:

- 서비스 등록
- 이벤트 발행 처리
- 이벤트 dispatch
- heartbeat 갱신
- health check
- service down 감지
- graceful shutdown 처리

### `framework::scheduler`

주기 작업 실행을 담당합니다.

현재는 `health-check` job이 Runtime에서 주기적으로 실행됩니다.

### `framework::service_api`

서비스가 Runtime 내부 구조를 직접 알지 않고 사용할 수 있는 API입니다.

현재 제공 API:

- `register()`
- `publish()`
- `heartbeat()`
- `shutdown()`
- `service_socket_path()`

## 실행 방법

먼저 전체 빌드를 확인합니다.

```bash
cargo check --workspace
```

터미널을 여러 개 열고 아래 순서대로 실행합니다.

터미널 1: Runtime 실행

```bash
cargo run -p launcher
```

터미널 2: Display 서비스 실행

```bash
cargo run -p display
```

터미널 3: Logger 서비스 실행

```bash
cargo run -p logger
```

터미널 4: Door 이벤트 발행

```bash
cargo run -p door
```

정상 동작하면 Runtime 쪽에서 `DoorOpened` 이벤트가 `display`, `logger`로 dispatch되고, 각 서비스 터미널에서 이벤트 수신 로그가 출력됩니다.

## 동작 시나리오

### DoorOpened

```text
door
  -> Runtime에 Publish(DoorOpened)
  -> Runtime이 Registry에서 DoorOpened 구독자 조회
  -> display에 Dispatch(DoorOpened)
  -> logger에 Dispatch(DoorOpened)
```

### Heartbeat

`display`와 `logger`는 주기적으로 Runtime에 heartbeat를 보냅니다.

```text
display/logger
  -> Heartbeat
  -> Runtime
  -> last_heartbeat 갱신
```

### Crash Detection

`display` 또는 `logger`를 `Ctrl+C`로 종료하면 Runtime은 heartbeat timeout 이후 해당 서비스를 `Down`으로 표시합니다.

```text
service crash
  -> heartbeat 중단
  -> Runtime health-check timeout
  -> ServiceDown { service_name }
  -> logger에 dispatch
```

### Graceful Shutdown

`display` 또는 `logger` 터미널에서 Enter를 누르면 서비스가 Runtime에 `Shutdown` 메시지를 보내고 정상 unregister됩니다.

```text
Enter
  -> Shutdown
  -> Runtime unregister
```

## 현재 한계

- Runtime이 서비스 프로세스를 직접 spawn/restart하지는 않습니다.
- 자동 재시작 정책은 아직 없습니다.
- 로그는 `println!`/`eprintln!` 기반입니다.
- 단위 테스트가 아직 없습니다.
- 서비스별 socket listener 시작 순서에 아주 짧은 race condition 가능성이 있습니다.
- `logger`의 heartbeat interval은 아직 config 함수로 완전히 정리되지 않았을 수 있습니다.

## 다음 작업 후보

우선순위가 높은 다음 작업:

1. `logger`의 heartbeat interval을 `framework::config::heartbeat_interval()`로 통일
2. `cargo fmt` 적용
3. 단위 테스트 추가
   - `Event::same_kind`
   - `ServiceRegistry::register`
   - `ServiceRegistry::subscribers_for`
   - heartbeat timeout
   - unregister
4. Logging 모듈 추가
5. Launcher가 서비스 프로세스를 직접 실행하도록 확장
6. Retry/Restart 정책 구현
7. Docker Compose 실행 환경 추가
8. Benchmark 모드 추가

## 프로젝트 방향

이 프로젝트의 중심은 예제 서비스가 아니라 Framework Runtime입니다.

서비스는 가능한 단순하게 유지하고, 아래 기능을 프레임워크 쪽에 축적하는 방향으로 개발합니다.

- IPC abstraction
- Event routing
- Service discovery
- Heartbeat
- Health monitor
- Crash detection
- Graceful shutdown
- Retry/restart policy
- Scheduler
- Logging
- Benchmark
