# praxis-echo

[![CI](https://github.com/dnacenta/praxis-echo/actions/workflows/ci.yml/badge.svg?branch=development)](https://github.com/dnacenta/praxis-echo/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/github/license/dnacenta/praxis-echo)](LICENSE)
[![Version](https://img.shields.io/github/v/tag/dnacenta/praxis-echo?label=version&color=green)](https://github.com/dnacenta/praxis-echo/tags)
[![crates.io](https://img.shields.io/crates/v/praxis-echo)](https://crates.io/crates/praxis-echo)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange)](https://rustup.rs/)

Pipeline enforcement for AI self-evolution. Keeps your document pipeline flowing — from raw research to integrated identity.

## Why

If you're building an AI entity that grows through documents — research notes, incubating thoughts, reflections, identity files — you need more than good intentions. Documents stagnate. Thoughts go stale. Questions pile up unanswered.

The core problem: **growth can't depend on the agent remembering to grow.** That's the same trap [recall-echo](https://github.com/dnacenta/recall-echo) solved for memory. praxis-echo solves it for the entire document lifecycle.

## How It Works

praxis-echo enforces a document pipeline via Claude Code hooks:

```
Encounter → LEARNING.md → THOUGHTS.md → REFLECTIONS.md → SELF.md / PRAXIS.md
             (capture)     (incubate)    (crystallize)     (integrate)
```

Three hooks run automatically:

| Hook | Trigger | What It Does |
|------|---------|-------------|
| `pulse` | Session start (PreToolUse) | Scans all documents, injects pipeline state into agent context |
| `checkpoint` | Context compaction (PreCompact) | Snapshots document state before context is compressed |
| `review` | Session end (SessionEnd) | Diffs session-start vs session-end, tracks pipeline movement |

The agent sees its pipeline state at the start of every session — document counts, staleness warnings, threshold breaches, frozen pipeline alerts. No manual checking required.

## Documents Tracked

| Document | Role | Soft Limit | Hard Limit |
|----------|------|-----------|------------|
| LEARNING.md | Research capture | 5 threads | 8 |
| THOUGHTS.md | Incubation | 5 active | 10 |
| CURIOSITY.md | Open questions | 3 open | 7 |
| REFLECTIONS.md | Crystallized observations | 15 entries | 20 |
| PRAXIS.md | Active behavioral policies | 5 active | 10 |
| SESSION-LOG.md | Session-level observations | 30 days | — |

## Install

### From crates.io (recommended)

```bash
cargo install praxis-echo
```

### From source

```bash
git clone https://github.com/dnacenta/praxis-echo.git
cd praxis-echo
cargo install --path .
```

### Initialize

```bash
praxis-echo init
```

This will:
- Create `~/.claude/praxis/` for state tracking
- Create `~/archives/` subdirectories for overflow content
- Install hooks into `~/.claude/settings.json`
- Deploy the pipeline protocol to `~/.claude/rules/praxis-echo.md`

## Commands

### `praxis-echo pulse`

Runs automatically at session start via PreToolUse hook. Scans all documents and outputs:

```
[PRAXIS — Pipeline State]
  LEARNING:    2 active threads
  THOUGHTS:    3 active, 1 graduated, 0 dissolved
  CURIOSITY:   2 open, 4 explored
  REFLECTIONS: 9 total (observations: 3)
  PRAXIS:      6 active, 0 retired ⚡ approaching limit
  LOG:         7 entries (2026-02-23 → 2026-02-26)
[END PRAXIS]
```

Includes staleness warnings (thoughts untouched >7 days) and frozen pipeline alerts (no movement in 3+ sessions).

### `praxis-echo status`

Visual health dashboard with progress bars:

```
praxis-echo — Pipeline Health

  LEARNING     ██░░░░░░░░░░░░░░░░░░  1/8
  THOUGHTS     ████░░░░░░░░░░░░░░░░  2/10
  CURIOSITY    █████░░░░░░░░░░░░░░░  2/7
  REFLECTIONS  █████████░░░░░░░░░░░  9/20
  PRAXIS       ████████████░░░░░░░░  6/10

  Pipeline Flow
    Last movement:  2026-02-26
    Graduations:    3
    Dissolutions:   1
    Archival ops:   0
```

### `praxis-echo scan [--format json]`

Deep inspection of all documents. Human-readable by default, JSON for programmatic use.

### `praxis-echo checkpoint`

Snapshots document state to `~/.claude/praxis/checkpoints/`. Runs automatically on context compaction.

### `praxis-echo review`

Post-session diff. Compares current document state against the session-start snapshot, tracks which documents changed, and updates the frozen pipeline counter.

### `praxis-echo archive [--dry-run]`

Checks all documents against hard limits. Creates archive markers in `~/archives/` for documents that need manual content migration. Use `--dry-run` to preview without changes.

### `praxis-echo nudge <topic> [--when +2h] [--priority normal]`

Queues a curiosity-driven research intent into the self-schedule system. Integrates with n8n's intent queue for AI-initiated research sessions.

```bash
praxis-echo nudge "emergence in complex systems" --when +2h --priority high
```

## Works With

- **[recall-echo](https://github.com/dnacenta/recall-echo)** — Persistent memory system. recall-echo handles memory, praxis-echo handles growth. Both coexist in the same `settings.json` hooks.
- **Claude Code** — Hooks integrate natively via `~/.claude/settings.json`
- **n8n** — Intent queue integration for self-scheduled research sessions

## License

AGPL-3.0
