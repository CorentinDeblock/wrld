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
- Added a new macro "BufferData" (see [doc](https://docs.rs/wrld/0.2.2/wrld/derive.BufferData.html) for more details)
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