pub fn nb_year(population: i32, percent: f64, aug: i32, target_population: i32) -> i32 {
    if population >= target_population {
        0
    } else {
        let new_population = population + (population as f64 * percent / 100f64) as i32 + aug;
        1 + nb_year(new_population, percent, aug, target_population)
    }
}
