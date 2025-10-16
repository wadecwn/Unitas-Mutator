use std::env;

//POWER
#[derive(Debug)]
enum PowerUnit {
    Watt,
    Kilowatt,
    Horsepower,
}

impl PowerUnit {
    fn to_watts(&self) -> f64 {
        match self {
            PowerUnit::Watt => 1.0,
            PowerUnit::Kilowatt => 1000.0,
            PowerUnit::Horsepower => 735.5,  // 1 hp ≈ 735.5 W
        }
    }
}

fn parse_power_unit(s: &str) -> Option<PowerUnit> {
    match s.to_lowercase().as_str() {
        "w" | "watt" | "watts" => Some(PowerUnit::Watt),
        "kw" | "kilowatt" | "kilowatts" => Some(PowerUnit::Kilowatt),
        "hp" | "horsepower" => Some(PowerUnit::Horsepower),
        _ => None,
    }
}

fn convert_power(value: f64, from: PowerUnit, to: PowerUnit) -> f64 {
    let watts = value * from.to_watts();
    watts / to.to_watts()
}

//MASS
#[derive(Debug)]
enum MassUnit {
    Gram,
    Kilogram,
    Grain,
    Dram,
    Ounce,
    Pound,
    Hundredweight,
    Ton,
    Tonne,
    Slug,
}

impl MassUnit {
    fn to_grams(&self) -> f64 {
        match self {
            MassUnit::Gram => 1.0,
            MassUnit::Kilogram => 1000.0,
            MassUnit::Grain => 0.06479891,
            MassUnit::Dram => 1.771845195,
            MassUnit::Ounce => 28.349523125,
            MassUnit::Pound => 453.59237,
            MassUnit::Hundredweight => 45359.237, 
            MassUnit::Ton => 907184.74,
            MassUnit::Tonne => 1_000_000.0,
            MassUnit::Slug => 14_593.903,
        }
    }
}

fn parse_mass_unit(s: &str) -> Option<MassUnit> {
    match s.to_lowercase().as_str() {
        "g" | "gram" | "grams" => Some(MassUnit::Gram),
        "kg" | "kilogram" | "kilograms" => Some(MassUnit::Kilogram),
        "gr" | "grain" | "grains" => Some(MassUnit::Grain),
        "dr" | "dram" | "drams" => Some(MassUnit::Dram),
        "oz" | "ounce" | "ounces" => Some(MassUnit::Ounce),
        "lb" | "pound" | "pounds" => Some(MassUnit::Pound),
        "cwt" | "hundredweight" => Some(MassUnit::Hundredweight),
        "ton" => Some(MassUnit::Ton),
        "t" | "tonne" | "metricton" => Some(MassUnit::Tonne),
        "slug" => Some(MassUnit::Slug),
        _ => None,
    }
}

fn convert_mass(value: f64, from: MassUnit, to: MassUnit) -> f64 {
    let grams = value * from.to_grams();
    grams / to.to_grams()
}


//TIME
#[derive(Debug)]
enum TimeUnit {
    Second,
    Minute,
    Hour, 
    Day, 
    Year,
}

impl TimeUnit {
    fn to_seconds(&self) -> f64 {
        match self {
            TimeUnit::Second => 1.0,
            TimeUnit::Minute => 60.0,
            TimeUnit::Hour => 3600.0,
            TimeUnit::Day => 86400.0,
            TimeUnit::Year => 31_556_736.0,
        }
    }
}

fn parse_time_unit(s: &str) -> Option<TimeUnit> {
    match s.to_lowercase().as_str() {
        "s" | "sec" | "second" | "seconds" => Some(TimeUnit::Second),
        "min" | "minute" | "minutes" => Some(TimeUnit::Minute),
        "h" | "hr" | "hour" | "hours" => Some(TimeUnit::Hour),
        "d" | "day" | "days" => Some(TimeUnit::Day),
        "a" | "yr" | "year" | "years" => Some(TimeUnit::Year),
        _ => None,
    }
}

fn convert_time(value: f64, from: TimeUnit, to: TimeUnit) -> f64 {
    let seconds = value * from.to_seconds();
    seconds / to.to_seconds()
}

//ANGLE
#[derive(Debug)]
enum AngleUnit {
    Radian,     
    Degree,      
    Steradian,   
}

impl AngleUnit {
    fn to_radians(&self) -> f64 {
        match self {
            AngleUnit::Radian => 1.0,
            AngleUnit::Degree => std::f64::consts::PI / 180.0,
            AngleUnit::Steradian => 1.0,
        }
    }
}

fn parse_angle_unit(s: &str) -> Option<AngleUnit> {
    match s.to_lowercase().as_str() {
        "rad" | "radian" | "radians" => Some(AngleUnit::Radian),
        "deg" | "°" | "degree" | "degrees" => Some(AngleUnit::Degree),
        "sr" | "steradian" | "steradians" => Some(AngleUnit::Steradian),
        _ => None,
    }
}

fn convert_angle(value: f64, from: AngleUnit, to: AngleUnit) -> f64 {
    let radians = value * from.to_radians();
    radians / to.to_radians()
}

//LENGTH
#[derive(Debug)]
enum LengthUnit {
    Millimeter,
    Centimeter,
    Decimeter,
    Meter,
    Kilometer,
    Inch,
    Foot,
    Yard,
    Rod,        
    Chain,       
    Furlong,     
    Mile,
    NauticalMile, 
    AstronomicalUnit, 
    LightYear,        
    Parsec,           
}

impl LengthUnit {
    fn to_meters_factor(&self) -> f64 {
        match self {
            LengthUnit::Millimeter => 0.001,
            LengthUnit::Centimeter => 0.01,
            LengthUnit::Decimeter => 0.1,
            LengthUnit::Meter => 1.0,
            LengthUnit::Kilometer => 1000.0,
            LengthUnit::Inch => 0.0254,
            LengthUnit::Foot => 0.3048,
            LengthUnit::Yard => 0.9144,
            LengthUnit::Rod => 5.0292,
            LengthUnit::Chain => 20.1168,
            LengthUnit::Furlong => 201.168,
            LengthUnit::Mile => 1609.344,
            LengthUnit::NauticalMile => 1852.0,
            LengthUnit::AstronomicalUnit => 149_597_870_700.0,
            LengthUnit::LightYear => 9.4607e15,
            LengthUnit::Parsec => 3.0857e16,
        }
    }
}

fn parse_length_unit(s: &str) -> Option<LengthUnit> {
    match s.to_lowercase().as_str() {
        "mm" | "millimeter" | "millimetre" => Some(LengthUnit::Millimeter),
        "cm" | "centimeter" | "centimetre" => Some(LengthUnit::Centimeter),
        "dm" | "decimeter" | "decimetre" => Some(LengthUnit::Decimeter),
        "m" | "meter" | "metre" => Some(LengthUnit::Meter),
        "km" | "kilometer" | "kilometre" => Some(LengthUnit::Kilometer),
        "in" | "inch" | "inches" => Some(LengthUnit::Inch),
        "ft" | "foot" | "feet" => Some(LengthUnit::Foot),
        "yd" | "yard" | "yards" => Some(LengthUnit::Yard),
        "rod" | "perch" | "pole" | "lug" => Some(LengthUnit::Rod),
        "ch" | "chain" => Some(LengthUnit::Chain),
        "fur" | "furlong" => Some(LengthUnit::Furlong),
        "mi" | "mile" | "miles" => Some(LengthUnit::Mile),
        "nm" | "nauticalmile" => Some(LengthUnit::NauticalMile),
        "au" | "astronomicalunit" => Some(LengthUnit::AstronomicalUnit),
        "ly" | "lightyear" => Some(LengthUnit::LightYear),
        "pc" | "parsec" => Some(LengthUnit::Parsec),
        _ => None,
    }
}

fn convert_length(value: f64, from: LengthUnit, to: LengthUnit) -> f64 {
    let meters = value * from.to_meters_factor();
    meters / to.to_meters_factor()
}

//VOLUME
#[derive(Debug)]
enum VolumeUnit {
    CubicMeter,   
    Liter,           
    Milliliter,      
    Centiliter,     
    Deciliter,      
    Hectoliter,     
    Teaspoon,       
    Tablespoon,      
    FluidOunce,    
    Cup,             
    Gill,          
    Pint,         
    Quart,          
    Gallon,         
    AcreFoot,        
    CubicInch,       
    CubicFoot,      
    CubicYard,       
}

impl VolumeUnit {
    fn to_milliliters(&self) -> f64 {
        match self {
            VolumeUnit::CubicMeter => 1_000_000.0,     
            VolumeUnit::Liter => 1_000.0,               
            VolumeUnit::Milliliter => 1.0,
            VolumeUnit::Centiliter => 10.0,              
            VolumeUnit::Deciliter => 100.0,             
            VolumeUnit::Hectoliter => 100_000.0,         
            VolumeUnit::Teaspoon => 4.92892,           
            VolumeUnit::Tablespoon => 14.7868,         
            VolumeUnit::FluidOunce => 29.5735,         
            VolumeUnit::Cup => 236.588,                 
            VolumeUnit::Gill => 118.294,                
            VolumeUnit::Pint => 473.176,                 
            VolumeUnit::Quart => 946.353,               
            VolumeUnit::Gallon => 3785.41,               
            VolumeUnit::AcreFoot => 1_233_480_000.0,    
            VolumeUnit::CubicInch => 16.3871,            
            VolumeUnit::CubicFoot => 28_316.8,           
            VolumeUnit::CubicYard => 764_555.0,          
        }
    }
}

fn parse_volume_unit(s: &str) -> Option<VolumeUnit> {
    match s.to_lowercase().as_str() {
        "m3" | "cubicmeter" | "cubicmeters" => Some(VolumeUnit::CubicMeter),
        "l" | "liter" | "litre" => Some(VolumeUnit::Liter),
        "ml" | "milliliter" | "millilitre" => Some(VolumeUnit::Milliliter),
        "cl" | "centiliter" => Some(VolumeUnit::Centiliter),
        "dl" | "deciliter" => Some(VolumeUnit::Deciliter),
        "hl" | "hectoliter" => Some(VolumeUnit::Hectoliter),
        "tsp" | "teaspoon" => Some(VolumeUnit::Teaspoon),
        "tbsp" | "tablespoon" => Some(VolumeUnit::Tablespoon),
        "floz" | "fl oz" => Some(VolumeUnit::FluidOunce),
        "cup" => Some(VolumeUnit::Cup),
        "gill" => Some(VolumeUnit::Gill),
        "pt" | "pint" => Some(VolumeUnit::Pint),
        "qt" | "quart" => Some(VolumeUnit::Quart),
        "gal" | "gallon" => Some(VolumeUnit::Gallon),
        "acreft" | "acre-ft" => Some(VolumeUnit::AcreFoot),
        "in3" | "cuin" | "cubicinch" => Some(VolumeUnit::CubicInch),
        "ft3" | "cuft" | "cubicfoot" => Some(VolumeUnit::CubicFoot),
        "yd3" | "cuyd" | "cubicyard" => Some(VolumeUnit::CubicYard),
        _ => None,
    }
}

fn convert_volume(value: f64, from: VolumeUnit, to: VolumeUnit) -> f64 {
    let value_ml = value * from.to_milliliters();
    value_ml / to.to_milliliters()
}

//AREA
#[derive(Debug)]
enum AreaUnit {
    SquareMeter,
    Hectare,
    Acre,
    SquareInch,
    SquareFoot,
    SquareYard,
    SquareMile,
}

impl AreaUnit {
    fn to_square_meters_factor(&self) -> f64 {
        match self {
            AreaUnit::SquareMeter => 1.0,
            AreaUnit::Hectare => 10_000.0,
            AreaUnit::Acre => 4_046.8564224,
            AreaUnit::SquareInch => 0.00064516,
            AreaUnit::SquareFoot => 0.09290304,
            AreaUnit::SquareYard => 0.83612736,
            AreaUnit::SquareMile => 2_589_988.110336,
        }
    }
}

fn parse_area_unit(s: &str) -> Option<AreaUnit> {
    match s.to_lowercase().as_str() {
        "m2" | "sqm" | "squaremeter" | "squaremeters" => Some(AreaUnit::SquareMeter),
        "ha" | "hectare" | "hectares" => Some(AreaUnit::Hectare),
        "acre" | "acres" => Some(AreaUnit::Acre),
        "in2" | "sqin" | "squareinch" | "squareinches" => Some(AreaUnit::SquareInch),
        "ft2" | "sqft" | "squarefoot" | "squarefeet" => Some(AreaUnit::SquareFoot),
        "yd2" | "sqyd" | "squareyard" | "squareyards" => Some(AreaUnit::SquareYard),
        "mi2" | "sqmi" | "squaremile" | "squaremiles" => Some(AreaUnit::SquareMile),
        _ => None,
    }
}

fn convert_area(value: f64, from: AreaUnit, to: AreaUnit) -> f64 {
    let base = value * from.to_square_meters_factor();
    base / to.to_square_meters_factor()
}

//TEMPERATURE
#[derive(Debug)]
enum TempUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

fn parse_temp_unit(s: &str) -> Option<TempUnit> {
    match s.to_lowercase().as_str() {
        "c" | "celsius" => Some(TempUnit::Celsius),
        "f" | "fahrenheit" => Some(TempUnit::Fahrenheit),
        "k" | "kelvin" => Some(TempUnit::Kelvin),
        _ => None,
    }
}

fn convert_temperature(value: f64, from: TempUnit, to: TempUnit) -> f64 {
    // convert to Celsius first
    let celsius = match from {
        TempUnit::Celsius => value,
        TempUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
        TempUnit::Kelvin => value - 273.15,
    };

    // then convert to target unit
    match to {
        TempUnit::Celsius => celsius,
        TempUnit::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
        TempUnit::Kelvin => celsius + 273.15,
    }
}

//Wrong Conversion
fn print_usage_and_exit(prog: &str) {
    eprintln!("Usage: {} <value> <from_unit> <to_unit>", prog);
    eprintln!("Examples:");
    eprintln!("  {} 1 mi km        # length", prog);
    eprintln!("  {} 64 F C         # temperature", prog);
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        print_usage_and_exit(&args[0]);
    }

    let value: f64 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Could not parse '{}' as a number", args[1]);
        print_usage_and_exit(&args[0]);
        0.0
    });

    // Parse each unit type
    let from_len = parse_length_unit(&args[2]);
    let to_len = parse_length_unit(&args[3]);

    let from_temp = parse_temp_unit(&args[2]);
    let to_temp = parse_temp_unit(&args[3]);

    let from_area = parse_area_unit(&args[2]);
    let to_area = parse_area_unit(&args[3]);

    let from_vol = parse_volume_unit(&args[2]);
    let to_vol = parse_volume_unit(&args[3]);

    let from_angle = parse_angle_unit(&args[2]);
    let to_angle = parse_angle_unit(&args[3]);

    let from_time = parse_time_unit(&args[2]);
    let to_time = parse_time_unit(&args[3]);

    let from_mass = parse_mass_unit(&args[2]);
    let to_mass = parse_mass_unit(&args[3]);

    let from_power = parse_power_unit(&args[2]);
    let to_power = parse_power_unit(&args[3]);

    // Conversion logic
    let result = if let (Some(f), Some(t)) = (from_len, to_len) {
        convert_length(value, f, t)
    } else if let (Some(f), Some(t)) = (from_temp, to_temp) {
        convert_temperature(value, f, t)
    } else if let (Some(f), Some(t)) = (from_area, to_area) {
        convert_area(value, f, t)
    } else if let (Some(f), Some(t)) = (from_vol, to_vol) {
        convert_volume(value, f, t)
    } else if let (Some(f), Some(t)) = (from_angle, to_angle) {
        convert_angle(value, f, t)
    } else if let (Some(f), Some(t)) = (from_time, to_time) { 
        convert_time(value, f, t)
    } else if let (Some(f), Some(t)) = (from_mass, to_mass) {
    convert_mass(value, f, t)
    } else if let (Some(f), Some(t)) = (from_power, to_power) {
    convert_power(value, f, t)
    } else {
        eprintln!("Invalid or incompatible units.");
        print_usage_and_exit(&args[0]);
        unreachable!()
    };

    println!("{} {} = {} {}", value, args[2], result, args[3]);
}
