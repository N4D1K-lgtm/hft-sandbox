# HFT Sandbox Project

<!--toc:start-->
- [HFT Sandbox Project](#hft-sandbox-project)
  - [Project Overview](#project-overview)
  - [Roadmap](#roadmap)
    - [Phase 1 - Market Data](#phase-1-market-data)
      - [Communication Protocols](#communication-protocols)
      - [1.2 ITCH Protocol](#12-itch-protocol)
  - [Performance Goals](#performance-goals)
  - [Development Approach](#development-approach)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Building the Project](#building-the-project)
    - [Running Tests](#running-tests)
  - [License](#license)
<!--toc:end-->

## Project Overview

This is a collection of educational projects designed to explore High-Frequency Trading (HFT) systems and adjacent technology.

The primary objectives are:

- Learn about the technical complexities of low-latency trading systems.
- Implement performant data structures and algorithms in the real world.
- Simulate market interactions at a realistic scale.
- Research and possibly test algorithmic trading strategies.

## Roadmap

| Component | Role | Status |
|-----------|------|--------|
| Market Data Handler | Data aggregation/ingest | In Progress |
| Limit Order Book (LOB) | Internal market representation | Planned |
| Strategy Engine | Quantitative analysis, trade strategy execution | Planned |
| Order Management System (OMS) | Trade execution and tracking | Planned |
| Simulated Exchange | Simulate live market interactions | Planned |
| Backtester | Historical simulation and strategy validation | Planned |

### Phase 1 - Market Data

> **Objective:** Build a set of initial components in order to parse raw,
> high-volume data streams from a financial exchange into a performant
> in-memory market model.

#### Communication Protocols

Before market data can be processed, it must be decoded from an incoming message
stream. Lots of complexity happens within an exchange: new trades, cancelled
trades, orders, Initial Public Offerings (IPO), etc. The primary design challenge
of market data communication is finding the most efficient way of transmitting
this information without compromising its utility.

#### 1.2 ITCH Protocol

ITCH (Integrated Trading System) is a strictly outbound communication protocol

```

## License

See the LICENSE file for details.

