# cuda-self-model

Agent self-model — metacognition, capability calibration, limitation awareness, growth tracking (Rust)

Part of the Cocapn fleet — a Lucineer vessel component.

## What It Does

### Key Types

- `Capability` — core data structure
- `Limitation` — core data structure
- `InternalState` — core data structure
- `GrowthRecord` — core data structure
- `SelfModel` — core data structure
- `CapabilityAssessment` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-self-model.git
cd cuda-self-model

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_self_model::*;

// See src/lib.rs for full API
// 12 unit tests included
```

### Available Implementations

- `Capability` — see source for methods
- `InternalState` — see source for methods
- `GrowthRecord` — see source for methods
- `SelfModel` — see source for methods

## Testing

```bash
cargo test
```

12 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: other
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates


## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
