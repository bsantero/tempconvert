# TempConvert

TempConvert is my first foray into [Rust](https://www.rust-lang.org/), a super-fast, precompiled programming language.

## Usage

`tempconvert %F` where %F is the temperature (integer or floating!) that you want to know in Celcius.

i.e.  
`tempconvert 16` will produce:  
`-8.88888888888889°C = 16°F`

`tempconvert 98.2` will produce:  
`36.77777777777778°C = 98.2°F`


Lacking parts are:
- error-checking (very-important)
- Celcius to Farenheit conversion (pretty important)
- negative number input (who cares about the cold, anyways?) (probably still important)
