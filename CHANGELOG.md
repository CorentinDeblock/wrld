## 1.0.0 - 2022-09-08
## Added
- DescInstance macro
- Matrix attribute
- Parser for matrix attribute
## 0.3.0 - 2022-08-30 (Merge of 0.21.31 and 0.21.3 due to semver violation)
## Added
- Copy derive macro on TypeToWGPU type (backend)
- Look 0.21.31 and 0.21.3
## Fix
- Renamed function derive implemention from wrsl to wrld
- Look 0.21.31 and 0.21.3
## 0.21.31 - 2022-08-29 (Yanked due to semver violation)
### Added
- Added more info on macro name definition for mutate_#structname and #structname_const_into
### Fix
- Fixed #[warn(unused_macros)] on mutate_#structname and #structname_const_into
- Fixed wrld::Desc macro example in doc
- Fixed exmaple on readme to add #[repr(C)] or #[repr(transparent)] macro
- Fixed error handing for attribute parameter of wrld::Desc macro
## 0.21.3 - 2022-08-29 (Yanked due to semver violation)
### Add
- Added macro conversion
### Fix
- Fixed typo on BufferData macro
- Fixed old example on Desc macro
- Fixed old example on readme
## 0.2.2 - 2022-08-28
### Add
- Added parse_args function helper for the backend
### Change
- wrld::Desc now require #[repr(C)] or #[repr(transparent)] attribute. This is to make sure to have a fixed layout and avoid safety problem
(more info [here](https://github.com/CorentinDeblock/wrld/issues/1))
### Fix
- Fixed typo BufferData example in doc
## 0.2.1 - 2022-08-26
### Fix
- Fixed "constant functions cannot evaluate destructors" error
## 0.2.0 - 2022-08-26
### Add
- Added a new macro "BufferData" (see [doc](https://docs.rs/wrld/0.21.3/wrld/derive.BufferData.html) for more details)
### Fix
- Fixed array_stride data to take only the size of provided attributes

## 0.1.2 - 2022-08-20
### Add
- Added a changelog.md
### Fix
- Fixed errornous shader location
## 0.1.1 - 2022-08-18
### Fix
- Removed debug printing information
- Removed useless file

# Summary (all patch since 0.1.0)
## Add
- Added DescInstance macro
- Added matrix attribute
- Added parser for matrix attribute
- Added more info on macro name definition for mutate_#structname and #structname_const_into
- Added macro conversion
- Added parse_args function helper for the backend
- Added a new macro "BufferData" (see [doc](https://docs.rs/wrld/0.21.3/wrld/derive.BufferData.html) for more details)
- Added a changelog.md
## Fix
- Fixed #[warn(unused_macros)] on mutate_#structname and #structname_const_into
- Fixed wrld::Desc macro example in doc
- Fixed exmaple on readme to add #[repr(C)] or #[repr(transparent)] macro
- Fixed error handing for attribute parameter of wrld::Desc macro
- Fixed typo on BufferData macro
- Fixed old example on Desc macro
- Fixed old example on readme
- Fixed "constant functions cannot evaluate destructors" error
- Fixed array_stride data to take only the size of provided attributes
- Fixed errornous shader location
- Removed debug printing information
- Removed useless file
## Change
- wrld::Desc now require #[repr(C)] or #[repr(transparent)] attribute. This is to make sure to have a fixed layout and avoid safety problem