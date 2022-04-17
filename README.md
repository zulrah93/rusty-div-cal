# Basic Dividend Calculator Written in Rust With Support For ADA Staking
[![License: CC0-1.0](https://licensebuttons.net/l/zero/1.0/80x15.png)](http://creativecommons.org/publicdomain/zero/1.0/)
:cuba:

Self-explanatory as what the tool does

## How to build and run

```
cargo run --release
```

## How to use cli tool

Create a json call it profolio.json and use this as your template:

``` json
{
    "tickers" : [ {"ticker" : "VOO", "shares" : 197.24, "apply_drip" : true}, {"ticker" : "VYM", "shares" : 285.16, "apply_drip" : true},
    {"ticker" : "VYMI", "shares" : 395.628, "apply_drip" : true}, {"ticker" : "WPC", "shares" : 874.482, "apply_drip" : true }  ],
    "dividend_projection_in_years" : 35,
    "yearly_growth_percentage" : 8,
    "div_yearly_growth_percentage" : 3
}
```

### Legal Disclamer
Any informations shown here is not a form of stock advice. If you invest in stocks you take all risks that includes any bugs or possible wrong perdications of the future.
This tool is meant to estimate potential dividend gains and isn't in a way a form of stock advice or whether the investements calculated are good advice. Please consult
an actual certified financial advisor. Feel free to fork this project to add your own features. Software is shown AS IS.
