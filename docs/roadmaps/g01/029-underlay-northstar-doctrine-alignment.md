# 029 - Underlay Northstar Doctrine Alignment

Status: Complete
Owner: Platform
Created: 2026-03-06
Depends on: 028

## Overview

Bring the Underlay docs system onto the Northstar contract while keeping `docs/` as the documentation authority inside the code repository.

## Decision

- [x] Keep the docs authority inside `docs/`
- [x] add `docs/vision/`, `docs/roadmaps/`, and `docs/logs/` as the Northstar core
- [x] preserve `architecture/`, `guides/`, `patterns/`, and `sweeps/` as supporting sections
- [x] remove the live `docs/roadmap/` and `docs/reports/` sections by migrating their contents into the new structure

## Checklist

- [x] create the `docs/vision/`, `docs/roadmaps/`, and `docs/logs/` spine
- [x] move numbered roadmap files into `docs/roadmaps/g01/`
- [x] move backlog items into `docs/roadmaps/backlog/`
- [x] move compatibility inventory material into `docs/contracts/`
- [x] move report files into month-sharded `docs/logs/YYYY-MM/`
- [x] move the shared-admin migration prompt out of logs into guide-supporting material
- [x] rewrite Underlay doctrine and internal references to the new paths
- [x] sweep for stale `docs/roadmap/` and `docs/reports/` references

## Completion

`g01.029` is complete. Future Underlay work should open as `g01.030` unless the roadmap generation is deliberately rolled forward.
