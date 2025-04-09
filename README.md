# traffic-flow-analysis
Simple API web server to provide data about traffic flow

## how to run
1. Copy 'sample-Rocket.toml' to 'Rocket.toml'
2. Edit `url` in `Rocket.toml` file to be your __mysql__ database url
3. compile and run the server with 
  ```sh
    cargo run
  ```
  
## MYSQL database Table (required, planned to remove)
1. weatherData

| # | Name        | Type         | Collation            | Attributes | Null | Default           | Comments | Extra             |
|---|-------------|--------------|----------------------|------------|------|-------------------|----------|-------------------|
| 1 | id          | int          |                      |            | No   | None              |          | AUTO_INCREMENT    |
| 2 | time        | datetime     |                      |            | No   | CURRENT_TIMESTAMP |          | DEFAULT_GENERATED |
| 3 | temp        | float        |                      |            | Yes  | NULL              |          |                   |
| 4 | weather     | varchar(255) | utf8mb3_general_ci   |            | Yes  | NULL              |          |                   |
| 5 | weatherDesc | varchar(255) | utf8mb3_general_ci   |            | Yes  | NULL              |          |                   |
| 6 | windSpeed   | float        |                      |            | Yes  | NULL              |          |                   |
| 7 | windHeading | int          |                      |            | Yes  | NULL              |          |                   |
| 8 | rainmmh     | float        |                      |            | Yes  | NULL              |          |                   |
| 9 | humidity    | tinyint      |                      |            | Yes  | NULL              |          |                   |
