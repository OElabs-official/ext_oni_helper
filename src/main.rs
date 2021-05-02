mod food;
mod planet;
fn main() {
    Vol_to_steam(1.18)
}

fn Oxyfern_vs_Electrolyzer()
{
    //电解器 vs 氧齿蕨
    //Oxyfern: 0-40 C; max 10000grams; require 0.625g CO2 >>>31.3g O2 per Sec  , 19Kg Water /cycle , 4Kg dirty,
    // Electrolyzer:75 C output; 1000g water/s >>> 888g O2, 112g H2 per Sec,
    let of_water_sec = 19_000.0/600.0;
    let of_dirty_sec = 4_000.0/600.0;
    println! ("oxf require water {} per sec \n dirty {} per sec.",of_water_sec,of_dirty_sec);
    let ox_ratio = 888.0/31.3;
    let of_water_vs = ox_ratio * of_water_sec;
    let of_dirty_vs = ox_ratio * of_dirty_sec;
    println!("output as Electrolyzer: oxf require water {}\n dirty {}",of_water_vs,of_dirty_vs);
    
}
fn Vol_to_steam(a:f64)
{
    let magma_avg=a;
    const magma_shc:f64     = 1.0;
    const magma_temp:f64    = 1726.85;
    const target_temp:f64   = 130.0;
    const water_shc:f64     = 4.179;
    const water_temp:f64    = 95.0;
    let x0  = (magma_temp-target_temp)*magma_shc;//单位岩浆降温释放的能量
    let x1  = (target_temp-water_temp)*water_shc;//单位水升温吸收的能量
    let ratio = x0/x1;
    let water_input = a*ratio;
    println! ("Ratio:{}\n{}",ratio,water_input)


}