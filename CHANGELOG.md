# Changelog

[unreleased]: https://github.com/s8sato/eth-offline-signer/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/s8sato/eth-offline-signer/releases/tag/v1.0.0

All notable changes to this project will be documented in this file.

## [unreleased]

## [1.0.0] - 2025-06-09

### 🚀 Features

- Implement commands to match README documentation

### 🐛 Bug Fixes

- Wrap transaction in EIP-2718 envelope for EIP-1559 compatibility

### 🚜 Refactor

- Preserve transaction type during encode and decode

### 📚 Documentation

- Add initial README draft
- *(readme)* Refine wording and layout

### 🧪 Testing

- Add Anvil-based smoke test for sign-submit-confirm flow
- *(integration)* Add CLI sign-submit-confirm test
- *(integration)* Add manual dispatch workflow for Sepolia testnet sends

### ⚙️ Miscellaneous Tasks

- Initial commit
- Add initial CHANGELOG.md skeleton
- Add initial GitHub Actions workflows
- Generate CLI help and check consistency
- *(plan)* Remove property-based tests from project roadmap

<!-- generated by git-cliff -->
