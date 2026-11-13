
pub fn first_degree(x_array: &Vec<f64>) -> Vec<f64>{
    // f(x) = 1 + 2x
    let mut fx_array = vec![0.0; x_array.len()];

    for i in 0..x_array.len(){
        fx_array[i] = 1.0 + 2.0*x_array[i];
    }

    fx_array
    
}

pub fn second_degree(x_array: &Vec<f64>) -> Vec<f64>{
    // f(x) = 1 + 2x + 3x^2
    let mut fx_array = vec![0.0; x_array.len()];

    for i in 0..x_array.len(){
        fx_array[i] = 1.0 + 2.0*x_array[i] + 3.0*x_array[i].powi(2);
    }

    fx_array
}

pub fn third_degree(x_array: &Vec<f64>) -> Vec<f64>{
    // f(x) = 1 + 2x + 3x^2 - 4x^3
    let mut fx_array = vec![0.0; x_array.len()];

    for i in 0..x_array.len(){
        fx_array[i] = 1.0 + 2.0*x_array[i] + 3.0*x_array[i].powi(2) - 4.0*x_array[i].powi(3);
    }

    fx_array
}

pub fn fourth_degree(x_array: &Vec<f64>) -> Vec<f64>{
    // f(x) = 1 + 2x + 3x^2 - 4x^3 + 5x^4
    let mut fx_array = vec![0.0; x_array.len()];

    for i in 0..x_array.len(){
        fx_array[i] = 1.0 + 2.0*x_array[i] + 3.0*x_array[i].powi(2) - 4.0*x_array[i].powi(3) + 5.0*x_array[i].powi(4);
    }

    fx_array
}

pub fn trg(x_array: &Vec<f64>) -> Vec<f64>{
    // f(x) = 1 + 2x + 3x^2 - 4x^3 + 5cos(x)^4
    let mut fx_array = vec![0.0; x_array.len()];

    for i in 0..x_array.len(){
        fx_array[i] = 1.0 + 2.0*x_array[i] + 3.0*x_array[i].powi(2) - 4.0*x_array[i].powi(3) + 5.0*x_array[i].cos().powi(4);
    }

    fx_array
}

pub fn find_x_array(from: f64, to: f64, n: usize) -> (Vec<f64>, f64){
    let elements = n + 1;
    let mut x_array = vec![0.0; elements];
    let delta: f64 = to - from;
    let h: f64 = delta/(n as f64);

    for i in 0..elements{
        x_array[i] = from + (h*i as f64);
    }

    return (x_array, h);
}

pub fn wrapper(from: f64, to: f64, n: usize) -> Vec<Vec<f64>>{
    let x_array = find_x_array(from, to, n);
    let mut matrix: Vec<Vec<f64>> = Vec::new();

    matrix.push(x_array.0);
    matrix.push(first_degree(&matrix[0]));
    matrix.push(second_degree(&matrix[0]));
    matrix.push(third_degree(&matrix[0]));
    matrix.push(fourth_degree(&matrix[0]));
    matrix.push(trg(&matrix[0]));

    matrix
}