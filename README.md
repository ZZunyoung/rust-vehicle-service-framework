# Rust Vehicle Service Framework

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

### `framework::message`

IPC로 전달되는 메시지 타입입니다.

- `Register`: 서비스 등록
- `Publish`: 서비스가 Runtime에 이벤트 발행
- `Dispatch`: Runtime이 구독 서비스에 이벤트 전달
- `Heartbeat`: 서비스 생존 신호
- `Shutdown`: 정상 종료 요청

### `framework::ipc`

Unix Domain Socket 기반 메시지 송수신을 담당합니다.

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

### `framework::service_api`

현재 제공 API:

- `register()`
- `publish()`
- `heartbeat()`
- `shutdown()`
- `service_socket_path()`

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

```text
display/logger
  -> Heartbeat
  -> Runtime
  -> last_heartbeat 갱신
```

### Crash Detection

```text
service crash
  -> heartbeat 중단
  -> Runtime health-check timeout
  -> ServiceDown { service_name }
  -> logger에 dispatch
```

### Graceful Shutdown

```text
Enter
  -> Shutdown
  -> Runtime unregister
```
