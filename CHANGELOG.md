# Changelog

## [1.2.1] - 2024.10.08


### Added

- `checked` methods for safer arithmetic operations.
- Bounds checking to ensure values stay within expected ranges.

### Changed

- Approximation of the lower bound for negative numbers.


## [1.1.1] - 2024.09.01

### Fixed

- Percentage error calculation now handles large negative numbers accurately

## [1.0.1] - 2024.08.31

### Added

- Clarified in the documentation that the approximate number is generally smaller but can sometimes be larger, particularly for certain negative values.
- Explanation of how input bit length affects the likelihood of value duplication, especially when reducing large inputs (e.g., 128 bits) to 24 bits.

### Changed

- Refactored code for consistency with the updated documentation.

## [1.0.0] - 2024.08.31

- Initial release
