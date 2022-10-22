# Desktopd

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/mothsART/desktopd/actions/workflows/ci.yml/badge.svg)](https://github.com/mothsART/desktopd/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/mothsART/desktopd/badge.svg?branch=master)](https://coveralls.io/github/mothsART/desktopd?branch=master)
[![Crates.io Version](https://img.shields.io/crates/v/desktopd.svg)](https://crates.io/crates/desktopd)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.62.0+-lightgray.svg)](#rust-version-requirements)

## Introduction

A linux daemon who store .desktop meta-datas from $XDG_DATA_DIRS on database.

## Goal

Give the best API to find applications store in $XDG_DATA_DIRS

## Dev

```sh
cargo install diesel_cli --no-default-features --features sqlite
```
