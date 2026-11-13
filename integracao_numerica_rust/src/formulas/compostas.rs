
pub fn wrapper(fx_array: &Vec<f64>, h: &f64) -> Vec<f64>{
    let mut result_vec: Vec<f64> = Vec::new();

    result_vec.push(trap(&fx_array, &h));
    result_vec.push(one_third(&fx_array, &h));
    result_vec.push(three_eighths(&fx_array, &h));

    result_vec
}

pub fn trap(fx: &[f64], h: &f64) -> f64{
    let mut sum: f64 = 0.0;
    for i in 0..fx.len(){
        if i == 0 || i == fx.len() - 1{
            sum += fx[i];
        }
        else{
            sum += 2.0*fx[i];
        }
    }

    sum *= h/2.0;

    return sum.clone();
}

pub fn one_third(fx: &[f64], h: &f64) -> f64{
    let mut sum: f64 = 0.0;

    for i in 0..fx.len(){
        if i == 0 || i == fx.len() - 1{
            sum += fx[i];
            println!("one_third i:{i} x1");
        }
        else if (i + 1)%2 == 0{
            sum += 4.0*fx[i];
            println!("one_third i:{i} x4");
        }
        else{
            sum += 2.0*fx[i];
            println!("one_third i:{i} x2");
        }
    }

    sum *= h/3.0;

    return sum.clone();
}

pub fn three_eighths(fx: &[f64], h: &f64) -> f64{
    let mut sum: f64 = 0.0;

    for i in 0..fx.len(){
        if i == 0 || i == fx.len() - 1{
            sum += fx[i];
            println!("three_eighths i:{i} x1");
        }
        else if (i + 1)%4 == 0{
            sum += 2.0*fx[i];
            println!("three_eighths i:{i} x2");
        }
        else{
            sum += 3.0*fx[i];
            println!("three_eighths i:{i} x3");
        }
    }

    sum *= h*(3.0/8.0);

    return sum.clone();
}